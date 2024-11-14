mod controllers;
use std::{fs, path::Path};

use controllers::{dir_tracker::DirTracker, file_tracker::FileTracker};
use fli::{init_fli_from_toml, Fli};

// hard sync cli a cli tool for syncing 2 directories similar to rsync but with a few more features
fn main() {
    let mut app = init_fli_from_toml!();
    let mut sync = app.command("sync", "Syncs 2 directories");
    sync.option("-s --src, <>", "Source Directory", sync_callback);
    sync.option("-d --dest, <>", "Destination Directory", sync_callback);
    sync.allow_duplicate_callback(false);

    let mut test = app.command("test", "Test command");
    test.default(test_callback);

    app.run();
}

fn sync_callback(x: &Fli) {
    let src = match x.get_values("src".to_string()) {
        Ok(v) => v.first().unwrap().clone(),
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    let dest = match x.get_values("dest".to_string()) {
        Ok(v) => v.first().unwrap().clone(),
        Err(e) => {
            println!("{}", e);
            return;
        }
    };

    // get path of src and dest
    let src = Path::new(&src);
    let dest = Path::new(&dest);

    // if the dir does not exist and -n is not passed then create the dir
    if !x.is_passed("-n".to_owned()) {
        if !src.exists() {
            println!("Source directory does not exist");
            return;
        }

        if !dest.exists() {
            println!("Destination directory does not exist");
            return;
        }
    }

    println!("Syncing {:?} to {:?}", src, dest);
}

fn test_callback(x: &Fli) {
    println!("Test");
    let index_path = "./test-data/index.tsx";
    let nav_path = "./test-data-1/index.tsx";
    let index_d_path = Path::new(index_path);
    let nav_d_path = Path::new(nav_path);
    // print path exists
    println!("Index path exists: {}", index_d_path.exists());
    println!("Nav path exists: {}", nav_d_path.exists());

    // print path is file
    println!("Index path is file: {}", index_d_path.is_file());
    println!("Nav path is file: {}", nav_d_path.is_file());

    // Get file metadata
    let index_metadata = fs::metadata(index_d_path).unwrap();
    let nav_metadata = fs::metadata(nav_d_path).unwrap();

    let index_hash = FileTracker::calculate_hash(&fs::read(index_d_path).unwrap());
    let nav_hash = FileTracker::calculate_hash(&fs::read(nav_d_path).unwrap());

    // print the hash
    println!("Index hash: {}", index_hash);
    println!("Nav hash: {}", nav_hash);

    // check if they are same
    println!("Index and Nav are same: {}", index_hash == nav_hash);
    let index_d_path = Path::new("./test-data-1");
    let nav_d_path = Path::new("./test-data");
    let mut dir = DirTracker::new(index_d_path).unwrap();
    let mut dir1 = DirTracker::new(nav_d_path).unwrap();
    println!("Dir path: {}", dir.get_path());
    println!("Before loading sub dirs and files");
    for file in dir.get_files() {
        println!("File path: {}", file.get_path());
    }
    println!("After loading sub dirs and files");
    dir.import_files_from_directory(true);
    dir1.import_files_from_directory(true);

    println!("Dir path: {:?}", dir.get_file_hashmap());


    for file in dir.get_files() {
        println!("File path: {}", file.get_path());
    }
    // show all files for dir1
    println!("Dir1 path: {}", dir1.get_path());
    for file in dir1.get_files() {
        println!("1 File path: {}", file.get_path());
    }
    println!("loading diff");
    let diff = dir.get_dir_diff(&dir1);
    if diff.len() == 0 {
        println!("No diff found");
    }
    for file in diff {
        println!(
            "Diff file path: {}",
            file.get_relative_path(Path::new(&dir.get_path()))
        );
    }
}
