use super::file_tracker::FileTracker;
use std::{collections::HashMap, path::Path};
use walkdir::WalkDir;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DirTracker {
    // path of the directory
    path: String,

    // meta data
    size: u64,
    last_modified: u64,
    created: u64,

    // dir contents
    sub_dirs: Vec<DirTracker>,
    files: HashMap<String, FileTracker>, // Key is the file relative path
}

impl DirTracker {
    pub fn new(path: &Path) -> Result<DirTracker, String> {
        if !path.exists() {
            return Err("Path does not exist".to_string());
        }
        let metadata = path.metadata().unwrap();
        if !metadata.is_dir() {
            return Err("Path is not a directory".to_string());
        }
        let path = path.to_str().unwrap().to_string();
        let size = metadata.len();
        let last_modified = metadata.modified().unwrap().elapsed().unwrap().as_secs();
        let created = metadata.created().unwrap().elapsed().unwrap().as_secs();

        Ok(DirTracker {
            path,
            size,
            last_modified,
            created,
            sub_dirs: Vec::new(),
            files: HashMap::new(),
        })
    }

    pub fn add_sub_dir(&mut self, sub_dir: DirTracker) {
        self.sub_dirs.push(sub_dir);
    }

    pub fn add_file(&mut self, file: FileTracker) {
        self.files.insert(file.get_relative_path(Path::new(&self.path)).to_string(), file);
    }
}

//  getter methods
impl DirTracker {
    pub fn get_path(&self) -> &str {
        &self.path
    }

    pub fn get_size(&self) -> u64 {
        self.size
    }

    pub fn get_last_modified(&self) -> u64 {
        self.last_modified
    }

    pub fn get_created(&self) -> u64 {
        self.created
    }

    pub fn get_sub_dirs(&self) -> &Vec<DirTracker> {
        &self.sub_dirs
    }

    pub fn get_files(&self) -> Vec<FileTracker> {
        self.files.values().cloned().collect()
    }

    pub fn get_file_hashmap(&self) -> &HashMap<String, FileTracker> {
        &self.files
    }

    pub fn get_file(&self, file_path: &str) -> Option<&FileTracker> {
        self.files.get(file_path)
    }

    pub fn has_file(&self, file_path: &str) -> bool {
        self.files.contains_key(file_path)
    }
}

//  implentation to get / load all the files and sub directories
impl DirTracker {
    pub fn import_files_from_directory(&mut self, recursive: bool) {
        let dir = Path::new(&self.path);
        let mut walker = WalkDir::new(dir);
        if !recursive {
            walker = walker.max_depth(1);
        }
        let walker = walker.into_iter();
        let mut files = HashMap::new();
        for entry in walker
            .filter_map(Result::ok)
            .filter(|e| e.file_type().is_file())
        {
            let metadata = entry.metadata().unwrap();
            if metadata.is_file() {
                let file_path = entry.path();
                files.insert(
                    // relative path
                    file_path
                        .strip_prefix(Path::new(&self.path))
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string(),
                    FileTracker::new(file_path.to_str().unwrap()).unwrap(),
                );
            }
        }

        self.files = files;
    }

    pub fn get_dir_diff(&self, other: &DirTracker) -> Vec<FileTracker> {
        let mut diff = Vec::new();
        for (key, file) in &self.files {
            if let Some(other_file) = other.files.get(key) {
                if file.get_last_file_hash() != other_file.get_last_file_hash() {
                    diff.push(file.clone());
                }
            } else {
                diff.push(file.clone());
            }
        }
        diff
    }
}
