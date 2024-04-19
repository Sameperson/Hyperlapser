
mod video_processor;
mod config;
mod downloader;
mod server;
mod routes;
mod handlers;

#[tokio::main]
async fn main() {
    let config = config::Config::new();

    match downloader::download_video(&config.download_url).await {
        Ok(video) => {
            println!("Video downloaded successfully!");
            video_processor::process_video("path_to_downloaded_video.mp4");
        },
        Err(e) => println!("Failed to download video: {}", e),
    }
}