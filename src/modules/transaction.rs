use std::{path::PathBuf, fs::{File, OpenOptions, self}, io::{BufReader, BufRead, Write, Seek, self, stdout}, collections::HashMap, time::Duration, thread::sleep};
use chrono::{Local, DateTime};
use log::{info, trace};
use serde::{Serialize, Deserialize};
use walkdir::WalkDir;
use colored::Colorize;
use std::thread;

use crossterm::{QueueableCommand, cursor, terminal, ExecutableCommand};

use crate::libs::helpers::{system_time_to_string, print_error};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PathTransactionData {
    path : String,
    last_modified : String,
    last_accessed : String,
    created : String,
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
            // stdout.write_all(format!("\rentry: {}", entry.path().to_str().unwrap().green()).as_bytes()).unwrap();
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
            let path = file_path.to_str().unwrap().to_string();
            stdout.queue(cursor::SavePosition).unwrap();
            // stdout.write_all(format!("\rFound path: {}", path.as_str().green()).as_bytes()).unwrap();
            println!("\rFound path: {}", path.as_str().green());
            stdout.queue(cursor::RestorePosition).unwrap();
            stdout.flush().unwrap();
            thread::sleep(Duration::from_millis(100));
            data.push(PathTransactionData {
                path,
                last_modified : system_time_to_string(last_modified),
                last_accessed : system_time_to_string(last_accessed),
                created : system_time_to_string(created),
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
        let mut binding = self.clone();
        let base_binding: &Vec<PathTransactionData> = binding.get_base_data();
        let mut binding = self.clone();
        let target_binding: &Vec<PathTransactionData> = binding.get_target_data();
        let mut base_data: HashMap<String, PathTransactionData> = Transaction::path_transaction_vec_to_hash_map(base_binding);
        let mut target_data: HashMap<String, PathTransactionData> = Transaction::path_transaction_vec_to_hash_map(target_binding);
        let mut success_count = 0;
        let mut total_new_files = 0;
        let mut stdout = stdout();
        stdout.execute(cursor::Hide).unwrap();
        // illerate through base data and check if it exists in target data with an older modified date
        for (d, data) in base_data {
            stdout.queue(cursor::SavePosition).unwrap();
            // stdout.write_all(format!("\rChecking if {} exists in target data", d).as_bytes()).unwrap();
            println!("\rChecking if {} exists in target data", d);
            stdout.queue(cursor::RestorePosition).unwrap();
            stdout.flush().unwrap();
            thread::sleep(Duration::from_millis(100));
            let mut copying = match target_data.get(&d) {
                Some(target_data) => {
                    // if the target data is a directory, skip it
                    if target_data.is_dir {
                        continue;
                    }
                    // if the target data is older than the base data, copy it to the target
                    target_data.last_modified < data.last_modified 
                }
                None => true
                
            };

            stdout.queue(cursor::SavePosition).unwrap();
            if copying {
                total_new_files += 1;
                // stdout.write_all(format!("\r [{}] {} to target \n", "Copying".blue(), d).as_bytes()).unwrap();
                println!("\r [{}] {} to target \n", "Copying".blue(), d);
                // copy the file to the target
                let mut base_path = PathBuf::from(binding.base.clone());
                base_path.push(&data.path);
                let mut target_path = PathBuf::from(binding.target.clone());
                target_path.push(&data.path);
                // create the parent directories if they don't exist
                if let Some(parent) = target_path.parent() {
                    std::fs::create_dir_all(parent).unwrap();
                }

                match fs::copy(base_path, target_path) {
                    Ok(_) => {
                        success_count += 1;
                        // stdout.write_all(format!("Successfully copied {:?} to target", d).as_bytes()).unwrap();
                        println!("Successfully copied {:?} to target", d);
                        stdout.queue(cursor::RestorePosition).unwrap();
                        stdout.flush().unwrap();
                        thread::sleep(Duration::from_millis(100));
                        stdout.queue(cursor::RestorePosition).unwrap();
                        stdout.queue(terminal::Clear(terminal::ClearType::FromCursorDown)).unwrap();
                    }
                    Err(err) => {
                        stdout.queue(cursor::RestorePosition).unwrap();
                        stdout.flush().unwrap();
                        stdout.queue(cursor::RestorePosition).unwrap();
                        stdout.queue(terminal::Clear(terminal::ClearType::FromCursorDown)).unwrap();
                        print_error(format!("\nError copying {:?} to target: {}", d, err).as_str(), false);
                    }
                }
            }
            
        }
        stdout.execute(cursor::Show).unwrap();
        println!("Successfully copied {}/{} files", success_count.to_string().green(), total_new_files.to_string().blue());
        self.save_target_lock_data();
    }
    

    pub fn path_transaction_vec_to_hash_map(data: &Vec<PathTransactionData>) -> HashMap<String, PathTransactionData> {
        let mut map = HashMap::new();
        for d in data {
            map.insert(d.path.clone(), d.clone());
        }
        return map;
    }
}
