use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Attempts to find the user's OneDrive Personal folder on Windows.
fn get_onedrive_path() -> Option<PathBuf> {
    // Check the environment variable provided by OneDrive on Windows
    if let Ok(path) = env::var("OneDrive") {
        let p = PathBuf::from(path);
        // Confirm it exists and is a directory
        if p.exists() && p.is_dir() {
            return Some(p);
        }
    }
    // Try default path as fallback
    if let Ok(userprofile) = env::var("USERPROFILE") {
        let candidate = PathBuf::from(userprofile).join("OneDrive");
        if candidate.exists() && candidate.is_dir() {
            return Some(candidate);
        }
    }
    None
}

/// Caches files from the provided source directory to the user's OneDrive Personal directory.
pub fn cache_to_onedrive() {
    let source_dir = "./cache_to_onedrive"; // You can change this as needed

    let onedrive_path = match get_onedrive_path() {
        Some(path) => path,
        None => {
            eprintln!("Could not locate OneDrive Personal directory. Is OneDrive installed and set up?");
            return;
        }
    };

    let src = Path::new(source_dir);
    if !src.exists() || !src.is_dir() {
        eprintln!(
            "Source directory '{}' does not exist. Place files to sync to OneDrive here.",
            source_dir
        );
        return;
    }

    // Iterate files in the source directory and copy them to OneDrive
    let entries = match fs::read_dir(src) {
        Ok(entries) => entries,
        Err(e) => {
            eprintln!("Failed to read source directory: {}", e);
            return;
        }
    };

    for entry in entries {
        if let Ok(entry) = entry {
            let file_type = match entry.file_type() {
                Ok(ft) => ft,
                Err(_) => continue,
            };
            if file_type.is_file() {
                let file_name = entry.file_name();
                let dest_path = onedrive_path.join(&file_name);
                match fs::copy(entry.path(), &dest_path) {
                    Ok(_) => println!("Copied {:?} to {:?}", entry.path(), dest_path),
                    Err(e) => eprintln!("Failed to copy {:?}: {}", entry.path(), e),
                }
            }
        }
    }

    println!(
        "Sync to OneDrive requested. OneDrive client will upload files automatically if running."
    );
}
