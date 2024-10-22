use std::error::Error;
use std::fs;
use std::path::Path;

pub async fn download_file(url: &str, dest: &Path) -> Result<(), Box<dyn Error>> {
    let response = reqwest::get(url).await?;
    let content = response.bytes().await?;
    fs::write(dest, content).expect("Failed to write to disk.");

    Ok(())
}