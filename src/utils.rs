use std::fs;
use std::path::Path;
use anyhow::{Result, Context};

pub async fn download_file(url: &str, dest: &Path) -> Result<()> {
    let response = reqwest::get(url)
    .await
    .context(format!("Failed to send GET request to {}", url))?;

    let content = response.bytes()
    .await
    .context("Failed to read response bytes")?;

    fs::write(dest, content).context(format!("Failed to write to disk: {:?}", dest))?;

    Ok(())
}