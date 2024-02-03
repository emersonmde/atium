use std::env;
use std::path::PathBuf;
use std::process::Command;

use image::GenericImageView;
use tempfile::tempdir;

mod algebra;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} '<expression>'", args[0]);
        std::process::exit(1);
    }

    let (_, expr) = algebra::parser::parse_expression(&args[1]).unwrap();
    println!("Starting expression: {:?}", expr);

    let mut typist_expression = expr.simplify().to_typist();
    println!("Simplified expression: {}", typist_expression);


    let temp_dir = tempdir()?;
    let temp_dir_path = temp_dir.path();


    let typist_file_path = temp_dir_path.join("expression.typ");
    typist_expression.push_str("\n\n");
    std::fs::write(&typist_file_path, typist_expression)?;

    // Compile the Typist file to PNG
    let output_png_path = temp_dir_path.join("output.png");
    let output = Command::new("typst")
        .args(["compile", typist_file_path.to_str().unwrap(), output_png_path.to_str().unwrap()])
        .output()?;
    if !output.status.success() {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("typst failed: {}", String::from_utf8_lossy(&output.stderr))));
    }

    // Trim whitespace from the PNG
    let trimmed_png_path = temp_dir_path.join("trimmed_output.png");
    trim_whitespace(output_png_path.to_str().unwrap(), trimmed_png_path.to_str().unwrap(), 10)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?;


    // Output using `imgcat`
    let imgcat_path = find_imgcat().expect("imgcat not found, install iTerm2 shell integrations: https://iterm2.com/documentation-images.html");
    let status = Command::new(imgcat_path)
        .arg(trimmed_png_path.to_str().unwrap())
        .status()?;
    if !status.success() {
        return Err(std::io::Error::new(std::io::ErrorKind::Other, "imgcat failed"));
    }

    Ok(())
}

fn trim_whitespace(input_path: &str, output_path: &str, margin: u32) -> image::ImageResult<()> {
    let img = image::open(input_path)?;
    let (width, height) = img.dimensions();

    let mut top = height;
    let mut bottom = 0;
    let mut left = width;
    let mut right = 0;

    let white_pixel = image::Rgba([255u8, 255u8, 255u8, 255u8]);
    for (x, y, pixel) in img.pixels() {
        if pixel != white_pixel { // Non-transparent pixel
            top = top.min(y);
            bottom = bottom.max(y);
            left = left.min(x);
            right = right.max(x);
        }
    }

    // Adding a small margin around the text
    top = top.saturating_sub(margin);
    bottom = (bottom + margin).min(height - 1);
    left = left.saturating_sub(margin);
    right = (right + margin).min(width - 1);

    // Ensure there is something to crop
    if top < bottom && left < right {
        let cropped = img.crop_imm(left, top, right - left + 1, bottom - top + 1);
        cropped.save(output_path)?;
    } else {
        return Err(image::ImageError::Parameter(image::error::ParameterError::from_kind(
            image::error::ParameterErrorKind::DimensionMismatch,
        )));
    }

    Ok(())
}


fn find_imgcat() -> Option<PathBuf> {
    // Check ~/.iterm2/imgcat
    let home_dir = env::var("HOME").ok()?;
    let mut imgcat_path = PathBuf::from(home_dir);
    imgcat_path.extend(&[".iterm2", "imgcat"]);

    if imgcat_path.exists() {
        return Some(imgcat_path);
    }

    // Check PATH
    if let Ok(path) = env::var("PATH") {
        env::split_paths(&path).find(|p| {
            let mut imgcat_path = p.clone();
            imgcat_path.push("imgcat");
            imgcat_path.exists()
        }).map(|p| {
            let mut imgcat_path = p;
            imgcat_path.push("imgcat");
            imgcat_path
        })
    } else {
        None
    }
}