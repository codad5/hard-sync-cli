use super::file_tracker::FileTracker;

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