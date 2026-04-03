use serde_json::Value;

#[tauri::command]
pub fn format_json(json: String) -> Result<String, String> {
    let value: Value = serde_json::from_str(&json).map_err(|e| e.to_string())?;
    serde_json::to_string_pretty(&value).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn minify_json(json: String) -> Result<String, String> {
    let value: Value = serde_json::from_str(&json).map_err(|e| e.to_string())?;
    serde_json::to_string(&value).map_err(|e| e.to_string())
}