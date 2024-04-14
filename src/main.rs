use clap::Parser;
use image::codecs::gif::{GifEncoder, Repeat};
use image::{Delay, Frame, ImageBuffer, Rgb, Rgba};
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

    let mut executor = CommandExecutor::new(args.command, &mut buffer);
    let mut time = 0;

    let mut frames = Vec::new();

    while let Ok((sleep, buf)) = executor.step() {
        if sleep && animation {
            time += 20;
            if time >= args.interval {
                time -= args.interval;
                let img = create_image(buf, args.width, args.height);
                frames.push(img);
            }
        }
    }

    if animation {
        save_gif_animation(&frames, &args.output, args.interval);
    } else {
        let img = create_image(&buffer, args.width, args.height);

        if extension == "jpg" {
            let img = ImageBuffer::from_fn(args.width, args.height, |x, y| {
                let color = img.get_pixel(x, y);
                let color = color.0;
                Rgb::<u8>([color[0], color[1], color[2]])
            });
            img.save(args.output).expect("Failed to save image");
        } else {
            img.save(args.output).expect("Failed to save image");
        }
    }
}

fn create_image(buffer: &CFRBuffer, width: u32, height: u32) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    ImageBuffer::from_fn(width, height, |x, y| {
        let color = buffer.data[(y * width + x) as usize];
        match color {
            CFRColor::Black => Rgba::<u8>([0, 0, 0, 255]),
            CFRColor::White => Rgba::<u8>([255, 255, 255, 255]),
            CFRColor::Red => Rgba::<u8>([255, 0, 0, 255]),
            CFRColor::Green => Rgba::<u8>([0, 255, 0, 255]),
            CFRColor::Blue => Rgba::<u8>([0, 0, 255, 255]),
            CFRColor::Yellow => Rgba::<u8>([255, 255, 0, 255]),
            CFRColor::Cyan => Rgba::<u8>([0, 255, 255, 255]),
            CFRColor::Magenta => Rgba::<u8>([255, 0, 255, 255]),
        }
    })
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
