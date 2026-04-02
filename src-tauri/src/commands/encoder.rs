use md5::{Md5, Digest};
use sha1::Sha1;
use sha2::{Sha256, Sha512};
use base64::{Engine as _, engine::general_purpose};
use rusqlite::Connection;
use tauri::{AppHandle, Manager};

#[tauri::command]
pub async fn encode_md5(input: String, app: AppHandle) -> Result<String, String> {
    let mut hasher = Md5::new();
    hasher.update(input.as_bytes());
    let hash = format!("{:x}", hasher.finalize());
    
    let app_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    std::fs::create_dir_all(&app_dir).map_err(|e| e.to_string())?;
    let db_path = app_dir.join("smu.db");
    
    let conn = Connection::open(&db_path).map_err(|e| e.to_string())?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS md5_lookup (
            hash TEXT PRIMARY KEY,
            plaintext TEXT NOT NULL
        )",
        [],
    ).map_err(|e| e.to_string())?;
    
    conn.execute(
        "INSERT OR REPLACE INTO md5_lookup (hash, plaintext) VALUES (?, ?)",
        [&hash, &input],
    ).map_err(|e| e.to_string())?;
    
    Ok(hash)
}

#[tauri::command]
pub async fn decode_md5(hash: String, app: AppHandle) -> Result<Option<String>, String> {
    let app_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let db_path = app_dir.join("smu.db");
    
    let conn = Connection::open(&db_path).map_err(|e| e.to_string())?;
    
    let mut stmt = conn.prepare("SELECT plaintext FROM md5_lookup WHERE hash = ?")
        .map_err(|e| e.to_string())?;
    
    let result = stmt.query_row([&hash], |row| row.get(0)).ok();
    
    Ok(result)
}

#[tauri::command]
pub fn encode_sha1(input: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

#[tauri::command]
pub fn encode_sha256(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

#[tauri::command]
pub fn encode_sha512(input: &str) -> String {
    let mut hasher = Sha512::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

#[tauri::command]
pub fn encode_base64(input: &str) -> String {
    general_purpose::STANDARD.encode(input.as_bytes())
}

#[tauri::command]
pub fn decode_base64(input: &str) -> Result<String, String> {
    match general_purpose::STANDARD.decode(input) {
        Ok(bytes) => String::from_utf8(bytes).map_err(|e| e.to_string()),
        Err(e) => Err(e.to_string())
    }
}

#[tauri::command]
pub fn encode_url(input: &str) -> String {
    urlencoding::encode(input).to_string()
}

#[tauri::command]
pub fn decode_url(input: &str) -> Result<String, String> {
    urlencoding::decode(input).map(|s| s.to_string()).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn encode_hex(input: &str) -> String {
    hex::encode(input)
}

#[tauri::command]
pub fn decode_hex(input: &str) -> Result<String, String> {
    hex::decode(input).map_err(|e| e.to_string())
        .and_then(|bytes| String::from_utf8(bytes).map_err(|e| e.to_string()))
}