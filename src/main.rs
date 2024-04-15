use clap::Parser;
use image::codecs::gif::{GifEncoder, Repeat};
use image::{Delay, Frame, ImageBuffer, Rgba};
use std::fs::File;
use std::path::PathBuf;

use cfrs::{CFRBuffer, CFRColor, CommandExecutor};

#[derive(Parser, Debug)]
struct Cli {
    #[clap(long, default_value = "256")]
    width: u32,
    #[clap(long, default_value = "256")]
    height: u32,
    #[clap(short, long, default_value = "black")]
    background: CFRColor,
    #[clap(long, default_value = "100")]
    interval: u32,
    output: PathBuf,
    command: String,
}

fn main() {
    let args = Cli::parse();

    let extension = args
        .output
        .extension()
        .and_then(std::ffi::OsStr::to_str)
        .unwrap_or("");
    let animation = extension == "gif";

    let mut buffer = CFRBuffer::new(args.width, args.height);
    buffer.data.iter_mut().for_each(|c| *c = args.background);

    let mut time = 0;
    let mut frames = Vec::new();

    let mut executor = CommandExecutor::new(args.command, &mut buffer);
    while let Ok((sleep, buf)) = executor.step() {
        if sleep && animation {
            time += 20;
            if time >= args.interval {
                time -= args.interval;
                let img = buf.to_rgba_image();
                frames.push(img);
            }
        }
    }

    if animation {
        save_gif_animation(&frames, &args.output, args.interval);
    } else {
        if extension == "jpg" {
            let img = buffer.to_rgb_image();
            img.save(args.output).expect("Failed to save image");
        } else {
            let img = buffer.to_rgba_image();
            img.save(args.output).expect("Failed to save image");
        }
    }
}

fn save_gif_animation(frames: &Vec<ImageBuffer<Rgba<u8>, Vec<u8>>>, path: &PathBuf, interval: u32) {
    let mut file = File::create(path).unwrap();
    let mut encoder = GifEncoder::new(&mut file);
    encoder.set_repeat(Repeat::Infinite).unwrap();
    for frame_data in frames {
        let frame = Frame::from_parts(
            frame_data.clone(),
            0,
            0,
            Delay::from_numer_denom_ms(interval, 1),
        );
        encoder.encode_frame(frame).unwrap();
    }
}
