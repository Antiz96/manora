//! Save man page to a PDF file

use std::fs;
use std::io::{self, Error, ErrorKind};
use std::path::Path;
use std::process::Command;

// Save man pas as a PDF
pub fn save_man_page(man_page: &str, dest_file_path: &Path) -> io::Result<()> {
    // Convert man page as a PDF
    let conversion = Command::new("man").args(["-Tpdf", man_page]).output()?;

    if !conversion.status.success() {
        return Err(Error::new(
            ErrorKind::NotFound,
            String::from_utf8_lossy(&conversion.stderr).to_string(),
        ));
    }

    // Save the converted man page as a PDF file
    fs::write(dest_file_path, conversion.stdout)?;

    Ok(())
}

// Save downloaded man page to file
pub fn save_downloaded_man_page(
    man_page: &str,
    cachedir: &Path,
    dest_file_path: &Path,
) -> io::Result<()> {
    // Set path to downloaded man page
    let dl_man_page_file_path = cachedir.join(format!("{}.pdf", man_page));

    // Move downloaded man page to the destination file
    // Using "copy" then "remove_file" functions instead of "rename" function
    // as the latter requires the source and destination to be on the same filesystem
    // while cachedir is in /tmp (which is likely tmpfs)
    fs::copy(&dl_man_page_file_path, dest_file_path)?;
    fs::remove_file(dl_man_page_file_path)?;

    Ok(())
}
