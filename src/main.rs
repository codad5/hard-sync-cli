mod helpers;
mod controllers;
use std::{fs, path::Path};

use colored::Colorize;
use controllers::{dir_tracker::DirTracker, file_tracker::FileTracker};
use fli::{init_fli_from_toml, Fli};
use helpers::logger::{print_error, print_success, Step};

// hard sync cli a cli tool for syncing 2 directories similar to rsync but with a few more features
fn main() {
    println!("{}", "Hard Sync CLI".cyan());
    let mut app = init_fli_from_toml!();
    let mut sync = app.command("sync", "Syncs 2 directories");
    sync.option("-s --src, <>", "Source Directory", sync_callback);
    sync.option("-d --dest, <>", "Destination Directory", sync_callback);
    sync.option("-i --init", "Initialize the directory", sync_callback);
    sync.allow_duplicate_callback(false);

    let mut test = app.command("test", "Test command");
    test.default(test_callback);

    app.run();
}

fn sync_callback(x: &Fli) {
    let src = match x.get_values("src".to_string()) {
        Ok(v) => v.first().unwrap().clone(),
        Err(e) => {
            print_error(&e);
            return;
        }
    };

    let dest = match x.get_values("dest".to_string()) {
        Ok(v) => v.first().unwrap().clone(),
        Err(e) => {
            print_error(&e);
            return;
        }
    };

    // get path of src and dest
    let src = Path::new(&src);
    let dest = Path::new(&dest);

    // if the dir does not exist and -n is not passed then create the dir
    if !x.is_passed("-n".to_owned()) {
        if !src.exists() {
            print_error("Source directory does not exist");
            return;
        }

        if !dest.exists() {
            print_error("Destination directory does not exist");
            return;
        }
    }

    // check if path are same 
    if src == dest {
        print_error("Source and destination directories are same");
        return;
    }
    

    Step::Syncing(format!("Syncing {:?} to {:?}", src, dest)).print();
    let mut src_dir = DirTracker::new(src).unwrap();
    let mut dest_dir = DirTracker::new(dest).unwrap();

    if dest_dir.dir_initialized().is_err() {
        if !x.is_passed("-i".to_owned()) {
            print_error("Source directory is not initialized");
            return;
        }
        Step::Start("Setting up source directory".to_string()).print();
        match dest_dir.setup_dir_config() {
            Ok(_) => {
                print_success("Source directory initialized");
            }
            Err(e) => {
                print_error(&e);
                return;
            }
        }
    }

    Step::Start("Loading Diff".to_string()).print();
    src_dir.import_files_from_directory(true);
    dest_dir.import_files_from_directory(true);
    let diff = src_dir.get_dir_diff(&dest_dir);
    if diff.len() == 0 {
        print_success("No diff found");
    }
    for file in diff {
        let status = match dest_dir.get_file(file.get_relative_path(Path::new(&src_dir.get_path()))) {
            Some(_) => "Modified".yellow(),
            None => "New".green(),
        };
        println!(
            "{} file path: {}",
            status,
            file.get_relative_path(Path::new(&src_dir.get_path()))
        );
        // copy the file
        let dest_file = dest.join(file.get_relative_path(Path::new(&src_dir.get_path())));
        Step::Copying((
            file.get_path().to_string(),
            dest_file.to_str().unwrap().to_string(),
        )).print();
        fs::copy(file.get_path(), dest_file.clone()).unwrap();
        Step::Completed(format!("Copied file to {:?}", dest_file)).print();

    }

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

    // print the json of dir
    let json_form = serde_json::to_string_pretty(&dir).unwrap();
    println!("Json form : {}", json_form);
}
