# **Hard-Sync-CLI**

**Hard-Sync-CLI** is a fast and lightweight command-line tool for synchronizing two directories, written in **Rust**. It offers powerful features, including support for dry runs, file exclusion, and reverse syncing, making it a practical solution for anyone looking to manage file syncing efficiently.

---

## **🚀 Why Hard-Sync-CLI?**

- **Speed and Simplicity**: Written in Rust, ensuring fast and reliable performance.
- **Customization**: Allows file exclusion and dry-run testing to give you full control.
- **Learning Project**: Built as a hands-on project to explore Rust programming while solving a real-world problem.

---

## **📥 Installation**

### **Install From Source**
1. **Prerequisites**: Install Rust and Cargo ([Get Rust](https://www.rust-lang.org/tools/install)).
2. Clone this repository:
   ```bash
   git clone https://github.com/your-username/hard-sync-cli.git
   cd hard-sync-cli
   ```
3. Install the tool globally:
   ```bash
   cargo install --path .
   ```
4. Confirm the installation:
   ```bash
   hsync --help
   ```

### **Install From Binary**
Prebuilt binaries will be available on the [Releases Page](#).

---

## **💻 Usage**

### **General Help**
To display the help message:
```bash
hsync --help
```

### **Command: `sync`**
The `sync` command synchronizes files between two directories.

#### **Basic Usage**
```bash
hsync sync --src <source_directory> --dest <destination_directory>
```

#### **Options**
| **Option**        | **Short-Hand** | **Description**                                                                 |
|--------------------|----------------|---------------------------------------------------------------------------------|
| `--src <path>`    | `-s <path>`    | Source directory to sync from.                                                 |
| `--dest <path>`   | `-d <path>`    | Destination directory to sync to.                                              |
| `--init`          | `-i`           | Initialize the destination directory for syncing.                              |
| `--reverse`       | `-r`           | Reverse the source and destination directories.                                |
| `--dry-run`       | `-dr`          | Perform a dry run to show what changes would be made without syncing files.    |
| `--exclude <...>` | `-e <...>`     | Exclude specific files or directories during sync. Supports multiple entries.  |

#### **Examples**
1. **First-Time Sync**:
   ```bash
   hsync sync -s /path/to/source -d /path/to/destination -i
   ```
2. **Dry Run**:
   ```bash
   hsync sync -s /path/to/source -d /path/to/destination -dr
   ```
3. **Exclude Files**:
   ```bash
   hsync sync -s /path/to/source -d /path/to/destination -e "*.tmp" "ignore-this-folder/"
   ```
4. **Reverse Sync**:
   ```bash
   hsync sync -s /path/to/source -d /path/to/destination -r
   ```

---

### **Ignoring Files and Directories**

You can specify files or directories to exclude from syncing by creating a `hard_sync.ignore` file in the destination directory. The syntax is the same as a `.gitignore` file.

#### Example `hard_sync.ignore` File:
```plaintext
# Ignore all .tmp files
*.tmp

# Ignore a specific folder
ignore-this-folder/

# Ignore a specific file
do-not-sync.txt
```

---

## **📦 Features**
- **File Syncing**: Sync files and directories from a source to a destination with support for initialization and reverse syncing.
- **Dry Run**: Preview changes without applying them.
- **File Exclusion**: Specify files or directories to exclude using the `--exclude` option or an `hard_sync.ignore` file.
- **Metadata Initialization**: Use the `--init` flag to set up the destination directory for syncing.
- **Colorized Output**: Get detailed status information with color-coded messages for errors, successes, and warnings.

---

## **🛠️ Advanced Features**
### **Planned Features**
- **Bidirectional Syncing**: Synchronize changes in both directions (`source ↔ destination`).
- **Network Support**: Enable syncing over SSH or SFTP.
- **Versioning**: Create backups or versions of overwritten files.
- **Configuration Files**: Support for `.toml` or `.json` configuration files for advanced settings.

---

## **📄 License**
This project is licensed under the **MIT License**. See the [LICENSE](LICENSE) file for details.

---

## **🤝 Contributing**

Contributions are welcome! Follow these steps to contribute:

1. Fork the repository.
2. Create a feature branch (`git checkout -b feature/your-feature-name`).
3. Commit your changes (`git commit -m "Add your message"`).
4. Push to your fork (`git push origin feature/your-feature-name`).
5. Open a pull request.

---

## **📋 To-Do**
- [x] Add dry-run functionality.
- [x] Add file exclusion via CLI and ignore files.
- [ ] Add tests (unit and integration).
- [ ] Support bidirectional syncing.
- [ ] Improve error handling and logging.
- [ ] Provide prebuilt binaries for major platforms.
- [ ] Add support for syncing over a network.

---

### 🎉 **Thank You!**
Enjoy using **Hard-Sync-CLI**! If you encounter any issues, feel free to open an issue on GitHub. 😊