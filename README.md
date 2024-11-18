# Hard-Sync-CLI 

**Hard-Sync-CLI** is a simple and efficient command-line tool to sync files between two directories. It is written in **Rust** for speed and reliability. 

---

## 🚀 **Why Hard-Sync-CLI?**

I built this project for two main reasons:

1. **Learning Rust**: This project helped me learn Rust through hands-on experience.
2. **Practical Use**: I needed an easier way to sync files between my laptop and external hard drive (e.g., my movie collection) without the headache of dealing with duplicate file dialogs or manual comparisons.

---

## 📥 **Installation**

### Install From Source
#### Prerequisites
- **Rust** (including `cargo`) installed on your system. [Get Rust](https://www.rust-lang.org/tools/install)

#### Steps
1. Clone this repository:
   ```bash
   git clone https://github.com/your-username/hard-sync-cli.git
   cd hard-sync-cli
   ```

2. Install the CLI tool globally using `cargo`:
   ```bash
   cargo install --path .
   ```

3. Run the tool to confirm installation:
   ```bash
   hsync --help
   ```

### Install From Binary
1. Download the prebuilt binary from the [Releases Page](#) (coming soon).
2. Add the binary to your system PATH (if needed).
3. Run the tool:
   ```bash
   hsync --help
   ```

---

## 💻 **Usage**

### General Help
To see the help message, run:
```bash
hsync --help
```

---

### Syncing Directories
To sync files between two directories, use the following command:
```bash
hsync sync --src <source_directory> --dest <destination_directory>
```

#### **How It Works**
- **Files in Source Not in Destination**: Copied to the destination directory.
- **Files in Destination Not in Source**: *Ignored* (not deleted).
- **Modified Files**: If the source version is newer, it replaces the destination version. Otherwise, no changes occur.

#### **Options for `sync`**
| **Flag**          | **Short-Hand** | **Description**                                                                 |
|--------------------|----------------|---------------------------------------------------------------------------------|
| `--init`          | `-i`           | Initializes Hard-Sync metadata in the destination folder. Required for first-time syncing. |
| `--reverse`       | `-r`           | Reverses the source and destination directories (syncs in the opposite direction). |
| `--both`          | `-b`           | Synchronizes both directions (source to destination and destination to source). |
| `--src <path>`    | `-s <path>`    | Specifies the source directory to sync from.                                    |
| `--dest <path>`   | `-d <path>`    | Specifies the destination directory to sync to.                                |

#### **Examples**
- First-time sync:
  ```bash
  hsync sync -s /path/to/source -d /path/to/destination -i
  ```
- Reverse sync:
  ```bash
  hsync sync -s /path/to/source -d /path/to/destination -r
  ```
- Bidirectional sync:
  ```bash
  hsync sync -s /path/to/source -d /path/to/destination -b
  ```

---

### Ignoring Files and Directories

To ignore files or directories from being synced:
1. Create a file named **`hard_sync.ignore`** in the destination directory.
2. Add the files or directories to be ignored using the same syntax as a `.gitignore` file.

#### Example of `hard_sync.ignore`:
```plaintext
# Ignore all `.tmp` files
*.tmp

# Ignore a specific folder
ignore-this-folder/

# Ignore a specific file
do-not-copy.txt
```

---

## 📦 **Dependencies**

The following Rust libraries power this project:
- [**Fli**](https://github.com/codad5/fli): Command-line argument parser similar to `commander.js`
- [**walkdir**](https://crates.io/crates/walkdir): For recursive directory traversal
- [**chrono**](https://crates.io/crates/chrono): For date and time management
- [**fs_extra**](https://crates.io/crates/fs_extra): For advanced file and directory operations
- [**colored**](https://crates.io/crates/colored): For colorful terminal outputs
- [**serde**](https://crates.io/crates/serde): For JSON serialization/deserialization

---

## 📄 **License**
This project is licensed under the **MIT License**. See the [LICENSE](LICENSE) file for details.

---

## 🤝 **Contributing**

Contributions are welcome! If you'd like to contribute:
1. Fork this repository.
2. Create a new branch (`git checkout -b feature-branch`).
3. Make your changes and commit them.
4. Submit a pull request.

For major changes, please open an issue first to discuss your ideas.

---

## 🛠️ **TODO**
- [x] Add tests
- [x] Add support for ignoring specific files/directories
- [ ] Set up CI/CD for automated testing, building, and releasing
- [ ] Improve error handling and logging
- [ ] Add detailed documentation with examples

---

### 🎉 **Thank you for using Hard-Sync-CLI!**

For feedback, suggestions, or issues, feel free to open an issue on GitHub. 😊 
