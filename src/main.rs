use clap::{builder::styling::RgbColor, Parser};
use image::{EncodableLayout, GenericImageView, ImageBuffer, Rgba, RgbaImage};
use std::io::{self, BufRead, Write};

#[derive(Parser, Debug)]
#[command(name = "img-mod")]
#[command(version = "1.0.0")]
#[command(about = "Bitwise operations and other stuff to images.", long_about = None)]
struct Args {
    #[arg(short, long)]
    input_path: Option<String>,

    #[arg(short, long)]
    output_path: String,

    #[arg(short, long)]
    function: String,

    #[arg(short, long)]
    color: Option<String>,

    #[arg(short, long)]
    bit_shift: Option<u8>,
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

fn left(path: &str, bits: u8) -> RgbaImage {
    let img = image::open(path).expect("Failed to open image.");
    let (width, height) = img.dimensions();

    let mut output = ImageBuffer::new(width, height);

    for (x, y, pixel) in img.pixels() {
        let r = pixel[0] << bits;
        let g = pixel[1] << bits;
        let b = pixel[2] << bits;
        let a = pixel[3];

        output.put_pixel(x, y, Rgba([r, g, b, a]));
    }

    output
}

fn right(path: &str, bits: u8) -> RgbaImage {
    let img = image::open(path).expect("Failed to open image.");
    let (width, height) = img.dimensions();

    let mut output = ImageBuffer::new(width, height);

    for (x, y, pixel) in img.pixels() {
        let r = pixel[0] >> bits;
        let g = pixel[1] >> bits;
        let b = pixel[2] >> bits;
        let a = pixel[3];

        output.put_pixel(x, y, Rgba([r, g, b, a]));
    }

    output
}

fn main() {
    let args = Args::parse();

    let in_path = if let Some(path) = args.input_path {
        path
    } else {
        let stdin = io::stdin();
        let input = stdin
            .lock()
            .lines()
            .next()
            .expect("No input provided")
            .expect("Failed to read input");
        input.trim().to_string()
    };
    let out_path = args.output_path;
    let operation = args.function;
    let color_arg = args.color;
    let bit_shift = args.bit_shift;

    let rgb = match color_arg {
        Some(hex) => hex_to_rgb(&hex).unwrap(),
        None => (0, 0, 0),
    };

    let output: RgbaImage = match operation.as_str() {
        "or" => or(&in_path, RgbColor(rgb.0, rgb.1, rgb.2)),
        "and" => and(&in_path, RgbColor(rgb.0, rgb.1, rgb.2)),
        "xor" => xor(&in_path, RgbColor(rgb.0, rgb.1, rgb.2)),
        "left" => left(&in_path, bit_shift.expect("No bitshift value arg provided")),
        "right" => right(&in_path, bit_shift.expect("No bitshift value arg provided")),
        _ => panic!("Invalid operation"),
    };

    match out_path.as_str() {
        "" => output
            .save("output.bmp")
            .expect("Failed to save output image"),
        _ => output
            .save(out_path.clone())
            .expect("Failed to save output image"),
    }

    let mut stdout = io::stdout().lock();
    stdout
        .write_all(format!("{}\n", out_path).as_bytes())
        .expect("Failed to write to stdout");

    //let _ = Command::new("nsxiv")
    //    .arg("output.bmp")
    //    .spawn()
    //    .expect("Failed to execute");
}
