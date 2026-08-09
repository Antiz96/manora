//! Create cache directory (if it doesn't exist):
//! ${TMPDIR:-/tmp}/manora-${UID}

use anyhow::Context;
use nix::unistd::Uid;
use std::ffi::OsString;
use std::path::PathBuf;
use std::{env, fs};

pub fn create_cachedir() -> anyhow::Result<PathBuf> {
    let uid = Uid::effective();

    let tmpdir = env::var_os("TMPDIR")
        .filter(|path| !path.is_empty())
        .unwrap_or_else(|| OsString::from("/tmp"));

    let cachedir = PathBuf::from(tmpdir).join(format!("manora-{uid}"));

    fs::create_dir_all(&cachedir).with_context(|| {
        format!(
            "Unable to create the {} cache directory",
            cachedir.display()
        )
    })?;

    Ok(cachedir)
}
