use fli::Fli;
use log::*;

use crate::{libs::helpers::{get_calling_path, print_error, resolve_path, is_already_initialized, perform_initialization, get_transaction}, modules::transaction};


pub fn sync(x : &Fli){
    let base = match x.get_arg_at(1) {
        Some(x) => {
            let mut x = x.to_string();
            if x == "." {
                x = get_calling_path();
            }
            resolve_path(x)
        }
        None => {
            print_error("No base directory specified", true);
            return;
        }
    };
    let target = match x.get_arg_at(2) {
        Some(x) => resolve_path(x),
        None => {
            print_error("No target directory specified", true);
            return;
        }
    };

    // check if paths exist
    if !base.exists() {
        print_error(format!("Base directory does not exist: {}", base.to_str().unwrap()).as_str(), true);
        return;
    }
    if !target.exists() {
        print_error(format!("Target directory does not exist: {}", target.to_str().unwrap()).as_str(), true);
        return;
    }

    //check if hard sync is already initialized in both directories
    if !perform_initialization(&base, x.is_passed("init".to_owned())) {
        x.print_help(format!("Base directory is not initialized: {}", base.to_str().unwrap()).as_str());
        print_error(format!("Base directory is not initialized: {}", base.to_str().unwrap()).as_str(), true);
        return;
    }
    
    if !perform_initialization(&target, x.is_passed("init".to_owned())) {
        x.print_help(format!("Target directory is not initialized: {}", target.to_str().unwrap()).as_str());
        print_error(format!("Target directory is not initialized: {}", target.to_str().unwrap()).as_str(), true);
        return;
    }

    info!("syncing {} to {}", base.to_str().unwrap(), target.to_str().unwrap());

    let mut transaction = get_transaction(base.to_str().unwrap().to_string(), target.to_str().unwrap().to_string());
    let reversed : bool = x.is_passed("reversed".to_owned());
    info!("reversed: {}", reversed);
    transaction.sync(reversed);

}