use image::{DynamicImage, ImageOutputFormat};
use std::io::Cursor;

/// Compress and convert image to WebP format
/// Django: products/models.py - Product.save() image compression
pub fn compress_image(
    image_data: &[u8],
    max_width: u32,
    max_height: u32,
    quality: u8,
) -> Result<Vec<u8>, image::ImageError> {
    let img = image::load_from_memory(image_data)?;

    // Resize if needed
    let resized = if img.width() > max_width || img.height() > max_height {
        img.resize(max_width, max_height, image::imageops::FilterType::Lanczos3)
    } else {
        img
    };

    // Convert to WebP
    let mut output = Cursor::new(Vec::new());
    resized.write_to(&mut output, ImageOutputFormat::WebP)?;

    Ok(output.into_inner())
}

/// Get image dimensions
pub fn get_dimensions(image_data: &[u8]) -> Result<(u32, u32), image::ImageError> {
    let img = image::load_from_memory(image_data)?;
    Ok((img.width(), img.height()))
}
