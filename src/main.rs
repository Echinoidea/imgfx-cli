use clap::{builder::styling::RgbColor, Parser};
use image::{GenericImage, GenericImageView, ImageBuffer, Pixel, Rgba, RgbaImage};
use std::process::Command;

#[derive(Parser, Debug)]
#[command(name = "img-mod")]
#[command(version = "1.0.0")]
#[command(about = "Bitwise operations and other stuff to images.", long_about = None)]
struct Args {
    #[arg(short, long)]
    input_path: String,

    #[arg(short, long)]
    output_path: String,

    #[arg(short, long)]
    function: String,

    #[arg(short, long)]
    color: String,
}

fn hex_to_rgb(hex: &str) -> Option<(u8, u8, u8)> {
    if hex.starts_with('#') && hex.len() == 7 {
        let r = u8::from_str_radix(&hex[1..3], 16).ok()?;
        let g = u8::from_str_radix(&hex[3..5], 16).ok()?;
        let b = u8::from_str_radix(&hex[5..7], 16).ok()?;
        Some((r, g, b))
    } else {
        None
    }
}

fn or(path: &str, color: RgbColor) -> RgbaImage {
    let img = image::open(path).expect("Failed to open image.");
    let (width, height) = img.dimensions();

    let mut output = ImageBuffer::new(width, height);

    for (x, y, pixel) in img.pixels() {
        let r = pixel[0] | color.r();
        let g = pixel[1] | color.g();
        let b = pixel[2] | color.b();
        let a = pixel[3];

        output.put_pixel(x, y, Rgba([r, g, b, a]));
    }

    output
}

fn and(path: &str, color: RgbColor) -> RgbaImage {
    let img = image::open(path).expect("Failed to open image.");
    let (width, height) = img.dimensions();

    let mut output = ImageBuffer::new(width, height);

    for (x, y, pixel) in img.pixels() {
        let r = pixel[0] & color.r();
        let g = pixel[1] & color.g();
        let b = pixel[2] & color.b();
        let a = pixel[3];

        output.put_pixel(x, y, Rgba([r, g, b, a]));
    }

    output
}

fn xor(path: &str, color: RgbColor) -> RgbaImage {
    let img = image::open(path).expect("Failed to open image.");
    let (width, height) = img.dimensions();

    let mut output = ImageBuffer::new(width, height);

    for (x, y, pixel) in img.pixels() {
        let r = pixel[0] ^ color.r();
        let g = pixel[1] ^ color.g();
        let b = pixel[2] ^ color.b();
        let a = pixel[3];

        output.put_pixel(x, y, Rgba([r, g, b, a]));
    }

    output
}

fn main() {
    let args = Args::parse();

    let in_path = args.input_path;
    let out_path = args.output_path;
    let operation = args.function;
    let color_arg = args.color;

    let rgb = hex_to_rgb(&color_arg).unwrap();

    let output: RgbaImage = match operation.as_str() {
        "or" => or(&in_path, RgbColor(rgb.0, rgb.1, rgb.2)),
        "and" => and(&in_path, RgbColor(rgb.0, rgb.1, rgb.2)),
        "xor" => xor(&in_path, RgbColor(rgb.0, rgb.1, rgb.2)),
        _ => panic!("Invalid operation"),
    };

    match out_path.as_str() {
        "" => output
            .save("output.bmp")
            .expect("Failed to save output image"),
        _ => output.save(out_path).expect("Failed to save output image"),
    }

    let _ = Command::new("nsxiv")
        .arg("output.bmp")
        .spawn()
        .expect("Failed to execute");
}
