use std::{path::PathBuf, fs::{File, OpenOptions, self}, io::{BufReader, BufRead, Write, Seek, self, stdout}, collections::HashMap, time::Duration, thread::sleep, option};
use chrono::{Local, DateTime};
use log::{info, trace};
use serde::{Serialize, Deserialize};
use walkdir::WalkDir;
use colored::Colorize;
use std::thread;
use fs_extra::copy_items_with_progress;

use crossterm::{QueueableCommand, cursor, terminal, ExecutableCommand};

use crate::libs::helpers::{system_time_to_u64, print_error, get_relative_path, file_copy_process_handler, get_calling_path, map_path_to_target};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PathTransactionData {
    path : String,
    last_modified : u64,
    last_accessed : u64,
    created : u64,
    size : String,
    is_dir : bool,
}


#[derive(Debug, Clone)]
pub struct Transaction {
    base: String,
    target: String,
    base_data : Vec<PathTransactionData>,
    target_data : Vec<PathTransactionData>,
    last_updated : [DateTime<Local>; 2], // [base, target]
}

// basic 
impl Transaction {
    pub fn new(base: String, target: String) -> Transaction {
        let mut t = Transaction {
            base,
            target,
            base_data : Vec::new(),
            target_data : Vec::new(),
            last_updated : [Local::now(), Local::now()],
        };
        t.prepare();
        return t;

    }

    pub fn get_base_data(&mut self) -> &Vec<PathTransactionData> {
        let time_diff = Local::now().signed_duration_since(self.last_updated[0]);
        if time_diff.num_seconds() > 5 {
            self.load_base_data();
        }
        return &self.base_data;
    }

    pub fn get_target_data(&mut self) -> &Vec<PathTransactionData> {
        let time_diff = Local::now().signed_duration_since(self.last_updated[1]);
        if time_diff.num_seconds() > 5 {
            self.load_target_data();
        }
        return &self.target_data;
    }


    pub fn prepare(&mut self) -> &mut Self {
        self.save_base_lock_data();
        self.load_base_data();
        self.load_target_data();
        return self;
    }
    


   pub fn load_base_data(&mut self) {
        self.base_data = Transaction::get_lock_data(&PathBuf::from(self.base.clone()));
    }

    pub fn load_target_data(&mut self) {
        self.target_data = Transaction::get_lock_data(&PathBuf::from(self.target.clone()));
    }

    pub fn save_base_data(&self) {
    }

}


//path handling
impl Transaction {
        pub fn resolve_lock(path: &PathBuf) -> File {
        // Construct the lock file path by appending ".hard-sync/hard-sync.lock" to the directory
        let lock_path = path.join(".hard-sync/hard-sync.lock");
        // Create the parent directories if they don't exist
        if let Some(parent) = lock_path.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }
        
        let mut file : File;
        // Create the lock file if it doesn't exist
        if !lock_path.exists() {
            file = File::create(&lock_path).unwrap();
        }
        else {
            file = OpenOptions::new().read(true).write(true).open(&lock_path).unwrap();
        }
        return file;
        
    }

    pub fn get_hard_sync_path(path: &PathBuf) -> PathBuf {
        let mut _path = path.clone();
        _path.push(".hard-sync");
        return _path;
    }

    pub fn get_base_hard_sync_path(&self) -> PathBuf {
        return Transaction::get_hard_sync_path(&PathBuf::from(self.base.clone()));
    }

    pub fn get_target_hard_sync_path(&self) -> PathBuf {
        return Transaction::get_hard_sync_path(&PathBuf::from(self.target.clone()));
    }

    pub fn get_base_lock(&self) -> File {
        let hard_sync_path = self.get_base_hard_sync_path();
        return Transaction::resolve_lock(&hard_sync_path);
    }

    pub fn get_target_lock(&self) -> File {
        let hard_sync_path = self.get_target_hard_sync_path();
        return Transaction::resolve_lock(&hard_sync_path);
    }
}


// data handling getting lock data
impl Transaction {
    fn get_lock_data(path: &PathBuf) -> Vec<PathTransactionData> {
        let mut data: Vec<PathTransactionData> = Vec::new();
        let lock_file = Transaction::resolve_lock(path);
        // println!("lock_file: {:?}", lock_file);
        let reader = BufReader::new(lock_file);
        for line in reader.lines() {
            trace!("trad data: {:?}", line);
            let line = line.unwrap();
            // println!("line: {}", line);
            match serde_json::from_str::<PathTransactionData>(&line) {
                Ok(d) => {
                    data.push(d.clone());
                    
                }
                Err(err) => {
                    print_error(format!("Error deserializing data: {}", err).as_str(), false);
                },

            }
        }
        return data;
    }

    fn save_lock_data(path : &PathBuf, data : Vec<PathTransactionData>) {
        // write to lock file
        let mut lock_file = Transaction::resolve_lock(path);
        // clean or empty the file
        lock_file.set_len(0).unwrap();
        for d in data {
            let json = serde_json::to_string(&d).unwrap();
            lock_file.write_all(json.as_bytes()).unwrap();
            lock_file.write_all("\n".as_bytes()).unwrap();
        }
    }

    pub fn save_base_lock_data(&self) {
        let mut lock_data = self.get_base_save_data();
        Transaction::save_lock_data(&PathBuf::from(self.base.clone()), lock_data);
    }

    pub fn save_target_lock_data(&self) {
        let mut lock_data = self.get_target_save_data();
        Transaction::save_lock_data(&PathBuf::from(self.target.clone()), lock_data);
    }

    fn get_save_data_path_transaction_data(path : &PathBuf) -> Vec<PathTransactionData> {
        let mut data: Vec<PathTransactionData> = Vec::new();
        let mut stdout = stdout();
        stdout.execute(cursor::Hide).unwrap();
        for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
            stdout.queue(cursor::SavePosition).unwrap();
            println!("Generating Lock Data for: {:?}", path);
            println!("\rentry: {}", entry.path().to_str().unwrap().green());
            stdout.queue(cursor::RestorePosition).unwrap();
            stdout.flush().unwrap();
            thread::sleep(Duration::from_millis(100));
            let file_path = entry.path();
            // if the part is a hard-sync directory or is base directory, skip it
            if file_path.ends_with(".hard-sync") || entry.path() == path || file_path.ends_with("hard-sync.lock") {
                continue;
            }
            let metadata = fs::metadata(file_path).unwrap();
            let last_modified = metadata.modified().unwrap();
            let last_accessed = metadata.accessed().unwrap();
            let created = metadata.created().unwrap();
            let size = metadata.len();
            let is_dir = metadata.is_dir();
            let path = get_relative_path(path.to_str().unwrap(), file_path.to_str().unwrap()).unwrap();
            stdout.queue(cursor::SavePosition).unwrap();
            // stdout.write_all(format!("\rFound path: {}", path.as_str().green()).as_bytes()).unwrap();
            println!("\rFound path: {}", path.as_str().green());
            stdout.queue(cursor::RestorePosition).unwrap();
            stdout.flush().unwrap();
            thread::sleep(Duration::from_millis(100));
            data.push(PathTransactionData {
                path,
                last_modified : system_time_to_u64(last_modified),
                last_accessed : system_time_to_u64(last_accessed),
                created : system_time_to_u64(created),
                size : size.to_string(),
                is_dir,
            });
            stdout.queue(cursor::RestorePosition).unwrap();
            stdout.queue(terminal::Clear(terminal::ClearType::FromCursorDown)).unwrap();
        }
        stdout.queue(cursor::RestorePosition).unwrap();
        stdout.flush().unwrap();
        let total_files = data.len();
        // stdout.write_all(format!("\n Generated Lock data for path: {:?} found {} files", path, total_files.to_string().blue()).as_bytes()).unwrap();
        println!("\n Generated Lock data for path: {:?} found {} files", path, total_files.to_string().blue());
        stdout.execute(cursor::Show).unwrap();
        // for testing purposes, print all file found
        // print all file found
        println!("Files found for path: {:?}", path.to_str().unwrap());
        for d in &data {
            println!("{} {}", d.path, d.size);
        }
        return data;
    }


    pub fn get_base_save_data(&self) -> Vec<PathTransactionData> {
        return Transaction::get_save_data_path_transaction_data(&PathBuf::from(self.base.clone()));
    }

    pub fn get_target_save_data(&self) -> Vec<PathTransactionData> {
        return Transaction::get_save_data_path_transaction_data(&PathBuf::from(self.target.clone()));
    }
    
}

// syncronization
impl Transaction {
    pub fn sync(&mut self) {
        // get base and target data
        let mut files_to_copy : Vec<String> = Vec::new();
        let mut binding = self.clone();
        let base_binding: &Vec<PathTransactionData> = binding.get_base_data();
        let mut binding = self.clone();
        let target_binding: &Vec<PathTransactionData> = binding.get_target_data();
        let base_data: HashMap<String, PathTransactionData> = Transaction::path_transaction_vec_to_hash_map(base_binding);
        let target_data: HashMap<String, PathTransactionData> = Transaction::path_transaction_vec_to_hash_map(target_binding);
        let mut total_new_files = 0;
        let mut stdout = stdout();
        stdout.execute(cursor::Hide).unwrap();
        println!("Checking for Updated/Untracked Files");
        let calling_path  = get_calling_path();
        for (d, data) in base_data {
            stdout.queue(cursor::SavePosition).unwrap();
            println!("\rChecking if {} exists in target data", d);
            stdout.queue(cursor::RestorePosition).unwrap();
            stdout.flush().unwrap();
            thread::sleep(Duration::from_millis(100));
            let mut copying = match target_data.get(&d) {
                Some(dat_exist) => {
                    // if the target data is a directory, skip it
                    if dat_exist.is_dir {
                        continue;
                    }
                    // if the target data is older than the base data, copy it to the target
                    dat_exist.last_modified < data.last_modified ||
                    (dat_exist.last_modified == data.last_modified && dat_exist.last_accessed < data.last_accessed) ||
                    (dat_exist.last_modified == data.last_modified && dat_exist.last_accessed == data.last_accessed && dat_exist.created < data.created)
                }
                None => true
                
            };

            if copying {
                stdout.queue(cursor::SavePosition).unwrap();
                let full_path = PathBuf::from(self.base.clone()).join(&data.path);
                // remove calling path from full path if it exists
                let full_path = match get_relative_path(calling_path.as_str(), full_path.to_str().unwrap()) {
                    Some(x) => x,
                    None => full_path.to_str().unwrap().to_string()
                };
                files_to_copy.push(full_path.clone());
                total_new_files += 1;
                println!("\r Found: {} with full path: {}", d, full_path.as_str().green());
                stdout.queue(cursor::RestorePosition).unwrap();
                stdout.flush().unwrap();
                thread::sleep(Duration::from_millis(100));
            }
            stdout.queue(cursor::RestorePosition).unwrap();
            stdout.queue(terminal::Clear(terminal::ClearType::FromCursorDown)).unwrap();
        }
        stdout.execute(cursor::Show).unwrap();
        info!("Found {} new files available for copying", total_new_files);
        let options = fs_extra::dir::CopyOptions { 
            overwrite : true,
            skip_exist : false,
            copy_inside : false,
            // content_only : true,
            ..Default::default()
        };
        let mapped_file_to_tar = map_path_to_target(files_to_copy, self.target.clone(), self.base.clone());
        println!("mapped_file_to_tar: {:?}", mapped_file_to_tar);
        for (files_to_copy, tar) in mapped_file_to_tar {
            match copy_items_with_progress(&files_to_copy, &tar, &options, file_copy_process_handler) {
                Ok(_) => {
                    println!("Successfully copied {} files", files_to_copy.len());
                    // for testing purposes, print all file found
                    for d in &files_to_copy {
                        println!("{} {} to {}", d, "copied".green(), tar.clone().blue());
                    }
                    
                }
                Err(err) => {
                    print_error(format!("\nError copying {:?} to target: {}", files_to_copy, err).as_str(), false);
                }
            }
        }
        self.save_target_lock_data();
        self.save_base_lock_data();
    }
    

    pub fn path_transaction_vec_to_hash_map(data: &Vec<PathTransactionData>) -> HashMap<String, PathTransactionData> {
        let mut map = HashMap::new();
        for d in data {
            map.insert(d.path.clone(), d.clone());
        }
        return map;
    }
}
