//! Save man page to a PDF file

use std::path::Path;
use std::process::Command;

// Save man pas as a PDF
pub fn save_man_page(man_page: &str, file: &Path) -> std::io::Result<()> {
    // Convert man page as a PDF
    let output = Command::new("man").args(["-Tpdf", man_page]).output()?;

    if !output.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }

    // Save the converted man page as a PDF file
    std::fs::write(file, output.stdout)?;

    Ok(())
}

// Save downloaded man page to file
pub fn save_downloaded_man_page(
    man_page: &str,
    cachedir: &Path,
    file: &Path,
) -> std::io::Result<()> {
    // Set path to downloaded man page
    let pdf_path = cachedir.join(format!("{}.pdf", man_page));

    // Move downloaded man page to file
    // Using "copy" then "remove_file" functions instead of "rename" function
    // as it requires the source and destination to be on the same filesystem
    // while cachedir is in /tmp (which is likely tmpfs)
    std::fs::copy(&pdf_path, file)?;
    std::fs::remove_file(pdf_path)?;

    Ok(())
}
