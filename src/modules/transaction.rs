use std::{path::PathBuf, fs::{File, OpenOptions, self}, io::{BufReader, BufRead, Write, Seek, self}};
use chrono::{Local, DateTime};
use log::info;
use serde::{Serialize, Deserialize};
use walkdir::WalkDir;

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
            println!("trad data: {:?}", line);
            let line = line.unwrap();
            println!("line: {}", line);
            match serde_json::from_str::<PathTransactionData>(&line) {
                Ok(d) => {
                    data.push(d.clone());
                    // if d.is_dir recursively get all files and folders
                    if d.is_dir {
                        let mut path_d = PathBuf::from(d.path);
                        let mut path_data = Transaction::get_lock_data(&path_d);
                        data.append(&mut path_data);
                    }
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
        info!("About to get save data for path: {:?}", path);
        let mut data: Vec<PathTransactionData> = Vec::new();
        let mut lock_file = Transaction::resolve_lock(path);
        for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
            info!("entry: {:?}", entry);
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
            data.push(PathTransactionData {
                path,
                last_modified : system_time_to_string(last_modified),
                last_accessed : system_time_to_string(last_accessed),
                created : system_time_to_string(created),
                size : size.to_string(),
                is_dir,
            });
        }
        return data;
    }


    pub fn get_base_save_data(&self) -> Vec<PathTransactionData> {
        info!("About to get base save data");
        return Transaction::get_save_data_path_transaction_data(&PathBuf::from(self.base.clone()));
    }

    pub fn get_target_save_data(&self) -> Vec<PathTransactionData> {
        info!("About to get target save data");
        return Transaction::get_save_data_path_transaction_data(&PathBuf::from(self.target.clone()));
    }
    
}