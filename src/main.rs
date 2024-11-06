use clap::{builder::styling::RgbColor, Parser};
use image::{EncodableLayout, GenericImageView, ImageBuffer, Pixel, Rgba, RgbaImage};
use std::io::{self, BufRead, Write};

#[derive(Parser, Debug)]
#[command(name = "img-mod")]
#[command(version = "1.0.0")]
#[command(about = "Bitwise operations and other stuff to images.", long_about = None)]
struct Args {
    /// path/to/input/image
    #[arg(short, long)]
    input_path: Option<String>,

    /// path/to/output/image
    #[arg(long, default_value = ".")]
    output: String,

    /// Function to perform on input image. 'or', 'and', 'xor', 'left', 'right'
    #[arg(short, long)]
    function: String,

    /// Specify the left hand side operands for the function. E.g. --lhs b g r
    #[arg(long, num_args(1..))]
    lhs: Option<Vec<String>>,

    /// Specify the right hand side operands for the function. E.g. --rhs b r b
    #[arg(long, num_args(1..))]
    rhs: Option<Vec<String>>,

    /// String hex color value to compare input image color to. Must be "#AABBCC" format
    #[arg(short, long)]
    color: Option<String>,

    /// If function is 'left' or 'right', how many bits to shift by.
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

fn get_channel_by_name_rgb_color(name: &str, color: &RgbColor) -> u8 {
    match name {
        "r" => color.r(),
        "g" => color.g(),
        "b" => color.b(),
        _ => 0,
    }
}

fn get_channel_by_name_rgba_u8(name: &str, color: &Rgba<u8>) -> u8 {
    match name {
        "r" => color[0],
        "g" => color[1],
        "b" => color[2],
        _ => 0,
    }
}

fn or(
    path: &str,
    lhs: Option<Vec<String>>,
    rhs: Option<Vec<String>>,
    color: RgbColor,
) -> RgbaImage {
    let img = image::open(path).expect("Failed to open image.");
    let (width, height) = img.dimensions();

    let mut output = ImageBuffer::new(width, height);

    let rhs = match rhs {
        Some(rhs) => (
            get_channel_by_name_rgb_color(&rhs[0], &color),
            get_channel_by_name_rgb_color(&rhs[1], &color),
            get_channel_by_name_rgb_color(&rhs[2], &color),
        ),
        None => (color.r(), color.b(), color.g()),
    };

    for (x, y, pixel) in img.pixels() {
        let lhs = match lhs {
            Some(ref lhs) => (
                get_channel_by_name_rgba_u8(&lhs[0], &pixel),
                get_channel_by_name_rgba_u8(&lhs[1], &pixel),
                get_channel_by_name_rgba_u8(&lhs[2], &pixel),
            ),
            None => (pixel[0], pixel[1], pixel[2]),
        };

        let r = lhs.0 | rhs.0;
        let g = lhs.1 | rhs.1;
        let b = lhs.2 | rhs.2;
        let a = pixel[3];

        output.put_pixel(x, y, Rgba([r, g, b, a]));
    }

    output
}

fn and(
    path: &str,
    lhs: Option<Vec<String>>,
    rhs: Option<Vec<String>>,
    color: RgbColor,
) -> RgbaImage {
    let img = image::open(path).expect("Failed to open image.");
    let (width, height) = img.dimensions();

    let mut output = ImageBuffer::new(width, height);

    let rhs = match rhs {
        Some(rhs) => (
            get_channel_by_name_rgb_color(&rhs[0], &color),
            get_channel_by_name_rgb_color(&rhs[1], &color),
            get_channel_by_name_rgb_color(&rhs[2], &color),
        ),
        None => (color.r(), color.b(), color.g()),
    };

    for (x, y, pixel) in img.pixels() {
        let lhs = match lhs {
            Some(ref lhs) => (
                get_channel_by_name_rgba_u8(&lhs[0], &pixel),
                get_channel_by_name_rgba_u8(&lhs[1], &pixel),
                get_channel_by_name_rgba_u8(&lhs[2], &pixel),
            ),
            None => (pixel[0], pixel[1], pixel[2]),
        };

        let r = lhs.0 & rhs.0;
        let g = lhs.1 & rhs.1;
        let b = lhs.2 & rhs.2;
        let a = pixel[3];

        output.put_pixel(x, y, Rgba([r, g, b, a]));
    }

    output
}

fn xor(
    path: &str,
    lhs: Option<Vec<String>>,
    rhs: Option<Vec<String>>,
    color: RgbColor,
) -> RgbaImage {
    let img = image::open(path).expect("Failed to open image.");
    let (width, height) = img.dimensions();

    let mut output = ImageBuffer::new(width, height);

    let rhs = match rhs {
        Some(rhs) => (
            get_channel_by_name_rgb_color(&rhs[0], &color),
            get_channel_by_name_rgb_color(&rhs[1], &color),
            get_channel_by_name_rgb_color(&rhs[2], &color),
        ),
        None => (color.r(), color.b(), color.g()),
    };

    for (x, y, pixel) in img.pixels() {
        let lhs = match lhs {
            Some(ref lhs) => (
                get_channel_by_name_rgba_u8(&lhs[0], &pixel),
                get_channel_by_name_rgba_u8(&lhs[1], &pixel),
                get_channel_by_name_rgba_u8(&lhs[2], &pixel),
            ),
            None => (pixel[0], pixel[1], pixel[2]),
        };

        let r = lhs.0 ^ rhs.0;
        let g = lhs.1 ^ rhs.1;
        let b = lhs.2 ^ rhs.2;
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
    let out_path = args.output;
    let operation = args.function;
    let color_arg = args.color;
    let bit_shift = args.bit_shift;
    let lhs = args.lhs;
    let rhs = args.rhs;

    let rgb = match color_arg {
        Some(hex) => hex_to_rgb(&hex).unwrap(),
        None => (0, 0, 0),
    };

    let output: RgbaImage = match operation.as_str() {
        "or" => or(&in_path, lhs, rhs, RgbColor(rgb.0, rgb.1, rgb.2)),
        "and" => and(&in_path, lhs, rhs, RgbColor(rgb.0, rgb.1, rgb.2)),
        "xor" => xor(&in_path, lhs, rhs, RgbColor(rgb.0, rgb.1, rgb.2)),
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
}
