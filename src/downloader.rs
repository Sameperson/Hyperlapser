use rustube::{VideoFetcher, Result, VideoDescrambler};
use rustube::url::Url;

pub async fn download_video(url: &str) -> Result<VideoDescrambler> {
    let url_obj = match url.parse::<Url>() {
        Ok(url) => url,
        Err(e) => return Err(e.into()),
    };

    let video_fetcher = VideoFetcher::from_url(&url_obj)?;
    let video = video_fetcher.fetch().await?;
    Ok(video)
}
