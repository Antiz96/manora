//! Convert and save selected man page as a PDF file

use std::fs;
use std::io::{self, Error, Write};
use std::path::Path;
use std::process::{Command, Stdio};

pub fn convert_man_page(man_page: &str, dest_file_path: &Path) -> io::Result<()> {
    // Convert man page as a PDF
    let conversion = Command::new("man").args(["-Tpdf", man_page]).output()?;

    if !conversion.status.success() {
        return Err(Error::other(
            String::from_utf8_lossy(&conversion.stderr).to_string(),
        ));
    }

    // Write the converted man page on the filesystem
    fs::write(dest_file_path, conversion.stdout)?;

    Ok(())
}

pub fn convert_downloaded_man_page(dl_man_page: &str, dest_file_path: &Path) -> io::Result<()> {
    // Convert downloaded man page to PDF
    let mut conversion = Command::new("groff")
        .args(["-mandoc", "-Tpdf"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    conversion
        .stdin
        .take()
        .ok_or_else(|| Error::other("Failed to access groff stdin"))?
        .write_all(dl_man_page.as_bytes())?;

    let conversion_output = conversion.wait_with_output()?;

    if !conversion_output.status.success() {
        return Err(Error::other(
            String::from_utf8_lossy(&conversion_output.stderr).to_string(),
        ));
    }

    // Write the converted man page on the filesystem
    fs::write(dest_file_path, conversion_output.stdout)?;

    Ok(())
}
