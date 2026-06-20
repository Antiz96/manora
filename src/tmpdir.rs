//! Create temporary working directory (automatically deleted once the process exits)
//! ${TMPDIR:-/tmp}/manora-XXXXXX

use std::env;
use tempfile::{Builder, TempDir};

pub fn create_tmpdir() -> std::io::Result<TempDir> {
    let tmpdir = env::var("TMPDIR")
        .ok()
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "/tmp".to_string());

    Builder::new().prefix("manora-").tempdir_in(tmpdir)
}
