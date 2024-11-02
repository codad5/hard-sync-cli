mod controllers;
use std::path::Path;

use fli::{init_fli_from_toml, Fli};

// hard sync cli a cli tool for syncing 2 directories similar to rsync but with a few more features
fn main() {

    let mut app = init_fli_from_toml!();
    let mut sync = app.command("sync", "Syncs 2 directories");
    sync.option("-s --src, <>", "Source Directory", sync_callback);
    sync.option("-d --dest, <>", "Destination Directory", sync_callback);


    sync.allow_duplicate_callback(false);

    app.run();
}



fn sync_callback(x: &Fli){
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