pub struct FileTracker {
    path : String,
    size : u64, 
    // content prop a file buffer
    content : Vec<u8>,
    last_modified : u64,
    created : u64,
    last_accessed : u64,
    extension : String,
    last_file_hash : String,
    current_file_hash : String,
    

}