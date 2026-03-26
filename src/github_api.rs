use crate::app::RepoItem;
use anyhow::{Result, anyhow};

pub fn parse_github_url(url: &str) -> Result<(String, String)> {
    let parts: Vec<&str> = url.trim_end_matches('/').split('/').collect();
    if parts.len() < 2 {
        return Err(anyhow!("Invalid Github URL format"));
    }
    let repo_name = parts[parts.len() - 1].trim_end_matches(".git").to_string();
    let owner_name = parts[parts.len() - 2].to_string();

    Ok((owner_name, repo_name))
}

pub async fn fetch_contents(owner: &str, repo: &str, path: &str) -> Result<Vec<RepoItem>> {
    let result = octocrab::instance()
        .repos(owner, repo)
        .get_content()
        .path(path)
        .send()
        .await?;

    let items = result
        .items
        .into_iter()
        .map(|item| RepoItem {
            name: item.name,
            path: item.path,
            is_dir: item.r#type == "dir",
            download_url: item.download_url.map(|u| u.to_string()),
        })
        .collect();

    Ok(items)
}
