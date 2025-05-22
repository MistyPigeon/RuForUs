mod usb;
mod onedrive;
mod file_explorer;

use std::env;
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: datrain <usb|onedrive|explorer> [options]");
        return;
    }

    match args[1].as_str() {
        "usb" => {
            // Call the C program for Rufus integration
            let status = Command::new("./rufus_usb")
                .status()
                .expect("Failed to execute Rufus USB C integration");
            if !status.success() {
                eprintln!("Rufus USB operation failed");
            }
        }
        "onedrive" => {
            // Call the C program for OneDrive sync
            let status = Command::new("./onedrive_sync")
                .status()
                .expect("Failed to execute OneDrive sync C integration");
            if !status.success() {
                eprintln!("OneDrive sync operation failed");
            }
        }
        "explorer" => {
            // Call the Perl script for file copy
            let status = Command::new("perl")
                .arg("file_copy.pl")
                .status()
                .expect("Failed to execute File Explorer Perl copy");
            if !status.success() {
                eprintln!("File copy operation failed");
            }
        }
        _ => eprintln!("Unknown command"),
    }
}
