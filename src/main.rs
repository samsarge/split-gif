use rayon::prelude::*;

use std::path::Path;

use std::fmt;
use image::codecs::gif::GifDecoder;

use image::{AnimationDecoder, DynamicImage, Frame, ImageFormat};
use std::fs::File;
use std::io::BufReader;

use clap::Parser;

#[derive(clap::ValueEnum, Clone, Copy, Debug)]
enum OutputType {
    Png,
    Jpg
}

impl OutputType {
    fn to_image_format(self) -> ImageFormat {
        match self {
            OutputType::Png => ImageFormat::Png,
            OutputType::Jpg => ImageFormat::Jpeg
        }
    }
}

impl fmt::Display for OutputType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self {
            OutputType::Png => write!(f, "png"),
            OutputType::Jpg => write!(f, "jpg")
        }
    }
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

    /// Optional argument to shorten the output to a certain amount of frames.
    /// For example; setting a 20 frame gif to 10 frames will save every 2nd frame
    #[arg(short, long)]
    max_frames: Option<usize>
}

fn main() {
    let args = Args::parse();

    let gif_path = Path::new(&args.input_path);

    let output_type = args.output_type;

    let mut frames = read_into_frames(gif_path);

    if args.max_frames.is_some()  {
        frames = reduce_frames(frames, args.max_frames.unwrap());
    }

    frames.par_iter().enumerate().for_each(|(index, frame)|{
        save_frame(frame.to_owned(), index, output_type.to_owned());
    })
}

fn reduce_frames(frames: Vec<Frame>, max: usize) -> Vec<Frame> {
    let length = frames.len();
    let step = (length as f64 / max as f64).floor() as usize;
    let mut new_frames: Vec<Frame> = Vec::with_capacity(max);

    frames.iter().step_by(step).enumerate().for_each(|(i, frame)|{
        // index of the new stepped iter
        // step_by always includes the first item
        // which we want, sometimes it might step over the max depending
        if i < max {
            new_frames.push(frame.clone());
        }
    });

    new_frames
}

fn read_into_frames(path: &Path) -> Vec<Frame> {
    let file = File::open(path).expect("No file at input path");
    let buffer = BufReader::new(file);
    let decoder = GifDecoder::new(buffer).expect("Error decoding gif");
    let frames = decoder.into_frames();

    frames.collect_frames().expect("Error decoding gif")
}

fn save_frame(frame: Frame, frame_count: usize, output_type: OutputType) {
    // frames output is rgba by default
    let img_buffer = frame.into_buffer();

    let path = format!("frame_{}.{}", frame_count, output_type);
    let path = Path::new(&path);

    let err_handle = {|err: image::ImageError| eprintln!("{}", err) };

    match output_type {
        OutputType::Jpg => {
            // jpg doesnt support opacity so has to be rgb instead of rgba
            let img_buffer = DynamicImage::ImageRgba8(img_buffer);
            let img_buffer = img_buffer.into_rgb8();
            img_buffer.save_with_format(path, output_type.to_image_format()).unwrap_or_else(err_handle);
        },
        OutputType::Png => {
            img_buffer.save_with_format(path, output_type.to_image_format()).unwrap_or_else(err_handle);
        }
    }

}
