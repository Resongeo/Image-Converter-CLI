use clap::Parser;
use image::{DynamicImage, ImageFormat};
use indicatif::ProgressBar;
use std::path::Path;

#[derive(Parser)]
#[command(name = "Image Converter")]
#[command(version)]
#[command(about = "Convert an image to another format")]
#[command(
    long_about = "Convert an image to another format.\nSupported formats: jpg, jpeg, png, webp, pnm"
)]
struct Cli {
    path: String,
    #[arg(short)]
    format: String,
}

fn get_filename_from_path(path: &String) -> &str {
    let filename = Path::new(path).file_name().unwrap().to_str().unwrap();
    let name: Vec<&str> = filename.split('.').collect();
    name[0]
}

fn format_to_image_format(format: &String) -> ImageFormat {
    match format.to_lowercase().as_str() {
        "jpg" | "jpeg" => ImageFormat::Jpeg,
        "webp" => ImageFormat::WebP,
        "bmp" => ImageFormat::Bmp,
        "png" => ImageFormat::Png,
        _ => panic!("Unsupported format: {}", format),
    }
}

fn main() {
    let args = Cli::parse();

    let spinner = ProgressBar::new_spinner();
    spinner.set_message("Converting image...");
    spinner.enable_steady_tick(std::time::Duration::from_millis(100));

    let mut img = image::open(&args.path).expect("Couldn't open image");
    let filename = format!(
        "{}.{}",
        get_filename_from_path(&args.path),
        args.format.to_lowercase()
    );

    let requested_format = format_to_image_format(&args.format);

    img = if requested_format == ImageFormat::Jpeg {
        DynamicImage::ImageRgb8(img.into_rgb8())
    } else {
        img
    };

    img.save_with_format(filename, requested_format).unwrap();

    spinner.finish_with_message("Image convert complete!");
}
