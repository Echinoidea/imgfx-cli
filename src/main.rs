use clap::{builder::styling::RgbColor, Parser};
use image::{
    codecs::png::PngEncoder, DynamicImage, GenericImageView, ImageBuffer, ImageEncoder,
    ImageReader, Rgba, RgbaImage,
};
use std::io::{self, BufWriter, Cursor, Read, Write};

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
    output: Option<String>,

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
    img: DynamicImage,
    lhs: Option<Vec<String>>,
    rhs: Option<Vec<String>>,
    color: RgbColor,
) -> RgbaImage {
    let (width, height) = img.dimensions();

    let mut output: RgbaImage = ImageBuffer::new(width, height);

    let rhs = match rhs {
        Some(rhs) => (
            get_channel_by_name_rgb_color(&rhs[0], &color),
            get_channel_by_name_rgb_color(&rhs[1], &color),
            get_channel_by_name_rgb_color(&rhs[2], &color),
        ),
        None => (color.r(), color.b(), color.g()),
    };

    output.enumerate_pixels_mut().for_each(|(x, y, pixel)| {
        let in_pixel = img.get_pixel(x, y);

        let lhs = match lhs {
            Some(ref lhs) => (
                get_channel_by_name_rgba_u8(&lhs[0], &in_pixel),
                get_channel_by_name_rgba_u8(&lhs[1], &in_pixel),
                get_channel_by_name_rgba_u8(&lhs[2], &in_pixel),
            ),
            None => (in_pixel[0], in_pixel[1], in_pixel[2]),
        };

        let r = lhs.0 | rhs.0;
        let g = lhs.1 | rhs.1;
        let b = lhs.2 | rhs.2;
        let a = in_pixel[3];

        *pixel = Rgba([r, g, b, a]);
    });

    output
}

fn and(
    img: DynamicImage,
    lhs: Option<Vec<String>>,
    rhs: Option<Vec<String>>,
    color: RgbColor,
) -> RgbaImage {
    let (width, height) = img.dimensions();

    let mut output: RgbaImage = ImageBuffer::new(width, height);

    let rhs = match rhs {
        Some(rhs) => (
            get_channel_by_name_rgb_color(&rhs[0], &color),
            get_channel_by_name_rgb_color(&rhs[1], &color),
            get_channel_by_name_rgb_color(&rhs[2], &color),
        ),
        None => (color.r(), color.b(), color.g()),
    };

    output.enumerate_pixels_mut().for_each(|(x, y, pixel)| {
        let in_pixel = img.get_pixel(x, y);

        let lhs = match lhs {
            Some(ref lhs) => (
                get_channel_by_name_rgba_u8(&lhs[0], &in_pixel),
                get_channel_by_name_rgba_u8(&lhs[1], &in_pixel),
                get_channel_by_name_rgba_u8(&lhs[2], &in_pixel),
            ),
            None => (in_pixel[0], in_pixel[1], in_pixel[2]),
        };

        let r = lhs.0 & rhs.0;
        let g = lhs.1 & rhs.1;
        let b = lhs.2 & rhs.2;
        let a = in_pixel[3];

        *pixel = Rgba([r, g, b, a]);
    });

    output
}

fn xor(
    img: DynamicImage,
    lhs: Option<Vec<String>>,
    rhs: Option<Vec<String>>,
    color: RgbColor,
) -> RgbaImage {
    let (width, height) = img.dimensions();

    let mut output: RgbaImage = ImageBuffer::new(width, height);

    let rhs = match rhs {
        Some(rhs) => (
            get_channel_by_name_rgb_color(&rhs[0], &color),
            get_channel_by_name_rgb_color(&rhs[1], &color),
            get_channel_by_name_rgb_color(&rhs[2], &color),
        ),
        None => (color.r(), color.b(), color.g()),
    };

    output.enumerate_pixels_mut().for_each(|(x, y, pixel)| {
        let in_pixel = img.get_pixel(x, y);

        let lhs = match lhs {
            Some(ref lhs) => (
                get_channel_by_name_rgba_u8(&lhs[0], &in_pixel),
                get_channel_by_name_rgba_u8(&lhs[1], &in_pixel),
                get_channel_by_name_rgba_u8(&lhs[2], &in_pixel),
            ),
            None => (in_pixel[0], in_pixel[1], in_pixel[2]),
        };

        let r = lhs.0 ^ rhs.0;
        let g = lhs.1 ^ rhs.1;
        let b = lhs.2 ^ rhs.2;
        let a = in_pixel[3];

        *pixel = Rgba([r, g, b, a]);
    });

    output
}

enum BitshiftDirection {
    LEFT,
    RIGHT,
}

fn bitshift(img: DynamicImage, direction: BitshiftDirection, bits: u8) -> RgbaImage {
    let (width, height) = img.dimensions();

    let mut output: RgbaImage = ImageBuffer::new(width, height);

    output.enumerate_pixels_mut().for_each(|(x, y, pixel)| {
        let in_pixel = img.get_pixel(x, y);

        let (r, g, b, a) = match direction {
            BitshiftDirection::LEFT => (
                in_pixel[0] << bits,
                in_pixel[1] << bits,
                in_pixel[2] << bits,
                in_pixel[3],
            ),
            BitshiftDirection::RIGHT => (
                in_pixel[0] >> bits,
                in_pixel[1] >> bits,
                in_pixel[2] >> bits,
                in_pixel[3],
            ),
        };

        *pixel = Rgba([r, g, b, a]);
    });

    output
}

fn main() {
    let args = Args::parse();

    let in_path = args.input_path;
    //let out_path = args.output;
    let operation = args.function;
    let color_arg = args.color;
    let bit_shift = args.bit_shift;
    let lhs = args.lhs;
    let rhs = args.rhs;

    let img = match in_path {
        Some(path) => image::open(path).expect("Failed to open image."),
        None => {
            let mut buffer = Vec::new();

            io::stdin()
                .read_to_end(&mut buffer)
                .expect("Failed to read from stdin");

            ImageReader::new(Cursor::new(buffer))
                .with_guessed_format()
                .expect("Failed to guess image format")
                .decode()
                .expect("Failed to decode image from stdin")
        }
    };

    let rgb = match color_arg {
        Some(hex) => hex_to_rgb(&hex).unwrap(),
        None => (0, 0, 0),
    };

    let output: RgbaImage = match operation.as_str() {
        "or" => or(img, lhs, rhs, RgbColor(rgb.0, rgb.1, rgb.2)),
        "and" => and(img, lhs, rhs, RgbColor(rgb.0, rgb.1, rgb.2)),
        "xor" => xor(img, lhs, rhs, RgbColor(rgb.0, rgb.1, rgb.2)),
        "left" => bitshift(img, BitshiftDirection::LEFT, bit_shift.unwrap()),
        "right" => bitshift(img, BitshiftDirection::RIGHT, bit_shift.unwrap()),
        _ => panic!("Invalid operation"),
    };

    // Epic fast buffer writing
    let stdout = io::stdout();
    let handle = stdout.lock();
    let mut writer = BufWriter::with_capacity(2048 * 2048, handle);

    let encoder = PngEncoder::new_with_quality(
        &mut writer,
        image::codecs::png::CompressionType::Fast,
        image::codecs::png::FilterType::NoFilter,
    );

    encoder
        .write_image(
            &output,
            output.width(),
            output.height(),
            image::ExtendedColorType::Rgba8,
        )
        .expect("Error while encoding buffer");

    writer.flush().expect("error flushing writer");
}
