use crate::common::color_palette::{load_color_palette, palette_to_bgr, usable_color_set};
use crate::common::constants::DATA_COLOR_SEQUENCE;
use crate::common::schema::load_schema;
use crate::encoder::data_packer::{encode, read_csv};
use crate::encoder::image_generator::{calculate_minimum_image_size, create_encoded_image};
use image::ImageEncoder;

pub struct EncodeResult {
    pub image_bytes: Vec<u8>,
    pub packed_data: Vec<u8>,
}

pub fn encode_csv_to_image(
    csv_bytes: &[u8],
    schema_bytes: Option<&[u8]>,
    palette_bytes: Option<&[u8]>,
) -> Result<EncodeResult, anyhow::Error> {
    let schema = load_schema(schema_bytes)?;
    
    let palette_bgr = if let Some(palette_bytes) = palette_bytes {
        let palette_rgb = load_color_palette(palette_bytes)?;
        let usable_palette_rgb = usable_color_set(&palette_rgb);
        palette_to_bgr(&usable_palette_rgb)
    } else {
        let default_palette_rgb: Vec<_> = DATA_COLOR_SEQUENCE.iter().copied().collect();
        palette_to_bgr(&default_palette_rgb)
    };
    
    let (headers, rows) = read_csv(csv_bytes)?;
    
    let packed_data = encode(&headers, &rows, Some(&schema))?;
    
    let padding = 4;
    let tag_data_gap = 1;
    let data_padding = 4;
    
    let image_size = calculate_minimum_image_size(
        &packed_data,
        tag_data_gap,
        data_padding,
        20,
        &palette_bgr,
    )?;
    
    let encoded_image = create_encoded_image(
        &packed_data,
        image_size,
        image_size,
        padding,
        tag_data_gap,
        data_padding,
        Some(&palette_bgr),
    )?;
    
    let mut png_bytes = Vec::new();
    {
        let encoder = image::codecs::png::PngEncoder::new(&mut png_bytes);
        encoder.write_image(
            encoded_image.as_raw(),
            encoded_image.width(),
            encoded_image.height(),
            image::ColorType::Rgb8,
        )?;
    }
    
    Ok(EncodeResult {
        image_bytes: png_bytes,
        packed_data,
    })
}

