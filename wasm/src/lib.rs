use wasm_bindgen::prelude::*;

mod utils;

#[wasm_bindgen]
pub fn encode_csv_to_image(
    csv: &[u8],
    schema: Option<Vec<u8>>,
    palette: Option<Vec<u8>>,
) -> Result<Vec<u8>, JsValue> {
    // Load schema separately so we can surface resolution warnings to the browser console.
    let (_, warnings) =
        scouting_data_compression::common::schema::load_schema_with_warnings(schema.as_deref())
            .map_err(|e| JsValue::from_str(&e.to_string()))?;
    for w in &warnings {
        web_sys::console::warn_1(&JsValue::from_str(&format!("[schema] {}", w)));
    }

    let result = scouting_data_compression::api::encode_csv_to_image(
        csv,
        schema.as_deref(),
        palette.as_deref(),
    )
    .map_err(|e| JsValue::from_str(&e.to_string()))?;
    Ok(result.image_bytes)
}
