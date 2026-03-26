use anyhow::Result;
use std::path::Path;
use tokio::fs;

use crate::{app::RepoItem, github_api::fetch_contents};

pub async fn download_file(url: &str, dest_path: &str) -> Result<()> {
    let response = reqwest::get(url).await?;
    let bytes = response.bytes().await?;

    if let Some(parent) = Path::new(dest_path).parent() {
        fs::create_dir_all(parent).await?;
    }

    fs::write(dest_path, bytes).await?;
    Ok(())
}

pub async fn download_recursive(owner: &str, repo: &str, item: RepoItem) -> Result<()> {
    if item.is_dir {
        let folder_items = fetch_contents(owner, repo, &item.path).await?;

        for sub_item in folder_items {
            Box::pin(download_recursive(owner, repo, sub_item)).await?;
        }
    } else if let Some(url) = item.download_url {
        let dest = format!("{}/{}", repo, item.path);
        download_file(&url, &dest).await?;
    }

    Ok(())
}
