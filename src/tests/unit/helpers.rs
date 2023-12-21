use crate::libs::helpers::{get_calling_path, get_running_path, get_drive_root_dir, get_relative_path};
use std::{env};






#[test]
fn test_get_running_path() {
      
        let running_path = get_running_path();
        let binding = env::current_exe().unwrap();
        let expected_path = binding.parent().unwrap().to_str().unwrap();
        assert_eq!(running_path, expected_path);
}

#[test]
fn test_get_calling_path(){
        let calling_path = get_calling_path();

        // Assert
        // Make assertions about the expected behavior of the function
        let binding = env::current_dir().unwrap();
        let expected_path = binding.to_str().unwrap();
        assert_eq!(calling_path, expected_path);
}

#[test]
fn test_get_drive_root_dir()
{       let calling_path = "c:/dev/test";
        let root_dir = get_drive_root_dir(calling_path).unwrap();
        assert_eq!("c:", root_dir);
        // expected to return None when no root dir is found 
        let calling_path = "/dev/test";
        let root_dir = get_drive_root_dir(calling_path);
        assert!(!root_dir.is_some());

}

#[test]
fn test_get_relative_path(){
        let relative_path = get_relative_path("c:/dev/test", "c:/dev/test/code/index.js");
        assert!(relative_path.is_some());
        assert_eq!("code/index.js", relative_path.unwrap());
        
        let relative_path = get_relative_path("c:/dev/test", "c:/dev/test/index.js");
        assert!(relative_path.is_some());
        assert_eq!("index.js", relative_path.unwrap());

        let relative_path = get_relative_path("/dev/test", "/dev/test/code/index.js");
        assert!(relative_path.is_some());
        assert_eq!("code/index.js", relative_path.unwrap());

        let relative_path = get_relative_path("/dev/test", "/dev/test/index.js");
        assert!(relative_path.is_some());
        assert_eq!("index.js", relative_path.unwrap());

        // for non relative path 
        let relative_path = get_relative_path("c:/pain/test", "c:/dev/test/code/index.js");
        assert!(!relative_path.is_some());
}