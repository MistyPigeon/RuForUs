use std::collections::VecDeque;
use std::env;
use std::fs::{self, DirEntry, File, OpenOptions};
use std::io::{self, BufRead, BufReader, Read, Write};
use std::path::{Path, PathBuf};
use std::time::{SystemTime, UNIX_EPOCH};

/// Represents a file or directory and its metadata.
#[derive(Debug)]
pub struct FileItem {
    pub path: PathBuf,
    pub is_dir: bool,
    pub size: u64,
    pub modified: Option<SystemTime>,
}

impl FileItem {
    pub fn new(entry: &DirEntry) -> io::Result<Self> {
        let metadata = entry.metadata()?;
        Ok(FileItem {
            path: entry.path(),
            is_dir: metadata.is_dir(),
            size: if metadata.is_file() { metadata.len() } else { 0 },
            modified: metadata.modified().ok(),
        })
    }

    pub fn display(&self) {
        let file_type = if self.is_dir { "<DIR>" } else { "     " };
        let size_disp = if self.is_dir { "".to_string() } else { format!("{:>10}", self.size) };
        let mod_disp = match self.modified {
            Some(m) => {
                let duration = m.duration_since(UNIX_EPOCH).unwrap_or_default().as_secs();
                format!("{}", duration)
            }
            None => "n/a".to_string(),
        };
        println!(
            "{} {:>10} {:<40} {}",
            file_type,
            size_disp,
            self.path.file_name().unwrap().to_string_lossy(),
            mod_disp
        );
    }
}

/// Lists contents in a directory with optional recursion.
pub fn list_dir(path: &Path, recursive: bool) -> io::Result<()> {
    let mut queue = VecDeque::new();
    queue.push_back(path.to_path_buf());
    while let Some(current_path) = queue.pop_front() {
        let entries = match fs::read_dir(&current_path) {
            Ok(e) => e,
            Err(e) => {
                eprintln!("Cannot access {:?}: {}", current_path, e);
                continue;
            }
        };
        println!("\nListing: {:?}", current_path);
        for entry in entries {
            let entry = entry?;
            let file_item = FileItem::new(&entry)?;
            file_item.display();
            if recursive && file_item.is_dir {
                queue.push_back(file_item.path.clone());
            }
        }
    }
    Ok(())
}

/// Copies a file from src to dst.
pub fn copy_file(src: &Path, dst: &Path) -> io::Result<u64> {
    let mut src_file = File::open(src)?;
    let mut dst_file = File::create(dst)?;
    let copied = io::copy(&mut src_file, &mut dst_file)?;
    fs::set_permissions(dst, fs::metadata(src)?.permissions())?;
    Ok(copied)
}

/// Recursively copies a directory.
pub fn copy_dir(src: &Path, dst: &Path) -> io::Result<()> {
    if !dst.exists() {
        fs::create_dir(dst)?;
    }
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let src_path = entry.path();
        let dst_path = dst.join(entry.file_name());
        if src_path.is_dir() {
            copy_dir(&src_path, &dst_path)?;
        } else {
            copy_file(&src_path, &dst_path)?;
        }
    }
    Ok(())
}

/// Moves a file or directory.
pub fn move_path(src: &Path, dst: &Path) -> io::Result<()> {
    if src.is_dir() {
        copy_dir(src, dst)?;
        fs::remove_dir_all(src)?;
    } else {
        fs::rename(src, dst)?;
    }
    Ok(())
}

/// Deletes a file or directory (recursive for directories).
pub fn delete_path(path: &Path) -> io::Result<()> {
    if path.is_dir() {
        fs::remove_dir_all(path)?;
    } else {
        fs::remove_file(path)?;
    }
    Ok(())
}

/// Reads the contents of a file and prints to stdout.
pub fn cat_file(path: &Path) -> io::Result<()> {
    let mut file = File::open(path)?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer)?;
    println!("{}", buffer);
    Ok(())
}

/// Creates an empty file or updates the modification time.
pub fn touch_file(path: &Path) -> io::Result<()> {
    if path.exists() {
        let now = filetime::FileTime::from_system_time(SystemTime::now());
        filetime::set_file_mtime(path, now)?;
    } else {
        File::create(path)?;
    }
    Ok(())
}

/// Renames a file or directory.
pub fn rename_path(src: &Path, dst: &Path) -> io::Result<()> {
    fs::rename(src, dst)?;
    Ok(())
}

/// Shows the current working directory.
pub fn print_cwd() -> io::Result<()> {
    let cwd = env::current_dir()?;
    println!("{}", cwd.display());
    Ok(())
}

/// Changes the current working directory.
pub fn change_dir(path: &Path) -> io::Result<()> {
    env::set_current_dir(path)?;
    Ok(())
}

/// Searches for files by name pattern in the directory tree.
pub fn search_files(root: &Path, pattern: &str) -> io::Result<()> {
    let mut stack = VecDeque::new();
    stack.push_back(root.to_path_buf());
    while let Some(current) = stack.pop_front() {
        let entries = match fs::read_dir(&current) {
            Ok(e) => e,
            Err(_) => continue,
        };
        for entry in entries {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                stack.push_back(path.clone());
            } else if let Some(name) = path.file_name() {
                if name.to_string_lossy().contains(pattern) {
                    println!("{}", path.display());
                }
            }
        }
    }
    Ok(())
}

/// Gets file metadata and prints details.
pub fn stat_file(path: &Path) -> io::Result<()> {
    let metadata = fs::metadata(path)?;
    println!("Path: {}", path.display());
    println!("Is directory: {}", metadata.is_dir());
    println!("Size: {}", metadata.len());
    if let Ok(modified) = metadata.modified() {
        println!("Modified: {:?}", modified);
    }
    Ok(())
}

/// Reads a file line by line.
pub fn read_lines(path: &Path) -> io::Result<()> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    for (i, line) in reader.lines().enumerate() {
        println!("{:>4}: {}", i + 1, line?);
    }
    Ok(())
}

/// Writes text to a file, overwriting or appending.
pub fn write_to_file(path: &Path, text: &str, append: bool) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(append)
        .truncate(!append)
        .open(path)?;
    file.write_all(text.as_bytes())?;
    Ok(())
}

/// Recursively calculates directory size.
pub fn dir_size(path: &Path) -> io::Result<u64> {
    let mut size = 0;
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let p = entry.path();
            if p.is_dir() {
                size += dir_size(&p)?;
            } else {
                size += fs::metadata(&p)?.len();
            }
        }
    } else {
        size = fs::metadata(path)?.len();
    }
    Ok(size)
}

/// Prints the directory tree.
pub fn print_tree(path: &Path, prefix: String) -> io::Result<()> {
    if path.is_dir() {
        println!("{}{}/", prefix, path.file_name().unwrap_or_default().to_string_lossy());
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let p = entry.path();
            print_tree(&p, format!("{}  ", prefix))?;
        }
    } else {
        println!("{}{}", prefix, path.file_name().unwrap_or_default().to_string_lossy());
    }
    Ok(())
}

/// Interactive explorer loop.
pub fn explorer_loop() -> io::Result<()> {
    let mut current_dir = env::current_dir()?;
    loop {
        print!("RuForUs:{}> ", current_dir.display());
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }
        match parts[0] {
            "ls" => {
                let rec = parts.get(1) == Some(&"-r");
                list_dir(&current_dir, rec)?;
            }
            "cd" => {
                if let Some(dir) = parts.get(1) {
                    let new_dir = current_dir.join(dir);
                    if new_dir.is_dir() {
                        current_dir = new_dir.canonicalize()?;
                    } else {
                        println!("Not a directory: {}", dir);
                    }
                }
            }
            "pwd" => {
                println!("{}", current_dir.display());
            }
            "cp" => {
                if let (Some(src), Some(dst)) = (parts.get(1), parts.get(2)) {
                    let src_path = current_dir.join(src);
                    let dst_path = current_dir.join(dst);
                    if src_path.is_dir() {
                        copy_dir(&src_path, &dst_path)?;
                    } else {
                        copy_file(&src_path, &dst_path)?;
                    }
                }
            }
            "mv" => {
                if let (Some(src), Some(dst)) = (parts.get(1), parts.get(2)) {
                    let src_path = current_dir.join(src);
                    let dst_path = current_dir.join(dst);
                    move_path(&src_path, &dst_path)?;
                }
            }
            "rm" => {
                if let Some(target) = parts.get(1) {
                    let target_path = current_dir.join(target);
                    delete_path(&target_path)?;
                }
            }
            "cat" => {
                if let Some(f) = parts.get(1) {
                    cat_file(&current_dir.join(f))?;
                }
            }
            "touch" => {
                if let Some(f) = parts.get(1) {
                    touch_file(&current_dir.join(f))?;
                }
            }
            "rename" => {
                if let (Some(src), Some(dst)) = (parts.get(1), parts.get(2)) {
                    rename_path(&current_dir.join(src), &current_dir.join(dst))?;
                }
            }
            "find" => {
                if let Some(pat) = parts.get(1) {
                    search_files(&current_dir, pat)?;
                }
            }
            "stat" => {
                if let Some(f) = parts.get(1) {
                    stat_file(&current_dir.join(f))?;
                }
            }
            "lines" => {
                if let Some(f) = parts.get(1) {
                    read_lines(&current_dir.join(f))?;
                }
            }
            "write" => {
                if let (Some(f), Some(txt)) = (parts.get(1), parts.get(2)) {
                    write_to_file(&current_dir.join(f), txt, false)?;
                }
            }
            "append" => {
                if let (Some(f), Some(txt)) = (parts.get(1), parts.get(2)) {
                    write_to_file(&current_dir.join(f), txt, true)?;
                }
            }
            "du" => {
                let size = dir_size(&current_dir)?;
                println!("Total size: {} bytes", size);
            }
            "tree" => {
                print_tree(&current_dir, "".to_string())?;
            }
            "exit" | "quit" => {
                break;
            }
            _ => {
                println!("Unknown command. Commands: ls, cd, pwd, cp, mv, rm, cat, touch, rename, find, stat, lines, write, append, du, tree, exit");
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_item_display() {
        let temp_dir = tempfile::tempdir().unwrap();
        let file_path = temp_dir.path().join("test.txt");
        File::create(&file_path).unwrap();
        let entry = fs::read_dir(temp_dir.path()).unwrap().next().unwrap().unwrap();
        let file_item = FileItem::new(&entry).unwrap();
        file_item.display();
    }

    #[test]
    fn test_copy_and_delete_file() {
        let temp_dir = tempfile::tempdir().unwrap();
        let src = temp_dir.path().join("a.txt");
        let dst = temp_dir.path().join("b.txt");
        fs::write(&src, b"hello").unwrap();
        copy_file(&src, &dst).unwrap();
        assert_eq!(fs::read(&dst).unwrap(), b"hello");
        delete_path(&dst).unwrap();
        assert!(!dst.exists());
    }

    #[test]
    fn test_touch_and_stat() {
        let temp_dir = tempfile::tempdir().unwrap();
        let file = temp_dir.path().join("touch.txt");
        touch_file(&file).unwrap();
        stat_file(&file).unwrap();
    }
}
