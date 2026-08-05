use md5::{Md5, Digest};
use sha1::Sha1;
use sha2::{Sha256, Sha512};
use sm3::Sm3;
use base64::{Engine as _, engine::general_purpose};
use rusqlite::Connection;
use smcrypto::sm3 as smcrypto_sm3;
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
        [hash.to_lowercase(), input],
    ).map_err(|e| e.to_string())?;
    
    Ok(hash)
}

#[tauri::command]
pub async fn decode_md5(hash: String, app: AppHandle) -> Result<Option<String>, String> {
    let app_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let db_path = app_dir.join("smu.db");
    
    let conn = Connection::open(&db_path).map_err(|e| e.to_string())?;
    
    let mut stmt = conn.prepare("SELECT plaintext FROM md5_lookup WHERE LOWER(hash) = LOWER(?)")
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
pub fn encode_sm3(input: &str) -> String {
    smcrypto_sm3::sm3_hash(input.as_bytes())
}

/// 对应 Java Sm3Utils/SM3Digest 密码哈希方案：
/// Base64( SM3( UTF8( hex( SM3(input) ) ) ) )
#[tauri::command]
pub fn encode_sm3_hash(input: &str) -> String {
    // Sm3Utils.encrypt：第一次 SM3，返回十六进制字符串
    let hex_hash = smcrypto_sm3::sm3_hash(input.as_bytes());
    // SM3Digest：对十六进制字符串的 UTF-8 字节再计算一次，取原始 32 字节摘要
    let mut hasher = Sm3::new();
    hasher.update(hex_hash.as_bytes());
    let final_hash = hasher.finalize();
    // Base64 编码（对应 Java Base64.getEncoder().encodeToString）
    general_purpose::STANDARD.encode(final_hash)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_sm3_matches_standard_vector() {
        // 国密标准测试向量：SM3("abc")
        assert_eq!(
            encode_sm3("abc"),
            "66c7f0f462eeedd9d1f2d46bdc10e4e24167c4875cf2f7a2297da02b8f4ba8e0"
        );
    }

    #[test]
    fn test_encode_sm3_hash_shape() {
        // 对应 Java: Base64( SM3( UTF8( hex( SM3(input) ) ) ) )
        // SM3("abc") = 66c7f0f462eeedd9d1f2d46bdc10e4e24167c4875cf2f7a2297da02b8f4ba8e0
        // 再对该十六进制字符串做 SM3，最后 Base64 编码原始 32 字节摘要
        assert_eq!(
            encode_sm3_hash("abc"),
            "DNmPGZthuVEanlyxNV3l8XoIDVKNulmrrM5rtVoJvic="
        );
    }
}