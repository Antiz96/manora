//! Convert the man page as a PDF and open it

use std::os::unix::process::CommandExt;
use std::path::Path;
use std::process::{Command, Stdio};

// Open man page as a PDF
pub fn open_man_page(man_page: &str, cachedir: &Path) -> std::io::Result<()> {
    // Convert man page as a PDF
    let conversion = Command::new("man").args(["-Tpdf", man_page]).output()?;

    if !conversion.status.success() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            String::from_utf8_lossy(&conversion.stderr).to_string(),
        ));
    }

    // Save the converted man page as a PDF file in the cachedir
    let dest_file_path = cachedir.join(format!("{}.pdf", man_page));
    std::fs::write(&dest_file_path, conversion.stdout)?;

    // Open in PDF reader
    let pdf_reader = get_pdf_reader().map_err(std::io::Error::other)?;
    let mut open = Command::new(&pdf_reader);

    open.arg(&dest_file_path)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());

    // Detach PDF reader process from terminal session
    unsafe {
        open.pre_exec(|| {
            nix::unistd::setsid()
                .map(|_| ())
                .map_err(std::io::Error::other)
        });
    }

    open.spawn()?;

    Ok(())
}

// Open downloaded man page as a PDF
pub fn open_downloaded_man_page(man_page: &str, cachedir: &Path) -> std::io::Result<()> {
    // Set path to downloaded man page
    let dest_file_path = cachedir.join(format!("{}.pdf", man_page));

    // Open in PDF reader
    let pdf_reader = get_pdf_reader().map_err(std::io::Error::other)?;
    let mut open = Command::new(&pdf_reader);

    open.arg(&dest_file_path)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null());

    // Detach PDF reader process from terminal session
    unsafe {
        open.pre_exec(|| {
            nix::unistd::setsid()
                .map(|_| ())
                .map_err(std::io::Error::other)
        });
    }

    open.spawn()?;

    Ok(())
}

// Get PDF reader, return error if none can be found
fn get_pdf_reader() -> Result<String, String> {
    // Check if a default PDF reader is configured in XDG
    let xdg_pdf_reader = Command::new("xdg-mime")
        .args(["query", "default", "application/pdf"])
        .output();

    if let Ok(output) = xdg_pdf_reader
        && !output.stdout.is_empty()
    {
        return Ok("xdg-open".into());
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

    // Return an error if none can be found
    Err(
        "No PDF reader defined in XDG Mime Application and zathura (fallback option) isn't installed"
            .to_string(),
    )
}
