//! Convert the man page as a PDF and open it

use std::path::Path;
use std::process::{Command, Stdio};

pub fn open_man_page(man_page: &str, workdir: &Path) -> std::io::Result<()> {
    // Convert man page as a PDF
    let output = Command::new("man").args(["-Tpdf", man_page]).output()?;

    if !output.status.success() {
        return Err(std::io::Error::other(
            String::from_utf8_lossy(&output.stderr).to_string(),
        ));
    }

    // Save the converted man page as a PDF file in the temporary workdir
    let pdf_path = workdir.join(format!("{}.pdf", man_page));
    std::fs::write(&pdf_path, output.stdout)?;

    // Open in PDF reader
    let pdf_reader = get_pdf_reader().map_err(std::io::Error::other)?;

    Command::new(&pdf_reader)
        .arg(&pdf_path)
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()?;

    // Slightly inelegant workaround:
    // Add a small delay before returning to give the PDF reader time
    // to open the file before the temporary working directory gets deleted.
    std::thread::sleep(std::time::Duration::from_millis(250));

    Ok(())
}

// Get PDF reader, return error if none can be found
fn get_pdf_reader() -> Result<String, String> {
    // Check if a default PDF reader is configured in XDG
    let xdg_pdf_reader = Command::new("xdg-mime")
        .args(["query", "default", "application/pdf"])
        .output();

    if let Ok(output) = xdg_pdf_reader {
        if !output.stdout.is_empty() {
            return Ok("xdg-open".into());
        }
    }

    // If no default PDF reader is configured in XDG, check if zathura is installed as a fallback
    if Command::new("zathura")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok_and(|status| status.success())
    {
        return Ok("zathura".to_string());
    }

    Err(
        "No PDF reader defined in XDG Mime Application and zathura (fallback option) isn't installed"
            .to_string(),
    )
}
