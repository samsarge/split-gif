use core::panic;

use std::path::Path;

use image::codecs::gif::GifDecoder;

use image::{Frame, AnimationDecoder, ImageFormat, DynamicImage};
use std::fs::File;
use std::io::BufReader;


use clap::Parser;

#[derive(clap::ValueEnum, Clone, Debug)]
enum OutputType {
    Png,
    Jpg
}


#[derive(Parser, Debug)]
#[command(version = "1", about = "Split GIF into frames", long_about = "Split a GIF file into frames and save them as images. Allows different file type ouputs")]
struct Args {
    /// Input GIF path
    #[arg(short, long)]
    input_path: String,

    /// Output file type of the frames.
    #[arg(short, long)]
    output_type: OutputType,
}

fn main() {
    let args = Args::parse();


    let gif_path = args.input_path;
    let output_type = args.output_type;

    let gif_path = Path::new(&gif_path);

    // Decode a gif into frames
    let frames = read_into_frames(gif_path);

    let mut frame_count = 0;

    frames.iter().for_each(|frame|{
        save_frame(frame.to_owned(), frame_count, output_type.to_owned());
        frame_count += 1;
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
    

    frames.collect_frames().expect("error decoding gif")
}

fn save_frame(frame: Frame, frame_count: i32, output_type: OutputType) {

    let img_buffer = frame.into_buffer();
    let img_buffer = DynamicImage::ImageRgba8(img_buffer);

    match output_type {
        OutputType::Jpg => {
            let path = format!("frame_{}.jpg", frame_count);
            let path = Path::new(&path);
            let img_buffer = img_buffer.into_rgb8();
            img_buffer.save_with_format(path, ImageFormat::Png).unwrap_or_else(|err|{
                eprintln!("{}", err)
            });
        },
        OutputType::Png => {
            let path = format!("frame_{}.png", frame_count);
            let path = Path::new(&path);
            img_buffer.save_with_format(path, ImageFormat::Png).unwrap_or_else(|err|{
                eprintln!("{}", err)
            });
        }
    }

}
