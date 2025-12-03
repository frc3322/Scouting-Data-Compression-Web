use crate::common::color_palette::{calculate_bits_per_pixel, BgrColor};

pub fn encode_byte_to_rgb(
    byte_val: u8,
    palette_bgr: &[BgrColor],
) -> Vec<BgrColor> {
    let num_colors = palette_bgr.len();
    let bits_per_pixel = calculate_bits_per_pixel(num_colors);
    let pixels_per_byte = (8.0 / bits_per_pixel as f64).ceil() as usize;
    
    let mut rgb_list = Vec::new();
    for pixel_idx in 0..pixels_per_byte {
        let bit_offset = pixel_idx * bits_per_pixel as usize;
        if bit_offset < 8 {
            let mask = (1u8 << bits_per_pixel) - 1;
            let color_index = ((byte_val >> bit_offset) & mask) as usize;
            rgb_list.push(palette_bgr[color_index]);
        } else {
            rgb_list.push(palette_bgr[0]);
        }
    }
    
    rgb_list
}

pub fn encode_2bytes_to_rgb(
    byte1: u8,
    byte2: u8,
    palette_bgr: &[BgrColor],
) -> Vec<BgrColor> {
    let num_colors = palette_bgr.len();
    let bits_per_pixel = calculate_bits_per_pixel(num_colors);
    
    if 16 % bits_per_pixel == 0 {
        let pixels_per_2bytes = 16 / bits_per_pixel;
        let combined_value = ((byte1 as u16) << 8) | (byte2 as u16);
        
        let mut rgb_list = Vec::new();
        for pixel_idx in 0..pixels_per_2bytes {
            let bit_offset = pixel_idx * bits_per_pixel;
            let mask = (1u16 << bits_per_pixel) - 1;
            let color_index = ((combined_value >> bit_offset) & mask) as usize;
            rgb_list.push(palette_bgr[color_index]);
        }
        rgb_list
    } else {
        let mut rgb_list = encode_byte_to_rgb(byte1, palette_bgr);
        rgb_list.extend(encode_byte_to_rgb(byte2, palette_bgr));
        rgb_list
    }
}

pub fn encode_bytes_to_rgb(
    data_bytes: &[u8],
    palette_bgr: &[BgrColor],
) -> Vec<BgrColor> {
    let num_colors = palette_bgr.len();
    let bits_per_pixel = calculate_bits_per_pixel(num_colors);
    
    let mut rgb_list = Vec::new();
    
    if 8 % bits_per_pixel == 0 {
        let mut i = 0;
        while i < data_bytes.len() {
            let byte1 = data_bytes[i];
            let byte2 = if i + 1 < data_bytes.len() { data_bytes[i + 1] } else { 0 };
            rgb_list.extend(encode_2bytes_to_rgb(byte1, byte2, palette_bgr));
            i += 2;
        }
    } else {
        for &byte_val in data_bytes {
            rgb_list.extend(encode_byte_to_rgb(byte_val, palette_bgr));
        }
    }
    
    rgb_list
}

