use core::panic;
use std::env;
use std::path::Path;

use image::codecs::gif::GifDecoder;

use image::{Frame, AnimationDecoder, ImageFormat, DynamicImage};
use std::fs::File;
use std::io::BufReader;

fn main() {
    let args: Vec<String> = env::args().collect();


    let gif_path = args.get(1).unwrap();
    let gif_path = Path::new(gif_path);

    // Decode a gif into frames
    let frames = read_into_frames(gif_path);


    let mut frame_count = 0;

    frames.iter().for_each(|frame|{
        let path = format!("frame_{}.png", frame_count);
        let path = Path::new(&path);
        frame_count += 1;
        save_frame(frame.to_owned(), path);
    })
}

fn read_into_frames(path: &Path) -> Vec<Frame> {
    let file = File::open(path).unwrap_or_else(|_|{
        panic!("No file");
    });

    let buffer = BufReader::new(file);

    let decoder = GifDecoder::new(buffer).unwrap_or_else(|_|{
        panic!("Not a gif bro");
    });

    let frames = decoder.into_frames();
    let frames = frames.collect_frames().expect("error decoding gif");

    frames
}

fn save_frame(frame: Frame, path: &Path) {
    let img_buffer = frame.into_buffer();
    let img_buffer = DynamicImage::ImageRgba8(img_buffer);

    img_buffer.save_with_format(path, ImageFormat::Png).unwrap_or_else(|err|{
        println!("{}", err)
    });
}
