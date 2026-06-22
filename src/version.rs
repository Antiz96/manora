//! Show name and version
//! Info retrieved directly from Cargo 

use std::env;

pub fn show_version() {
    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
}
