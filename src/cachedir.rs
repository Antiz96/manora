//! Create cache directory (if it doesn't exist):
//! ${TMPDIR:-/tmp}/manora-${UID}

use std::path::PathBuf;
use std::{env, fs};

pub fn create_cachedir() -> std::io::Result<PathBuf> {
    let uid = nix::unistd::Uid::effective();

    let tmpdir = env::var("TMPDIR")
        .ok()
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "/tmp".to_string());

    let cachedir = PathBuf::from(tmpdir).join(format!("manora-{uid}"));

    fs::create_dir_all(&cachedir)?;

    Ok(cachedir)
}
