use std::{path::PathBuf, fs::{File, OpenOptions, self}, io::{BufReader, BufRead, Write, Seek, self}};
use serde::{Serialize, Deserialize};
use walkdir::WalkDir;

use crate::libs::helpers::system_time_to_string;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct PathTransactionData {
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
}

impl Transaction {
    pub fn new(base: String, target: String) -> Transaction {
        Transaction {
            base,
            target,
            base_data : Vec::new(),
            target_data : Vec::new(),
        }
    }
    
    pub fn resolve_lock(path: &PathBuf) -> File {
        // Construct the lock file path by appending ".hard-sync/hard-sync.lock" to the directory
        let lock_path = path.join(".hard-sync/hard-sync.lock");

        // Create the parent directories if they don't exist
        if let Some(parent) = lock_path.parent() {
            std::fs::create_dir_all(parent).unwrap();
        }

        // Create the lock file if it doesn't exist
        if !lock_path.exists() {
            File::create(&lock_path).unwrap();
        }

        // Open the lock file for writing
        OpenOptions::new().write(true).open(&lock_path).unwrap()
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

   pub fn load_base_data(&mut self) {
        self.base_data = Transaction::get_lock_data(&PathBuf::from(self.base.clone()));
    }

    pub fn load_target_data(&mut self) {
        self.target_data = Transaction::get_lock_data(&PathBuf::from(self.target.clone()));
    }

    pub fn save_base_data(&self) {
    }


    fn get_lock_data(path: &PathBuf) -> Vec<PathTransactionData> {
        let mut data: Vec<PathTransactionData> = Vec::new();
        let lock_file = Transaction::resolve_lock(path);
        let reader = BufReader::new(lock_file);
        for line in reader.lines() {
            let line = line.unwrap();
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
                Err(err) => eprintln!("Error parsing line: {}", err),
            }
        }
        return data;
    }

    fn save_lock_data(path : &PathBuf, data : Vec<PathTransactionData>) {
        let mut lock_file = Transaction::resolve_lock(path);
        for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
            let file_path = entry.path();
            let metadata = fs::metadata(file_path).unwrap();
            let last_modified = metadata.modified().unwrap();
            let last_accessed = metadata.accessed().unwrap();
            let created = metadata.created().unwrap();
            let size = metadata.len();
            let is_dir = metadata.is_dir();
            let path = file_path.to_str().unwrap().to_string();
            let data = PathTransactionData {
                path,
                last_modified : system_time_to_string(last_modified),
                last_accessed : system_time_to_string(last_accessed),
                created : system_time_to_string(created),
                size : size.to_string(),
                is_dir,
            };
            let data = serde_json::to_string(&data);
            // Add data to lock file on a new line
            writeln!(lock_file, "{}", match data {
                Ok(d) => d,
                Err(err) => {
                    eprintln!("Error serializing data: {}", err);
                    String::new()
                }
            }).unwrap();
        }
        // Ensure the lock file is rewound to the beginning
        lock_file.seek(io::SeekFrom::Start(0));
    }
}