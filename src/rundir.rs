//! Create runtime directory (if it doesn't exist)

use anyhow::Context;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

pub fn create_run_dir() -> anyhow::Result<PathBuf> {
    let run_dir = dirs::runtime_dir()
        .map(|dir| dir.join("manora"))
        .map_or_else(
            || {
                tempfile::Builder::new()
                    .prefix("manora.")
                    .disable_cleanup(true)
                    .permissions(fs::Permissions::from_mode(0o700))
                    .tempdir()
                    .context("Failed to create a temporary runtime directory")
                    .map(|dir| dir.path().to_path_buf())
            },
            Ok,
        )?;

    fs::create_dir_all(&run_dir).with_context(|| {
        format!(
            "Failed to create the {} runtime directory",
            run_dir.display()
        )
    })?;

    Ok(run_dir)
}
