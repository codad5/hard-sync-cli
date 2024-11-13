use std::fs;
use std::path::Path;
use std::time::UNIX_EPOCH;
use sha2::{Sha256, Digest};  // You may need to add `sha2` crate for hashing
#[derive(Debug, Clone)]
pub struct FileTracker {
    path: String,
    size: u64,
    content: Vec<u8>,
    last_modified: u64,
    created: u64,
    last_accessed: u64,
    extension: String,
    last_file_hash: String,
    current_file_hash: String,
}

impl FileTracker {
    pub fn new(file_path: &str) -> std::io::Result<Self> {
        let path = Path::new(file_path);
        
        // Get file metadata
        let metadata = fs::metadata(path)?;

        // Read file content
        let content = fs::read(path)?;

        let last_modified = metadata.modified()?.duration_since(UNIX_EPOCH).unwrap().as_secs();
        let created = metadata.created()?.duration_since(UNIX_EPOCH).unwrap().as_secs();
        let last_accessed = metadata.accessed()?.duration_since(UNIX_EPOCH).unwrap().as_secs();


        // File extension
        let extension = path.extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("")
            .to_string();

        // File hashes
        let current_file_hash = Self::calculate_hash(&content);
        let last_file_hash = current_file_hash.clone();  // Assume it's the same initially

        Ok(FileTracker {
            path: file_path.to_string(),
            size: metadata.len(),
            content,
            last_modified,
            created,
            last_accessed,
            extension,
            last_file_hash,
            current_file_hash,
        })
    }

    pub fn calculate_hash(content: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(content);
        format!("{:x}", hasher.finalize())  // Return as a hex string
    }

    pub fn update_hash(&mut self) {
        self.last_file_hash = self.current_file_hash.clone();
        self.current_file_hash = Self::calculate_hash(&self.content);
    }
}



// impl to get all the file info
impl FileTracker {
    pub fn get_path(&self) -> &str {
        &self.path
    }

    pub fn get_relative_path(&self, base_path: &Path) -> &str {
        let path = Path::new(&self.path);
        path.strip_prefix(base_path).unwrap().to_str().unwrap()
    }

    pub fn get_size(&self) -> u64 {
        self.size
    }

    pub fn get_content(&self) -> &[u8] {
        &self.content
    }

    pub fn get_last_modified(&self) -> u64 {
        self.last_modified
    }

    pub fn get_created(&self) -> u64 {
        self.created
    }

    pub fn get_last_accessed(&self) -> u64 {
        self.last_accessed
    }

    pub fn get_extension(&self) -> &str {
        &self.extension
    }

    pub fn get_last_file_hash(&self) -> &str {
        &self.last_file_hash
    }

    pub fn get_current_file_hash(&self) -> &str {
        &self.current_file_hash
    }
}


// implemtation for loading the file