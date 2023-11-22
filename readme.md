# Clipper
Clipper is a simple command line tool for managing the Windows clipboard.

## Usage
```powershell
# Print the clipboard
clipper
# Set the clipboard to "asdf"
echo asdf | clipper
# Copy the contents of a file into the clipboard
clipper readme.md
# Paste the clipboard into a file
clipper -o out.txt
# Copy files and folders like the Windows file explorer (lazy)
clipper -e foo.txt images
# Print which files are in the clipboard, if any
clipper -l
# Paste the copied files inside D:/some_directory
clipper -o D:/some_directory
# Clear the clipboard
clipper -x
```

## Building The Program
```powershell
cargo build --release
# The executable will be in target/release/clipper.exe
```

