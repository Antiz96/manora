//! Open man page in PDF reader

use anyhow::Context;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use which::which;

// Get PDF reader, then open PDF man page
pub fn open_pdf_man_page(path: &Path) -> anyhow::Result<()> {
    // Check if a default PDF reader is configured in XDG, fallback to zathura otherwise
    let xdg_mime_output = Command::new("xdg-mime")
        .args(["query", "default", "application/pdf"])
        .output()
        .context("Failed to run xdg-mime")?;

    let pdf_reader = if xdg_mime_output.status.success() && !xdg_mime_output.stdout.is_empty() {
        PathBuf::from("xdg-open")
    } else {
        which("zathura").context("No PDF reader defined in XDG Mime Application and zathura (fallback option) cannot be found")?
    };

    // Open the man page in PDF reader
    Command::new("setsid")
        .arg(&pdf_reader)
        .arg(path)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .context("Failed to run setsid")?;

    Ok(())
}
