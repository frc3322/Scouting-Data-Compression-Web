use image::{GrayImage, Rgb, RgbImage};
use std::path::PathBuf;

pub fn get_april_tag_size() -> usize {
    8
}

pub fn load_april_tag(tag_id: u32) -> Result<GrayImage, anyhow::Error> {
    let tag_filename = format!("tag36_11_{:05}.png", tag_id);
    let tag_path = get_asset_path(&tag_filename)?;
    
    let img = image::open(&tag_path)?;
    let gray = img.to_luma8();
    
    let width = gray.width() as usize;
    let height = gray.height() as usize;
    
    if width < 2 || height < 2 {
        return Err(anyhow::anyhow!("AprilTag image too small"));
    }
    
    let cropped = GrayImage::from_fn(
        (width - 2) as u32,
        (height - 2) as u32,
        |x, y| *gray.get_pixel(x + 1, y + 1)
    );
    
    Ok(cropped)
}

pub fn generate_april_tags_image(
    image_width: usize,
    image_height: usize,
    padding: usize,
) -> Result<RgbImage, anyhow::Error> {
    let mut image = RgbImage::from_pixel(
        image_width as u32,
        image_height as u32,
        Rgb([255, 255, 255]),
    );
    
    let tag_0 = load_april_tag(0)?;
    let tag_1 = load_april_tag(1)?;
    let tag_2 = load_april_tag(2)?;
    let tag_size = tag_0.width() as usize;
    
    if image_width < (2 * padding) + tag_size || image_height < (2 * padding) + tag_size {
        return Err(anyhow::anyhow!(
            "Image dimensions are too small for the AprilTag and padding: width={}, height={}, padding={}, tag_size={}",
            image_width, image_height, padding, tag_size
        ));
    }
    
    // Top-left: tag 0
    place_tag(&mut image, &tag_0, padding, padding)?;
    
    // Top-right: tag 1
    place_tag(&mut image, &tag_1, padding, image_width - padding - tag_size)?;
    
    // Bottom-left: tag 2
    place_tag(&mut image, &tag_2, image_height - padding - tag_size, padding)?;
    
    Ok(image)
}

fn place_tag(
    image: &mut RgbImage,
    tag: &GrayImage,
    row: usize,
    col: usize,
) -> Result<(), anyhow::Error> {
    let tag_width = tag.width() as usize;
    let tag_height = tag.height() as usize;
    
    if row + tag_height > image.height() as usize || col + tag_width > image.width() as usize {
        return Err(anyhow::anyhow!("Tag placement out of bounds"));
    }
    
    for y in 0..tag_height {
        for x in 0..tag_width {
            let gray_val = tag.get_pixel(x as u32, y as u32)[0];
            image.put_pixel(
                (col + x) as u32,
                (row + y) as u32,
                Rgb([gray_val, gray_val, gray_val]),
            );
        }
    }
    
    Ok(())
}

fn get_asset_path(filename: &str) -> Result<PathBuf, anyhow::Error> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("src");
    path.push("common");
    path.push("assets");
    path.push(filename);
    
    if !path.exists() {
        return Err(anyhow::anyhow!("AprilTag image not found: {}", path.display()));
    }
    
    Ok(path)
}

