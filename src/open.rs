//! Open man page in PDF reader

use std::io::{self, Error};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use which::which;

// Get PDF reader, then open PDF man page
pub fn open_pdf_man_page(path: &Path) -> io::Result<()> {
    // Check if a default PDF reader is configured in XDG, fallback to zathura otherwise
    let pdf_reader = if Command::new("xdg-mime")
        .args(["query", "default", "application/pdf"])
        .output()
        .is_ok_and(|output| !output.stdout.is_empty())
    {
        PathBuf::from("xdg-open")
    } else {
        which("zathura").map_err(|_| {
            Error::other("No PDF reader defined in XDG Mime Application and zathura (fallback option) isn't installed")
        })?
    };

    // Open the man page in PDF reader
    Command::new("setsid")
        .arg(&pdf_reader)
        .arg(path)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;

    Ok(())
}
