use clap::Parser;
use image::{GenericImage, GenericImageView, ImageBuffer, Pixel, Rgba, RgbaImage};

#[derive(Parser, Debug)]
#[command(name = "ImgMod")]
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

fn or(path: &str) -> RgbaImage {
    let img = image::open(path).expect("Failed to open image.");
    let (width, height) = img.dimensions();

    let mut output = ImageBuffer::new(width, height);

    for (x, y, pixel) in img.pixels() {
        let r = pixel[0] | 0xFF;
        let g = pixel[1];
        let b = pixel[2];
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
    let color = args.color;

    println!("{}", in_path);
    let output: RgbaImage = match operation.as_str() {
        "or" => or(&in_path),
        "and" => {
            todo!()
        }
        "xor" => {
            todo!()
        }
        _ => panic!("Invalid operation"),
    };

    output
        .save("output.bmp")
        .expect("Failed to save output image");
}
