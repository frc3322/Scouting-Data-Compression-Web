use crate::common::apriltag::generate_april_tags_image;
use crate::common::color_palette::{calculate_bits_per_pixel, palette_to_bgr, BgrColor};
use crate::common::constants::DATA_COLOR_SEQUENCE;
use crate::common::data_regions::get_data_regions;
use crate::encoder::color_encoder::encode_bytes_to_rgb;
use image::{Rgb, RgbImage};

pub fn calculate_minimum_image_size(
    data_bytes: &[u8],
    tag_data_gap: usize,
    data_padding: usize,
    start_size: usize,
    palette_bgr: &[BgrColor],
) -> Result<usize, anyhow::Error> {
    let bits_per_pixel = calculate_bits_per_pixel(palette_bgr.len());
    let pixels_per_byte = (8.0 / bits_per_pixel as f64).ceil() as usize;
    let num_calibration_colors = palette_bgr.len();
    
    let pixels_needed = if 16 % bits_per_pixel == 0 {
        let pixels_per_2bytes = (16 / bits_per_pixel) as usize;
        ((data_bytes.len() + 1) / 2) * pixels_per_2bytes + num_calibration_colors
    } else {
        data_bytes.len() * pixels_per_byte + num_calibration_colors
    };
    
    let mut image_size = start_size;
    while image_size <= 1000 {
        let data_regions = get_data_regions(image_size, image_size, tag_data_gap, data_padding);
        let mut pixel_coords: Vec<(usize, usize)> = Vec::new();
        
        for region in &data_regions {
            for row in region.row_start..region.row_end {
                for col in region.col_start..region.col_end {
                    pixel_coords.push((row, col));
                }
            }
        }
        
        if pixel_coords.len() >= pixels_needed {
            return Ok(image_size);
        }
        
        image_size += 2;
    }
    
    Err(anyhow::anyhow!(
        "Cannot find suitable image size for {} bytes of data",
        data_bytes.len()
    ))
}

pub fn create_encoded_image(
    data_bytes: &[u8],
    image_width: usize,
    image_height: usize,
    padding: usize,
    tag_data_gap: usize,
    data_padding: usize,
    palette_bgr: Option<&[BgrColor]>,
) -> Result<RgbImage, anyhow::Error> {
    let palette_bgr = match palette_bgr {
        Some(p) => p,
        None => {
            let default_palette_rgb: Vec<_> = DATA_COLOR_SEQUENCE.iter().copied().collect();
            &palette_to_bgr(&default_palette_rgb)
        }
    };
    
    let mut image = generate_april_tags_image(image_width, image_height, padding)?;
    
    let data_regions = get_data_regions(image_width, image_height, tag_data_gap, data_padding);
    
    let encoded_colors = encode_bytes_to_rgb(data_bytes, palette_bgr);
    
    let mut pixel_coords: Vec<(usize, usize)> = Vec::new();
    for region in &data_regions {
        for row in region.row_start..region.row_end {
            for col in region.col_start..region.col_end {
                pixel_coords.push((row, col));
            }
        }
    }
    
    let bytes_needed = data_bytes.len();
    let bits_per_pixel = calculate_bits_per_pixel(palette_bgr.len());
    let pixels_per_byte = (8.0 / bits_per_pixel as f64).ceil() as usize;
    
    let pixels_needed = if 16 % bits_per_pixel == 0 {
        let pixels_per_2bytes = (16 / bits_per_pixel) as usize;
        ((bytes_needed + 1) / 2) * pixels_per_2bytes
    } else {
        bytes_needed * pixels_per_byte
    };
    
    let pixels_available = pixel_coords.len();
    let num_calibration_colors = palette_bgr.len();
    let calibration_pixels_needed = num_calibration_colors;
    
    if pixels_needed + calibration_pixels_needed > pixels_available {
        return Err(anyhow::anyhow!(
            "Not enough pixels to encode data: need {} pixels for {} bytes + calibration, but only have {} pixels available",
            pixels_needed + calibration_pixels_needed,
            bytes_needed,
            pixels_available
        ));
    }
    
    // Place encoded data first
    for (i, &(row, col)) in pixel_coords.iter().enumerate() {
        if i < encoded_colors.len() {
            let bgr = encoded_colors[i];
            image.put_pixel(col as u32, row as u32, Rgb([bgr.2, bgr.1, bgr.0]));
        }
    }
    
    // Place calibration pixels at the very end of data regions
    let calibration_colors: Vec<BgrColor> = palette_bgr.iter().copied().collect();
    
    for (i, &calibration_color) in calibration_colors.iter().enumerate() {
        let coord_index = pixels_available - calibration_pixels_needed + i;
        let (row, col) = pixel_coords[coord_index];
        image.put_pixel(col as u32, row as u32, Rgb([calibration_color.2, calibration_color.1, calibration_color.0]));
    }
    
    Ok(image)
}

