use fli::Fli;
use std::fs::File;
use log::*;
use colored::Colorize;


use crate::app::commands;
use crate::libs::helpers::{get_running_path, get_drive_root_dir, get_calling_path};

use super::commands::sync;



pub fn init_app() -> Result<(), &'static str>
{
    // check root directory of the disk for a file called ".hard-sync" like "c:/hard-sync" or "/hard-sync" or "/home/hard-sync" or "D:/hard-sync"
    // if file does not exist, create it
    let root = File::open(".hard-sync");
    if root.is_err()
    {
        let mut file = File::create(".hard-sync");
        if file.is_err()
        {
            info!("Error creating file");
            return Err("Error creating file");
        }
    }
    info!("Setup complete");
    Ok(())
}

pub fn init(){
    init_app().expect("Error initializing app");
    // let root_dir = get_running_path();
    // println!("root_dir: {}", root_dir);
    // let root_dir = get_drive_root_dir(&root_dir).unwrap();
    // println!("root_dir: {}", root_dir);
    // let call_dir = get_calling_path();
    // println!("call_dir: {}", call_dir);
    // let call_dir_root = get_drive_root_dir(&call_dir).unwrap();
    // println!("call_dir_root: {}", call_dir_root);   


    let mut app = Fli::init("my app",  "Help sync files between two directories");
    let sync_command = app.command("sync", "Sync files between two directories");

    sync_command.default(commands::sync);
    sync_command.option("-i --init", "Initialize hard-sync in the current directory", commands::sync);
    sync_command.option("-r --reverse", "Sync from target to base", commands::sync);


    app.run();
    
}