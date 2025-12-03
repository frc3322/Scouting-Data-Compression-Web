use serde::{Deserialize, Serialize};

pub type RgbColor = (u8, u8, u8);
pub type BgrColor = (u8, u8, u8);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorPalette {
    pub colors: Vec<[u8; 3]>,
}

pub fn load_color_palette(palette_bytes: &[u8]) -> Result<Vec<RgbColor>, anyhow::Error> {
    let palette: Vec<[u8; 3]> = serde_json::from_slice(palette_bytes)
        .map_err(|e| anyhow::anyhow!("Invalid palette JSON: {}", e))?;
    
    if palette.len() < 2 {
        return Err(anyhow::anyhow!("Palette must contain at least 2 colors"));
    }
    
    Ok(palette.iter().map(|c| (c[0], c[1], c[2])).collect())
}

pub fn usable_color_set(palette: &[RgbColor]) -> Vec<RgbColor> {
    if palette.len() < 2 {
        return palette.to_vec();
    }
    
    let mut power_of_two = 1;
    while power_of_two * 2 <= palette.len() {
        power_of_two *= 2;
    }
    
    palette[..power_of_two].to_vec()
}

pub fn rgb_to_bgr(rgb: RgbColor) -> BgrColor {
    (rgb.2, rgb.1, rgb.0)
}

pub fn palette_to_bgr(palette: &[RgbColor]) -> Vec<BgrColor> {
    palette.iter().map(|&rgb| rgb_to_bgr(rgb)).collect()
}

pub fn calculate_bits_per_pixel(num_colors: usize) -> u32 {
    if num_colors == 0 {
        return 0;
    }
    (num_colors as f64).log2() as u32
}

