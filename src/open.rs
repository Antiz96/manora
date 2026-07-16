//! Open man page in PDF reader

use std::io::{self, Error};
use std::path::Path;
use std::process::{Command, Stdio};

// Get PDF reader, then open PDF man page
pub fn open_pdf_man_page(path: &Path) -> io::Result<()> {
    // Check if a default PDF reader is configured in XDG, fallback to zathura otherwise
    let pdf_reader = if Command::new("xdg-mime")
        .args(["query", "default", "application/pdf"])
        .output()
        .is_ok_and(|output| !output.stdout.is_empty())
    {
        "xdg-open".into()
    } else if Command::new("zathura")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok_and(|status| status.success())
    {
        "zathura".to_string()
    } else {
        return Err(Error::other(
            "No PDF reader defined in XDG Mime Application and zathura (fallback option) isn't installed",
        ));
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
