use anyhow::{anyhow, Result};
use std::env;
use std::path::PathBuf;
use std::process::Command;

use crate::algebra::expression::Expression;
use image::GenericImageView;
use tempfile::tempdir;

mod algebra;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} '<expression>'", args[0]);
        std::process::exit(1);
    }

    let (_, expr) = algebra::parser::parse_expression(&args[1]).unwrap();

    let simplified_expr = expr.simplify();

    println!("Simplified Expression: {:?}\n", simplified_expr.to_typist());
    let imgcat_path = find_imgcat();
    if let Some(imgcat_path) = imgcat_path {
        print_expr_as_img(simplified_expr, imgcat_path).unwrap();
    }

    Ok(())
}

fn print_expr_as_img(simplified_expr: Box<dyn Expression>, imgcat_path: PathBuf) -> Result<()> {
    let temp_dir = tempdir()?;
    let temp_dir_path = temp_dir.path();

    let typist_file_path = temp_dir_path.join("expression.typ");
    let mut typist_expression = simplified_expr.to_typist();
    typist_expression.push_str("\n\n");
    std::fs::write(&typist_file_path, typist_expression)?;

    // Compile the Typist file to PNG
    let output_png_path = temp_dir_path.join("output.png");
    let output = Command::new("typst")
        .args([
            "compile",
            typist_file_path.to_str().unwrap(),
            output_png_path.to_str().unwrap(),
        ])
        .output()?;
    if !output.status.success() {
        return Err(anyhow!(
            "typst failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    // Trim whitespace from the PNG
    let trimmed_png_path = temp_dir_path.join("trimmed_output.png");
    crop_and_scale(
        output_png_path.to_str().unwrap(),
        trimmed_png_path.to_str().unwrap(),
        10,
        2.0,
    )?;

    // Output using imgcat
    let status = Command::new(imgcat_path)
        .arg(trimmed_png_path.to_str().unwrap())
        .status()?;
    if !status.success() {
        return Err(anyhow!("imgcat failed"));
    }

    Ok(())
}

fn crop_and_scale(
    input_path: &str,
    output_path: &str,
    margin: u32,
    scaling_factor: f32,
) -> Result<()> {
    let img = image::open(input_path)?;
    let (width, height) = img.dimensions();

    let mut top = height;
    let mut bottom = 0;
    let mut left = width;
    let mut right = 0;

    let white_pixel = image::Rgba([255u8, 255u8, 255u8, 255u8]);
    for (x, y, pixel) in img.pixels() {
        if pixel != white_pixel {
            // Non-transparent pixel
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
        let scaled = cropped.resize(
            (cropped.width() as f32 * scaling_factor) as u32,
            (cropped.height() as f32 * scaling_factor) as u32,
            image::imageops::FilterType::CatmullRom,
        );
        scaled.save(output_path)?;
    } else {
        return Err(anyhow!("No content found in the image"));
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
        env::split_paths(&path)
            .find(|p| {
                let mut imgcat_path = p.clone();
                imgcat_path.push("imgcat");
                imgcat_path.exists()
            })
            .map(|p| {
                let mut imgcat_path = p;
                imgcat_path.push("imgcat");
                imgcat_path
            })
    } else {
        None
    }
}
