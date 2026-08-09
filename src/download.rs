//! Download man page from https://manned.org

use anyhow::{Context, anyhow};
use reqwest::StatusCode;
use reqwest::blocking::get;

pub fn download_man_page(man_page: &str) -> anyhow::Result<String> {
    // Try to download man page from https://manned.org
    let url = format!("https://manned.org/raw/{man_page}");

    let response = get(&url).context("Failed to request the man page from https://manned.org")?;

    // HTTP 404
    let dl_man_page = if response.status() == StatusCode::NOT_FOUND {
        return Err(anyhow!(
            "No manual entry for {man_page} on https://manned.org"
        ));
    // Any other HTTP error code
    } else {
        response
            .error_for_status()
            .context("Failed to download the man page from https://manned.org")?
            .text()
            .context("Failed to read the downloaded man page")?
    };

    Ok(dl_man_page)
}
