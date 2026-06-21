//! Save man page to a PDF file

use std::path::Path;
use std::process::Command;

pub fn save_man_page(man_page: &str, file: &Path) -> std::io::Result<()> {
    // Convert man page as a PDF
    let output = Command::new("man").args(["-Tpdf", man_page]).output()?;

    if !output.status.success() {
        return Err(std::io::Error::other(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }

    // Save the converted man page as a PDF file
    std::fs::write(file, output.stdout)?;

    Ok(())
}
