//! Download man page from https://manned.org

use reqwest::blocking::get;
use std::io::{self, Error};

pub fn download_man_page(man_page: &str) -> io::Result<String> {
    // Try to download man page from https://manned.org
    let url = format!("https://manned.org/raw/{man_page}");
    let dl_man_page = get(&url)
        .map_err(Error::other)?
        .text()
        .map_err(Error::other)?;

    // Check if the man page was found
    if dl_man_page.contains("the page you were looking for doesn't exist.") {
        return Err(Error::other(format!(
            "No manual entry for {man_page} on https://manned.org"
        )));
    }

    Ok(dl_man_page)
}
