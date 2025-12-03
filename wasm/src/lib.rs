use wasm_bindgen::prelude::*;

mod utils;

#[wasm_bindgen]
pub fn encode_csv_to_image(
    csv: &[u8],
    schema: Option<&[u8]>,
    palette: Option<&[u8]>,
) -> Result<Vec<u8>, JsValue> {
    let result = scouting_data_compression::api::encode_csv_to_image(csv, schema, palette)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    Ok(result.image_bytes)
}

