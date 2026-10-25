//! Create runtime directory (if it doesn't exist):

use anyhow::Context;
use nix::unistd::Uid;
use std::ffi::OsString;
use std::os::unix::fs::MetadataExt;
use std::path::PathBuf;
use std::{env, fs};

pub fn create_run_dir() -> anyhow::Result<PathBuf> {
    let run_dir = dirs::runtime_dir()
        .map(|dir| dir.join("manora"))
        .or_else(|| {
            let tmp_dir = env::var_os("TMPDIR")
                .filter(|path| !path.is_empty())
                .unwrap_or_else(|| OsString::from("/tmp"));

            Some(PathBuf::from(tmp_dir).join(format!("manora-{}", Uid::effective())))
        })
        .context("Failed to determine runtime directory")?;

    fs::create_dir_all(&run_dir).with_context(|| {
        format!(
            "Failed to create the {} runtime directory",
            run_dir.display()
        )
    })?;

    // Best effort permissions check to ensure the runtime dir wasn't
    // pre-created in a malicious way
    //
    // This is mostly relevant in case we use the $TMPDIR or /tmp fallback,
    // as $XDG_RUNTIME_DIR is expected to have secure permissions already
    //
    // This check ensures that the runtime dir is:
    // - not a symlink
    // - owned by the current user
    // - only writable by the current user
    let run_dir_metadata = fs::symlink_metadata(&run_dir).with_context(|| {
        format!(
            "Failed to get metadata of the {} runtime directory",
            run_dir.display()
        )
    })?;

    anyhow::ensure!(
        run_dir_metadata.is_dir()
            && run_dir_metadata.uid() == Uid::effective().as_raw()
            && run_dir_metadata.mode() & 0o022 == 0,
        "Invalid state or permissions for the {} runtime directory",
        run_dir.display()
    );

    Ok(run_dir)
}
