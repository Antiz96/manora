//! Download man page from https://manned.org
//! and convert it to PDF (in case it cannot be found locally)

use reqwest::blocking::get;
use std::fs;
use std::io::{self, Error, ErrorKind, Write};
use std::path::Path;
use std::process::{Command, Stdio};

pub fn download_man_page(man_page: &str, cachedir: &Path) -> io::Result<()> {
    // Try to download raw man page
    let url = format!("https://manned.org/raw/{man_page}");
    let raw_man_page = get(&url)
        .map_err(Error::other)?
        .text()
        .map_err(Error::other)?;

    // Check if the man page was found
    if raw_man_page.contains("the page you were looking for doesn't exist.") {
        return Err(Error::new(
            ErrorKind::NotFound,
            format!("No manual entry found on https://manned.org for {man_page}"),
        ));
    }

    // Convert raw_man_page to PDF
    let mut conversion = Command::new("groff")
        .args(["-mandoc", "-Tpdf"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    conversion
        .stdin
        .take()
        .ok_or_else(|| Error::other("Failed to access groff stdin"))?
        .write_all(raw_man_page.as_bytes())?;

    let conversion_output = conversion.wait_with_output()?;

    if !conversion_output.status.success() {
        return Err(Error::other(
            String::from_utf8_lossy(&conversion_output.stderr).to_string(),
        ));
    }

    // Save converted man page in cache directory
    let dest_file_path = cachedir.join(format!("{man_page}.pdf"));
    fs::write(dest_file_path, conversion_output.stdout)?;

    Ok(())
}
