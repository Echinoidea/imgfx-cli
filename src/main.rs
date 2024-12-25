use clap::{builder::styling::RgbColor, Parser, Subcommand};
use image::{codecs::png::PngEncoder, ImageEncoder, ImageReader};
use std::io::{self, BufWriter, Cursor, Read, Write};

use imgfx::*;

#[derive(Subcommand)]
enum SubCommands {
    Or {
        color: String,
        negate: Option<String>,
    },
    And {
        color: String,
        negate: Option<String>,
    },
    Xor {
        color: String,
        negate: Option<String>,
    },
    Left {
        bits: String,
        raw: Option<String>,
    },
    Right {
        bits: String,
        raw: Option<String>,
    },
    Add {
        color: String,
    },
    Sub {
        color: String,
        raw: Option<String>,
    },
    Mult {
        color: String,
    },
    Pow {
        color: String,
    },
    Div {
        color: String,
    },
    Average {
        color: String,
    },
    Screen {
        color: String,
    },
    Overlay {
        color: String,
    },
    Bloom {
        intensity: f32,
        radius: f32,
        min_threshold: u8,
        max_threshold: Option<u8>,
    },
    Sort {
        direction: Direction,
        sort_by: SortBy,
        min_threshold: f32,
        max_threshold: f32,
        reversed: Option<String>,
    },
}

#[derive(Parser)]
#[command(name = "imgfx")]
#[command(version = "0.2.2")]
#[command(about = "Fast and configurable image filtering and operations.", long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: SubCommands,

    /// path/to/input/image
    #[arg(short, long)]
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
    ///// If function is 'left' or 'right', how many bits to shift by.
    //#[arg(short, long)]
    //bit_shift: Option<u8>,
}

fn main() {
    rayon::ThreadPoolBuilder::new()
        .num_threads(num_cpus::get())
        .build_global()
        .unwrap();

    let args = Args::parse();

    //let mut bit_shift = "";

    let in_path = args.input;
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

    let output = match args.cmd {
        SubCommands::Or { color, negate } => {
            let rgb = hex_to_rgb(&color).expect("Could not convert color to rgb");
            let n = match negate.as_deref() {
                Some("negate") => true,
                Some(_) => false,
                None => false,
            };

            or(img, lhs, rhs, RgbColor(rgb.0, rgb.1, rgb.2), n)
        }
        SubCommands::And { color, negate } => {
            let rgb = hex_to_rgb(&color).expect("Could not convert color to rgb");
            let n = match negate.as_deref() {
                Some("negate") => true,
                Some(_) => false,
                None => false,
            };
            and(img, lhs, rhs, RgbColor(rgb.0, rgb.1, rgb.2), n)
        }
        SubCommands::Xor { color, negate } => {
            let rgb = hex_to_rgb(&color).expect("Could not convert color to rgb");
            let n = match negate.as_deref() {
                Some("negate") => true,
                Some(_) => false,
                None => false,
            };
            xor(img, lhs, rhs, RgbColor(rgb.0, rgb.1, rgb.2), n)
        }
        SubCommands::Add { color } => {
            let rgb = hex_to_rgb(&color).expect("Could not convert color to rgb");
            add(img, lhs, rhs, RgbColor(rgb.0, rgb.1, rgb.2))
        }
        SubCommands::Sub { color, raw } => {
            let rgb = hex_to_rgb(&color).expect("Could not convert color to rgb");

            let raw = match raw.as_deref() {
                Some("raw") => true,
                Some(_) => false,
                None => false,
            };

            sub(img, lhs, rhs, RgbColor(rgb.0, rgb.1, rgb.2), raw)
        }
        SubCommands::Mult { color } => {
            let rgb = hex_to_rgb(&color).expect("Could not convert color to rgb");
            mult(img, lhs, rhs, RgbColor(rgb.0, rgb.1, rgb.2))
        }
        SubCommands::Pow { color } => {
            let rgb = hex_to_rgb(&color).expect("Could not convert color to rgb");
            pow(img, lhs, rhs, RgbColor(rgb.0, rgb.1, rgb.2))
        }
        SubCommands::Div { color } => {
            let rgb = hex_to_rgb(&color).expect("Could not convert color to rgb");
            div(img, lhs, rhs, RgbColor(rgb.0, rgb.1, rgb.2))
        }
        SubCommands::Left { bits, raw } => {
            let bit_shift = &bits;

            let raw = match raw.as_deref() {
                Some("raw") => true,
                Some(_) => false,
                None => false,
            };

            bitshift(
                img,
                BitshiftDirection::LEFT,
                lhs,
                bit_shift
                    .parse::<u8>()
                    .expect("Could not parse bits arg to u8"),
                raw,
            )
        }
        SubCommands::Right { bits, raw } => {
            let bit_shift = &bits;

            let raw = match raw.as_deref() {
                Some("raw") => true,
                Some(_) => false,
                None => false,
            };

            bitshift(
                img,
                BitshiftDirection::RIGHT,
                lhs,
                bit_shift
                    .parse::<u8>()
                    .expect("Could not parse bits arg to u8"),
                raw,
            )
        }
        SubCommands::Average { color } => {
            let rgb = hex_to_rgb(&color).expect("Could not convert color to rgb");
            average(img, lhs, rhs, RgbColor(rgb.0, rgb.1, rgb.2))
        }
        SubCommands::Screen { color } => {
            let rgb = hex_to_rgb(&color).expect("Could not convert color to rgb");
            screen(img, lhs, rhs, RgbColor(rgb.0, rgb.1, rgb.2))
        }
        SubCommands::Overlay { color } => {
            let rgb = hex_to_rgb(&color).expect("Could not convert color to rgb");
            overlay(img, lhs, rhs, RgbColor(rgb.0, rgb.1, rgb.2))
        }
        SubCommands::Bloom {
            intensity,
            radius,
            min_threshold,
            max_threshold,
        } => bloom(img, intensity, radius, min_threshold, max_threshold),
        SubCommands::Sort {
            direction,
            sort_by,
            min_threshold,
            max_threshold,
            reversed,
        } => {
            let reversed = match reversed.as_deref() {
                Some("reversed") => true,
                Some(_) => false,
                None => false,
            };
            sort(
                Into::into(img),
                direction,
                sort_by,
                min_threshold,
                max_threshold,
                reversed,
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
