# Hard-Sync-Cli 
This is a simple cli tool to sync files between two directories. It is written in Rust

## Reason for creation
I mainly created this project for two reasons:
- I wanted to learn Rust and this was a good project to start with (cause probably another tool like this already exists)
- I wanted to sync my files between my laptop and my external hard drive(esp my-movies), and not having to think about which files to copy and dealing with windows `similar file exists` dialog box was a pain. So I created this tool to do it for me.

## Installation
### From source
#### Prerequisites
- Rust
- Cargo
#### Steps
1. Clone the repository
2. Run `cargo install --path .` in the root directory of the repository
3. Run `hard-sync-cli` to see the help message

### From binary
#### Steps
1. Download the binary from the [releases]()
2. Run `hard-sync-cli` to see the help message

## Usage

### Help
To see the help message, run
```bash
hard-sync-cli --help || hard-sync-cli -h
```

### Syncing
To sync two directories, run
```bash
hard-sync-cli sync <source> <destination>
```
##### How it works
- This will sync the files from the source directory to the destination directory. 
- If a file exists in the destination directory but not in the source directory, it will be *WONT BE* deleted. 
- If a file exists in the source directory but not in the destination directory, it will be copied to the destination directory.
- If a file exists in both directories, it would check if the file in the source directory is newer or recently modified than the file in the destination directory. If it is, it will be copied to the destination directory. If not, Nothing will happen to the file in the destination directory.

##### Available options
- `-i, --init` : This will initialize hard-sync metadata in any of the directories where `hard-sync-cli` have not been initialized. 
> Note: This is required for first time syncing between two directories. If you have already synced two directories, you don't need to use this option again.
- `-r, -reverse` : This will reverse the source and destination directories. This means that the source directory will be the destination directory and the destination directory will be the source directory.
- `-b --both` : This will sync both directories. This means that the files in the source directory will be copied to the destination directory and the files in the destination directory will be copied to the source directory.


### Tools and Libraries used
- [Fli](https://github.com/codad5/fli) - A Rust library for parsing command line arguments like commander.js
- [walkdir](https://crates.io/crates/walkdir) - A Rust library for walking through directories recursively
- [chrono](https://crates.io/crates/chrono) - A Rust library for dealing with time
- [fs_extra](https://crates.io/crates/fs_extra) - A Rust library for dealing with files and directories

### License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details

### Contributing
Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

### TODO
- [x] Add tests
- [ ] Support ignoring files/directories