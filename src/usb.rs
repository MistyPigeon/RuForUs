use std::fs::{self, File};
use std::io::{self, Write, Read, BufReader, BufWriter};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::thread::sleep;
use std::time::Duration;

/// Represents a USB device (very basic, for demonstration).
#[derive(Debug, Clone)]
pub struct UsbDevice {
    pub device_id: String,
    pub mount_point: PathBuf,
    pub label: Option<String>,
    pub total_space: Option<u64>,
    pub free_space: Option<u64>,
}

/// Lists removable drives (Windows only, basic implementation).
pub fn list_usb_devices() -> io::Result<Vec<UsbDevice>> {
    let mut usb_devices = Vec::new();
    // Query WMIC for removable drives and capture their device id and label
    let output = Command::new("wmic")
        .args(["logicaldisk", "where", "DriveType=2", "get", "DeviceID,VolumeName,Size,FreeSpace", "/format:csv"])
        .output()?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    for line in stdout.lines().skip(2) {
        let fields: Vec<&str> = line.split(',').collect();
        if fields.len() >= 5 {
            let device_id = fields[1].trim().to_string();
            let label = if fields[2].trim().is_empty() { None } else { Some(fields[2].trim().to_owned()) };
            let total_space = fields[3].trim().parse::<u64>().ok();
            let free_space = fields[4].trim().parse::<u64>().ok();
            let mount_point = PathBuf::from(&device_id);
            if mount_point.exists() {
                usb_devices.push(UsbDevice {
                    device_id,
                    mount_point,
                    label,
                    total_space,
                    free_space,
                });
            }
        }
    }
    Ok(usb_devices)
}

/// Checks if there's enough free space on the USB for the file
pub fn has_enough_space(usb: &UsbDevice, file_path: &Path) -> io::Result<bool> {
    let metadata = fs::metadata(file_path)?;
    if let Some(free) = usb.free_space {
        Ok(metadata.len() < free)
    } else {
        Ok(false)
    }
}

/// Copies a file to the given USB device with progress reporting.
pub fn copy_file_to_usb<P: AsRef<Path>>(usb: &UsbDevice, src_file: P) -> io::Result<()> {
    let file_name = src_file.as_ref().file_name().unwrap();
    let dest = usb.mount_point.join(file_name);
    let src_metadata = fs::metadata(&src_file)?;
    let total_size = src_metadata.len();
    let mut src = BufReader::new(File::open(&src_file)?);
    let mut dst = BufWriter::new(File::create(&dest)?);

    let mut transferred: u64 = 0;
    let mut buffer = [0u8; 8192];
    loop {
        let n = src.read(&mut buffer)?;
        if n == 0 { break; }
        dst.write_all(&buffer[..n])?;
        transferred += n as u64;
        print!("\rCopying {}... {}/{} bytes ({:.1}%)", file_name.to_string_lossy(), transferred, total_size, (transferred as f64 / total_size as f64) * 100.0);
        io::stdout().flush().ok();
    }
    dst.flush()?;
    println!("\nFile copied to USB: {:?}", dest);
    Ok(())
}

/// Lists all files and directories on the USB device (non-recursive).
pub fn list_files_on_usb(usb: &UsbDevice) -> io::Result<()> {
    println!("Listing files on USB ({}):", usb.device_id);
    let entries = fs::read_dir(&usb.mount_point)?;
    for entry in entries {
        let entry = entry?;
        let typ = if entry.file_type()?.is_dir() { "DIR " } else { "FILE" };
        println!("[{}] {:?}", typ, entry.file_name());
    }
    Ok(())
}

/// Deletes a file from the USB device.
pub fn delete_file_from_usb(usb: &UsbDevice, file_name: &str) -> io::Result<()> {
    let path = usb.mount_point.join(file_name);
    if path.exists() && path.is_file() {
        fs::remove_file(&path)?;
        println!("Deleted file from USB: {:?}", path);
    } else {
        println!("File not found on USB: {:?}", path);
    }
    Ok(())
}

/// Safely ejects the USB device (Windows only, uses PowerShell).
pub fn eject_usb(usb: &UsbDevice) -> io::Result<()> {
    // Try to eject using PowerShell's Remove-PhysicalDisk
    let script = format!(
        r#"
        $usb = Get-WmiObject -Class Win32_LogicalDisk | Where-Object {{$_.DeviceID -eq '{}'}}
        if ($usb) {{
            $vol = $usb.DeviceID
            $shell = New-Object -ComObject Shell.Application
            $shell.Namespace(17).ParseName($vol).InvokeVerb("Eject")
        }}
        "#,
        usb.device_id
    );
    let status = Command::new("powershell")
        .arg("-NoProfile")
        .arg("-Command")
        .arg(&script)
        .status()?;
    if status.success() {
        println!("Safely ejected USB device: {}", usb.device_id);
        Ok(())
    } else {
        eprintln!("Failed to eject USB device: {}", usb.device_id);
        Err(io::Error::new(io::ErrorKind::Other, "Failed to eject USB"))
    }
}

/// Writes a test file to the USB device to verify write access.
pub fn test_usb_write(usb: &UsbDevice) -> io::Result<()> {
    let test_file = usb.mount_point.join("test_write.txt");
    let mut file = File::create(&test_file)?;
    file.write_all(b"USB write test successful.")?;
    println!("Test file written to USB: {:?}", test_file);
    fs::remove_file(&test_file)?;
    Ok(())
}

/// Calls Rufus via a C wrapper to create bootable USB.
pub fn create_bootable_usb_with_rufus(usb: &UsbDevice, iso_path: &Path) -> io::Result<()> {
    // Assuming rufus_usb.exe is in PATH or current directory and takes arguments: <usb_path> <iso_path>
    let status = Command::new("rufus_usb.exe")
        .arg(&usb.device_id)
        .arg(iso_path)
        .status()?;
    if status.success() {
        println!("Rufus operation completed successfully.");
        Ok(())
    } else {
        eprintln!("Rufus failed to create bootable USB.");
        Err(io::Error::new(io::ErrorKind::Other, "Rufus failed"))
    }
}

/// Formats the USB device (WARNING: This will erase all data).
pub fn format_usb(usb: &UsbDevice, fs_type: &str, label: Option<&str>) -> io::Result<()> {
    let label = label.unwrap_or("USB");
    let status = Command::new("format")
        .arg(&usb.device_id)
        .arg("/FS:".to_owned() + fs_type)
        .arg("/V:".to_owned() + label)
        .arg("/Q")
        .arg("/Y")
        .status()?;
    if status.success() {
        println!("Formatted USB device: {}", usb.device_id);
        Ok(())
    } else {
        eprintln!("Failed to format USB device: {}", usb.device_id);
        Err(io::Error::new(io::ErrorKind::Other, "Failed to format USB"))
    }
}

/// Example workflow: List devices, write test, copy file, list files, delete test, eject
pub fn example_usb_workflow() -> io::Result<()> {
    let usbs = list_usb_devices()?;
    if usbs.is_empty() {
        println!("No USB devices detected.");
        return Ok(());
    }
    println!("Detected USB devices:");
    for (i, usb) in usbs.iter().enumerate() {
        println!(
            "[{}] {} at {:?} (label: {:?}, free: {:?}, total: {:?})",
            i, usb.device_id, usb.mount_point, usb.label, usb.free_space, usb.total_space
        );
    }
    // We'll use the first USB device for this example
    let usb = &usbs[0];
    test_usb_write(usb)?;
    // Example: copy a file named "example.txt" if it exists
    let src = Path::new("example.txt");
    if src.exists() && has_enough_space(usb, src)? {
        copy_file_to_usb(usb, src)?;
    }
    list_files_on_usb(usb)?;
    delete_file_from_usb(usb, "test_write.txt").ok();
    // Eject (uncomment if you want to actually eject)
    // eject_usb(usb)?;
    Ok(())
