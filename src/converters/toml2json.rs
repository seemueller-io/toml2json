use crate::wasm_bindgen::JsValue;

pub fn convert_toml_to_json(input: &str) -> Result<String, JsValue> {
    let val: toml::Value = toml::from_str(input)
        .map_err(|err| JsValue::from_str(&err.to_string()))?;

    let serde_json = serde_json::to_string_pretty(&val)
        .map_err(|err| JsValue::from_str(&err.to_string()))?;

    Ok(serde_json)
}