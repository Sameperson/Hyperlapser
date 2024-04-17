pub struct Config {
    pub download_url: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
            download_url: "https://www.youtube.com/watch?v=asyT-N1Ip70".to_string(),
        }
    }
}
