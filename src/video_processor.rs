use ffmpeg_next::format::{input, Pixel};
use ffmpeg_next::codec::{self, decoder};
use ffmpeg_next::util::frame::video::Video as FrameVideo;
use ffmpeg_next::software::scaling::{context::Context as ScaleContext, flag::Flags};
use std::path::Path;

pub fn process_video(video_path: &str) {
    // Initialize FFmpeg
    ffmpeg_next::init().unwrap();

    // Open the video file
    let mut ictx = match input(&Path::new(video_path)) {
        Ok(ctx) => ctx,
        Err(e) => {
            eprintln!("Error opening video file: {}", e);
            return;
        }
    };

    let input = match ictx.streams().best(ffmpeg_next::media::Type::Video) {
        Some(stream) => stream,
        None => {
            eprintln!("No video stream found in file");
            return;
        }
    };

    // Create a decoder from the codec parameters
    let codec_params = input.parameters();
    let mut codec_context = match codec::Context::from_parameters(codec_params) {
        Ok(ctx) => ctx,
        Err(e) => {
            eprintln!("Error creating codec context: {}", e);
            return;
        }
    };

    let mut decoder = match codec_context.decoder().video() {
        Ok(dec) => dec,
        Err(e) => {
            eprintln!("Error creating video decoder: {}", e);
            return;
        }
    };

    // Set up a scaler to convert images to RGB24, which is easier to work with
    let mut scaler = match ScaleContext::get(
        decoder.format(),
        decoder.width(),
        decoder.height(),
        Pixel::RGB24,
        decoder.width(),
        decoder.height(),
        Flags::BILINEAR,
    ) {
        Ok(scale) => scale,
        Err(e) => {
            eprintln!("Error setting up scaler: {}", e);
            return;
        }
    };

    // Processing frames
    let mut frame = FrameVideo::empty();
    while let Ok(_) = decoder.receive_frame(&mut frame) {
        let mut rgb_frame = FrameVideo::empty();
        scaler.run(&frame, &mut rgb_frame).unwrap();
        // Here, you could modify frames or analyze them
        println!("Processed a frame");
    }

    // Error handling and cleanup would go here
}
