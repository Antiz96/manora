//! Convert and save selected man page as a PDF file

use anyhow::Error as AnyhowError;
use anyhow::{Context, anyhow};
use std::fs;
use std::io::{Error, ErrorKind, Write};
use std::path::Path;
use std::process::{Command, Stdio};

pub fn convert_man_page(man_page: &str, dest_file_path: &Path) -> anyhow::Result<()> {
    // Convert man page as a PDF
    let conversion = Command::new("man")
        .args(["-Tpdf", man_page])
        .output()
        .context("Failed to run man")?;

    if !conversion.status.success() {
        // man exits with error code 16 if the man page isn't found
        if conversion.status.code() == Some(16) {
            return Err(AnyhowError::from(Error::new(
                ErrorKind::NotFound,
                String::from_utf8_lossy(&conversion.stderr).into_owned(),
            )));
        } else {
            return Err(anyhow!("{}", String::from_utf8_lossy(&conversion.stderr))
                .context("Failed to convert the man page as a PDF"));
        }
    }

    // Write the converted man page on the filesystem
    fs::write(dest_file_path, conversion.stdout)
        .with_context(|| format!("Failed to write the {} file", dest_file_path.display()))?;

    Ok(())
}

pub fn convert_downloaded_man_page(dl_man_page: &str, dest_file_path: &Path) -> anyhow::Result<()> {
    // Convert downloaded man page to PDF
    let mut conversion = Command::new("groff")
        .args(["-mandoc", "-Tpdf"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .context("Failed to run groff")?;

    conversion
        .stdin
        .take()
        .context("Failed to access groff stdin")?
        .write_all(dl_man_page.as_bytes())
        .context("Failed to redirect downloaded man page into groff stdin")?;

    let conversion_output = conversion
        .wait_with_output()
        .context("Failed to collect groff output")?;

    if !conversion_output.status.success() {
        return Err(
            anyhow!("{}", String::from_utf8_lossy(&conversion_output.stderr))
                .context("Failed to convert the downloaded man page as a PDF"),
        );
    }

    // Write the converted man page on the filesystem
    fs::write(dest_file_path, conversion_output.stdout)
        .with_context(|| format!("Failed to write the {} file", dest_file_path.display()))?;

    Ok(())
}
