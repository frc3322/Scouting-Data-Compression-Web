use wasm_bindgen::prelude::*;

mod utils;

#[wasm_bindgen]
pub fn encode_csv_to_image(
    csv: &[u8],
    schema: Option<Vec<u8>>,
    palette: Option<Vec<u8>>,
) -> Result<Vec<u8>, JsValue> {
    let result = scouting_data_compression::api::encode_csv_to_image(
        csv,
        schema.as_deref(),
        palette.as_deref(),
    )
    .map_err(|e| JsValue::from_str(&e.to_string()))?;
    Ok(result.image_bytes)
}

