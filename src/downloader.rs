use std::path::PathBuf;
use rustube::{VideoFetcher, Result, VideoDescrambler};
use rustube::url::Url;

pub async fn download_video(url: &str) -> Result<PathBuf> {
    let path_to_video = rustube::download_best_quality(url).await?;

    Ok(path_to_video)
}
