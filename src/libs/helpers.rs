use std::path::{PathBuf, Component, Path, self};

use log::error;

pub fn get_running_path() -> String {
    let mut path = std::env::current_exe().unwrap();
    path.pop();
    path.to_str().unwrap().to_string()
}


pub fn get_calling_path() -> String {
    let mut path = std::env::current_dir().unwrap();
    path.to_str().unwrap().to_string()
}


pub fn get_drive_root_dir(path: &str) -> Option<String> {
    if let Some(drive_component) = Path::new(path).components().next() {
        if let Component::Prefix(prefix_component) = drive_component {
            if let Some(drive_str) = prefix_component.as_os_str().to_str() {
                return Some(drive_str.to_string());
            }
        }
    }
    None
}

pub fn print_error(message: &str, exit: bool) {
    error!("{}", message);

    // if it is release mode print to console
    if cfg!(not(debug_assertions)) {
        println!("{}", message);
    }

    if exit {
        std::process::exit(1);
    }
}

pub fn resolve_path(path: String) -> PathBuf {
    let mut path = PathBuf::from(path);
    if path.is_relative() {
        let mut _path = PathBuf::from(get_calling_path());
        _path.push(path.clone());
        path = _path;
    }
    path
}

pub fn is_already_initialized(path: &PathBuf) -> bool {
    let mut _path = path.clone();
    _path.push(".hard-sync");
    _path.exists() && _path.is_dir()
}

pub fn perform_initialization(path: &PathBuf, init: bool) -> bool {
    let already_initialized = is_already_initialized(path);
    if init && !already_initialized {
        prepare_path_initalization(path);
    }
    return is_already_initialized(path);
}

pub fn prepare_path_initalization(path: &PathBuf)  {
    // create .hard-sync directory and all necessary files
    std::fs::create_dir(path.join(".hard-sync")).unwrap();
    
}