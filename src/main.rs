use clap::{builder::styling::RgbColor, ArgAction, Parser, Subcommand};
use image::{
    codecs::png::PngEncoder, DynamicImage, GenericImageView, ImageBuffer, ImageEncoder,
    ImageReader, Rgba, RgbaImage,
};
use std::io::{self, BufWriter, Cursor, Read, Write};

#[derive(Subcommand)]
enum SubCommands {
    OR { color: String },
    AND { color: String },
    XOR { color: String },
    LEFT { bits: String },
    RIGHT { bits: String },
    ADD { color: String },
    SUB { color: String },
    MULT { color: String },
    DIV { color: String },
}

#[derive(Parser)]
#[command(name = "img-mod")]
#[command(version = "1.0.1")]
#[command(about = "Bitwise operations and other stuff to images.", long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: SubCommands,

    /// path/to/input/image
    #[arg(short, long, global = true)]
    input: Option<String>,

    /// path/to/output/image
    #[arg(long, default_value = ".")]
    output: Option<String>,

    /// Specify the left hand side operands for the function. E.g. --lhs b g r
    #[arg(long, num_args(1..), global = true)]
    lhs: Option<Vec<String>>,

    /// Specify the right hand side operands for the function. E.g. --rhs b r b
    #[arg(long, num_args(1..), global = true)]
    rhs: Option<Vec<String>>,

    /// If function is 'left' or 'right', how many bits to shift by.
    #[arg(short, long)]
    bit_shift: Option<u8>,

    /// Negate the logical operator
    #[arg(short, long, action=ArgAction::SetTrue, global = true)]
    negate: bool,
}

fn hex_to_rgb(hex: &str) -> Option<(u8, u8, u8)> {
    if hex.starts_with('#') && hex.len() == 7 {
        let r = u8::from_str_radix(&hex[1..3], 16).ok()?;
        let g = u8::from_str_radix(&hex[3..5], 16).ok()?;
        let b = u8::from_str_radix(&hex[5..7], 16).ok()?;
        Some((r, g, b))
    } else if hex.len() == 6 {
        let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
        let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
        let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
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
    negate: bool,
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

        let (r, g, b) = match negate {
            true => (!(lhs.0 | rhs.0), !(lhs.1 | rhs.1), !(lhs.2 | rhs.2)),
            false => ((lhs.0 | rhs.0), (lhs.1 | rhs.1), (lhs.2 | rhs.2)),
        };

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
    negate: bool,
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

        let (r, g, b) = match negate {
            true => (!(lhs.0 & rhs.0), !(lhs.1 & rhs.1), !(lhs.2 & rhs.2)),
            false => ((lhs.0 & rhs.0), (lhs.1 & rhs.1), (lhs.2 & rhs.2)),
        };

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
    negate: bool,
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

        let (r, g, b) = match negate {
            true => (!(lhs.0 ^ rhs.0), !(lhs.1 ^ rhs.1), !(lhs.2 ^ rhs.2)),
            false => ((lhs.0 ^ rhs.0), (lhs.1 ^ rhs.1), (lhs.2 ^ rhs.2)),
        };

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
                ((in_pixel[0] as u16) << bits).min(255) as u8,
                ((in_pixel[1] as u16) << bits).min(255) as u8,
                ((in_pixel[2] as u16) << bits).min(255) as u8,
                in_pixel[3],
            ),
            BitshiftDirection::RIGHT => (
                ((in_pixel[0] as u16).wrapping_shr(bits.into())).min(255) as u8,
                ((in_pixel[1] as u16).wrapping_shr(bits.into())).min(255) as u8,
                ((in_pixel[2] as u16).wrapping_shr(bits.into())).min(255) as u8,
                in_pixel[3],
            ),
        };

        *pixel = Rgba([r, g, b, a]);
    });

    output
}

fn add(
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

        let r = lhs.0.wrapping_add(rhs.0);
        let g = lhs.1.wrapping_add(rhs.1);
        let b = lhs.2.wrapping_add(rhs.2);
        let a = in_pixel[3];

        *pixel = Rgba([r, g, b, a]);
    });

    output
}

fn sub(
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

        let r = lhs.0 - rhs.0;
        let g = lhs.1 - rhs.1;
        let b = lhs.2 - rhs.2;
        let a = in_pixel[3];

        *pixel = Rgba([r, g, b, a]);
    });

    output
}

fn mult(
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

        let r = lhs.0.wrapping_mul(rhs.0);
        let g = lhs.1.wrapping_mul(rhs.1);
        let b = lhs.2.wrapping_mul(rhs.2);
        let a = in_pixel[3];

        *pixel = Rgba([r, g, b, a]);
    });

    output
}

fn div(
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

        let r = lhs.0 / rhs.0.max(1);
        let g = lhs.1 / rhs.1.max(1);
        let b = lhs.2 / rhs.2.max(1);
        let a = in_pixel[3];

        *pixel = Rgba([r, g, b, a]);
    });

    output
}

fn main() {
    let args = Args::parse();

    let mut color_arg: Option<&str> = None;
    let mut bit_shift = "";

    let in_path = args.input;
    let lhs = args.lhs;
    let rhs = args.rhs;
    let negate = args.negate;

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

    let output = match args.cmd {
        SubCommands::OR { color } => {
            let rgb = hex_to_rgb(&color).expect("Could not convert color to rgb");
            or(img, lhs, rhs, RgbColor(rgb.0, rgb.1, rgb.2), negate)
        }
        SubCommands::AND { color } => {
            let rgb = hex_to_rgb(&color).expect("Could not convert color to rgb");
            and(img, lhs, rhs, RgbColor(rgb.0, rgb.1, rgb.2), negate)
        }
        SubCommands::XOR { color } => {
            let rgb = hex_to_rgb(&color).expect("Could not convert color to rgb");
            xor(img, lhs, rhs, RgbColor(rgb.0, rgb.1, rgb.2), negate)
        }
        SubCommands::ADD { color } => {
            let rgb = hex_to_rgb(&color).expect("Could not convert color to rgb");
            add(img, lhs, rhs, RgbColor(rgb.0, rgb.1, rgb.2))
        }
        SubCommands::SUB { color } => {
            let rgb = hex_to_rgb(&color).expect("Could not convert color to rgb");
            sub(img, lhs, rhs, RgbColor(rgb.0, rgb.1, rgb.2))
        }
        SubCommands::MULT { color } => {
            let rgb = hex_to_rgb(&color).expect("Could not convert color to rgb");
            mult(img, lhs, rhs, RgbColor(rgb.0, rgb.1, rgb.2))
        }
        SubCommands::DIV { color } => {
            let rgb = hex_to_rgb(&color).expect("Could not convert color to rgb");
            div(img, lhs, rhs, RgbColor(rgb.0, rgb.1, rgb.2))
        }
        SubCommands::LEFT { bits } => {
            bit_shift = &bits;
            bitshift(
                img,
                BitshiftDirection::LEFT,
                bit_shift
                    .parse::<u8>()
                    .expect("Could not parse bits arg to u8"),
            )
        }
        SubCommands::RIGHT { bits } => {
            bit_shift = &bits;
            bitshift(
                img,
                BitshiftDirection::RIGHT,
                bit_shift
                    .parse::<u8>()
                    .expect("Could not parse bits arg to u8"),
            )
        }
    };

    // Epic fast buffer writing
    let stdout = io::stdout();
    let handle = stdout.lock();
    let mut writer = BufWriter::new(handle);

    let encoder = PngEncoder::new_with_quality(
        &mut writer,
        image::codecs::png::CompressionType::Default,
        image::codecs::png::FilterType::Adaptive,
    );

    encoder
        .write_image(
            &output,
            output.width(),
            output.height(),
            image::ExtendedColorType::Rgba8,
        )
        .expect("Error while encoding buffer");
    //output.write_to(&mut writer, image::ImageFormat::Png);

    writer.flush().expect("error flushing writer");
}
