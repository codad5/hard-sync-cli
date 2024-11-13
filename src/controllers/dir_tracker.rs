use std::path::Path;
use super::file_tracker::FileTracker;
use walkdir::WalkDir;
pub struct DirTracker{
    // path of the directory
    path : String,

    // meta data 
    size : u64,
    last_modified : u64,
    created : u64,
    is_dir : bool,


    // dir contents
    sub_dirs : Vec<DirTracker>,
    files: Vec<FileTracker>

}

impl DirTracker {
    pub fn new(path: &Path) -> Result<DirTracker, String> {
        let metadata = path.metadata().unwrap();
        let path = path.to_str().unwrap().to_string();
        let size = metadata.len();
        let last_modified = metadata.modified().unwrap().elapsed().unwrap().as_secs();
        let created = metadata.created().unwrap().elapsed().unwrap().as_secs();
        let is_dir = metadata.is_dir();

        Ok(DirTracker {
            path,
            size,
            last_modified,
            created,
            is_dir,
            sub_dirs: Vec::new(),
            files: Vec::new()
        })
    }

    pub fn add_sub_dir(&mut self, sub_dir: DirTracker) {
        self.sub_dirs.push(sub_dir);
    }

    pub fn add_file(&mut self, file: FileTracker) {
        self.files.push(file);
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

    pub fn is_dir(&self) -> bool {
        self.is_dir
    }

    pub fn get_sub_dirs(&self) -> &Vec<DirTracker> {
        &self.sub_dirs
    }

    pub fn get_files(&self) -> &Vec<FileTracker> {
        &self.files
    }
}


//  implentation to get / load all the files and sub directories
impl DirTracker {
    pub fn load_files_from_self(&mut self, recursive: bool) {
        let dir = Path::new(&self.path);
        let mut walker = WalkDir::new(dir);
        if !recursive {
            walker = walker.max_depth(1);
        }
        let walker = walker.into_iter();
        let mut files = Vec::new();
        for entry in walker.filter_map(Result::ok).filter(|e| e.file_type().is_file()) {
            let metadata = entry.metadata().unwrap();
            if metadata.is_file() {
                let file_path = entry.path();
                files.push(FileTracker::new(file_path.to_str().unwrap()).unwrap());
            }
        }

        self.files = files;

    }

    pub fn get_dir_diff(&self, other: &DirTracker) -> Vec<FileTracker> {
        let mut diff = Vec::new();
        for file in &self.files {
            let mut found = false;
            for other_file in &other.files {
                if file.get_relative_path(Path::new(&self.path)) == other_file.get_relative_path(Path::new(&other.path)) {
                    found = true;
                    if file.get_last_file_hash() != other_file.get_last_file_hash() {
                        diff.push(file.clone());
                    }
                    break;
                }
            }
            if !found {
                diff.push(file.clone());
            }
        }

        diff
    }
}

