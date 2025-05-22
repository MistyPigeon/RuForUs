RuForUs

RuForUs is a privacy-first, Windows-centric toolkit for managing and caching files across USB drives (with Rufus integration), OneDrive Personal, and beyond. RuForUs empowers users to securely cache, scan, and protect their files using a blend of Rust, C, Perl, Scala, and C# utilities.
Features include and are:

    USB Caching & Rufus Integration:
    Cache files to USB drives and leverage Rufus for creating bootable USBs via a native C bridge.

    OneDrive Personal Sync:
    Mirror and cache files to OneDrive Personal folders using C utilities for direct filesystem access.

    File Explorer Integration & Copy:
    Move, cache, and organize files using a flexible Perl script for robust file operations.

    Download Caching & Malware Scanning:
    A Scala tool monitors your Downloads folder, automatically caches new files (to a secure vault), and invokes a C-based malware detector for safety.

    User Privacy Enforcer:
    A Windows C# tool ensures only the current user can access sensitive cached data using NTFS permissions.

Architecture

    Rust: Orchestrator and CLI entry point.
    C: Bridge for Rufus and OneDrive integration, as well as malicious file scanning.
    Perl: File copy and explorer utilities.
    Scala: Download monitoring and automated scanner invoker.
    C#: NTFS privacy enforcement for your cache.

Getting Started
Prerequisites

    Windows 10/11
    Rust
    Perl
    .NET 6+ SDK
    Scala
    Rufus (add to PATH)
    GCC for Windows or MinGW

Build & Usage
1. Build C Utilities

gcc -o filesafe/malicious_detector src/malicious_detector.c
gcc -o rufus_usb src/rufus_usb.c
gcc -o onedrive_sync src/onedrive_sync.c

2. Build C# Privacy Tool

cd src
dotnet build UserPrivacyEnforcer.csproj
cd ..

3. Run Rust Orchestrator

cargo run -- <usb|onedrive|explorer>

4. Run Scala Download Cache

scalac src/DownloadCache.scala
scala -cp src DownloadCache

5. Use Perl File Copy

perl src/file_copy.pl <source> <destination>

6. Enforce Privacy on a Folder/File

dotnet run --project src/UserPrivacyEnforcer.csproj "C:\path\to\your\data"

Security & Privacy

    All cached files can be scanned for malicious content before storage.
    NTFS permissions restrict cached data to the current Windows user only.
    No data is ever sent to third parties or the cloud unless explicitly using OneDrive sync.

Contributing

Pull requests and feature suggestions are warmly welcome! Please open an issue or fork the repo and submit a PR.
License
MIT License. See LICENSE for details.

How to use C security utillity:

    Open a terminal or PowerShell in the src directory.
    Run:
    Code

dotnet build

This will produce an executable in the bin/Debug/net6.0/ directory.
To protect a file or folder, run:
Code

dotnet run --project UserPrivacyEnforcer.csproj "C:\path\to\your\data"

Or run the .exe directly:
Code

    bin\Debug\net6.0\UserPrivacyEnforcer.exe "C:\path\to\your\data"

Let me know if you need any additional customization or want to integrate this step into your main workflow!
