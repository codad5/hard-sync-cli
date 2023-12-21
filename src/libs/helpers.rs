use crate::modules::transaction::Transaction;

use std::{path::{PathBuf, Component, Path, self}, time::{SystemTime, UNIX_EPOCH}};
use fs_extra::{TransitProcess, dir::TransitProcessResult};

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

pub fn get_transaction(base: String, target: String) -> Transaction {
    Transaction::new(base, target)
}

pub fn system_time_to_u64(system_time: SystemTime) -> u64 {
    system_time.duration_since(UNIX_EPOCH).unwrap().as_secs()
}

pub fn get_relative_path(parent_path: &str, child_path: &str) -> Option<String> {
    let parent_path = Path::new(parent_path);
    let child_path = Path::new(child_path);

    if let Ok(rel_path) = child_path.strip_prefix(parent_path) {
        return Some(rel_path.to_str().unwrap().to_string());
    } 
    // else if the beginning of the child path is the end of the parent then there is a possible relative path
    else if let Some(parent_first_component) = parent_path.components().last()  
    {
        if parent_path.components().count() <= 1 {
            return None
        }
        return get_relative_path(parent_first_component.as_os_str().to_str().unwrap(), child_path.as_os_str().to_str().unwrap());
    }
    
    None
    
}

pub fn file_copy_process_handler(process_info: TransitProcess) -> TransitProcessResult {
    let TransitProcess { total_bytes, copied_bytes, file_name , ..} = process_info;
    let percentage = (copied_bytes as f64 / total_bytes as f64) * 100.0;
    println!("{}: {:.2}%", file_name.as_str(), percentage);
    TransitProcessResult::ContinueOrAbort
}

/**
 * files to copy - paths of all files to copy
 * target - expected end dir 
 * base - paths where those files are located 
 */
pub fn map_path_to_target(files_to_copy: Vec<String>, target: String, base: String) -> Vec<(Vec<String>, String)> {
    println!("Mapping files to target ==================== ===============");
    println!("==========================================================");
    println!("Files to copy: {:?}", files_to_copy);
    println!("Target: {}", target);
    println!("Base: {}", base);
    
    let mut mapped_files: Vec<(Vec<String>, String)> = Vec::new();

    for file in files_to_copy {
        // Remove base path from file
        let rel_file = match get_relative_path(base.as_str(), file.as_str()) {
            Some(x) => x,
            None => file.clone()
        };
        // let mut file_parts = rel_file.split("/").collect::<Vec<&str>>();
        let mut file_parts = Path::new(rel_file.as_str()).components().map(|x| x.as_os_str().to_str().unwrap()).collect::<Vec<&str>>();
        // let file_name = file_parts.pop().unwrap();
        file_parts.pop();
        let mut target_path = target.clone();

        for path_part in file_parts.iter() {
            // if path_part is empty, skip
            if path_part.is_empty() {
                continue;
            }

            // If target_path does not end with a path separator, add one
            if !target_path.ends_with(path::MAIN_SEPARATOR) {
                // target_path.push_str(path::MAIN_SEPARATOR.to_string().as_str());
                target_path.push('/');
            }

            target_path.push_str(path_part);
        }
        let path_for_sys = Path::new(file.as_str());
        let path_for_sys = path_for_sys.components().filter_map(|x| {
            //remove path that are not a possible file name
            if ["/", "//", ".", "\\"].contains(&x.as_os_str().to_str().unwrap()) {
                None
            } else {
                Some(x.as_os_str().to_str().unwrap().to_string())
            }}).collect::<Vec<String>>().join("/");
        // Check if that target already exists in mapped_files; if not found, create a new entry
        if let Some(existing_mapping) = mapped_files.iter_mut().find(|x| { x.1 == target_path}) {
            existing_mapping.0.push(path_for_sys);
        } else {
            mapped_files.push((vec![path_for_sys], target_path));
        }
        
    }

    println!("Mapped files: {:?}", mapped_files);
    println!("==========================================================");

    mapped_files
}