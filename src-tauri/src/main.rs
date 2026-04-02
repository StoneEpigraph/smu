#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use md5::{Md5, Digest};
use sha1::Sha1;
use sha2::Sha256;
use sha2::Sha512;
use base64::{Engine as _, engine::general_purpose};
use tauri::{
    menu::{Menu, MenuItem},
    tray::{TrayIconBuilder, TrayIconEvent, MouseButton, MouseButtonState},
    Manager, WindowEvent,
};

#[tauri::command]
async fn encode_md5(input: String, app: tauri::AppHandle) -> Result<String, String> {
    let mut hasher = Md5::new();
    hasher.update(input.as_bytes());
    let hash = format!("{:x}", hasher.finalize());
    
    // 保存到数据库
    let app_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    std::fs::create_dir_all(&app_dir).map_err(|e| e.to_string())?;
    let db_path = app_dir.join("smu.db");
    
    let conn = rusqlite::Connection::open(&db_path).map_err(|e| e.to_string())?;
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
async fn decode_md5(hash: String, app: tauri::AppHandle) -> Result<Option<String>, String> {
    let app_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let db_path = app_dir.join("smu.db");
    
    let conn = rusqlite::Connection::open(&db_path).map_err(|e| e.to_string())?;
    
    let mut stmt = conn.prepare("SELECT plaintext FROM md5_lookup WHERE hash = ?")
        .map_err(|e| e.to_string())?;
    
    let result = stmt.query_row([&hash], |row| row.get(0)).ok();
    
    Ok(result)
}

#[tauri::command]
fn encode_sha1(input: &str) -> String {
    let mut hasher = Sha1::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

#[tauri::command]
fn encode_sha256(input: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

#[tauri::command]
fn encode_sha512(input: &str) -> String {
    let mut hasher = Sha512::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

#[tauri::command]
fn encode_base64(input: &str) -> String {
    general_purpose::STANDARD.encode(input.as_bytes())
}

#[tauri::command]
fn decode_base64(input: &str) -> Result<String, String> {
    match general_purpose::STANDARD.decode(input) {
        Ok(bytes) => String::from_utf8(bytes).map_err(|e| e.to_string()),
        Err(e) => Err(e.to_string())
    }
}

#[tauri::command]
fn encode_url(input: &str) -> String {
    urlencoding::encode(input).to_string()
}

#[tauri::command]
fn decode_url(input: &str) -> Result<String, String> {
    urlencoding::decode(input).map(|s| s.to_string()).map_err(|e| e.to_string())
}

#[tauri::command]
fn encode_hex(input: &str) -> String {
    hex::encode(input)
}

#[tauri::command]
fn decode_hex(input: &str) -> Result<String, String> {
    hex::decode(input).map_err(|e| e.to_string())
        .and_then(|bytes| String::from_utf8(bytes).map_err(|e| e.to_string()))
}

#[tauri::command]
fn exit_app(app: tauri::AppHandle) {
    app.exit(0);
}

#[tauri::command]
async fn add_note(content: String, app: tauri::AppHandle) -> Result<i64, String> {
    let app_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    std::fs::create_dir_all(&app_dir).map_err(|e| e.to_string())?;
    let db_path = app_dir.join("smu.db");
    
    let conn = rusqlite::Connection::open(&db_path).map_err(|e| e.to_string())?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS quick_notes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            content TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    ).map_err(|e| e.to_string())?;
    
    conn.execute(
        "INSERT INTO quick_notes (content) VALUES (?)",
        [&content],
    ).map_err(|e| e.to_string())?;
    
    Ok(conn.last_insert_rowid())
}

#[tauri::command]
async fn get_notes(app: tauri::AppHandle) -> Result<Vec<(i64, String, String)>, String> {
    let app_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let db_path = app_dir.join("smu.db");
    
    let conn = rusqlite::Connection::open(&db_path).map_err(|e| e.to_string())?;
    
    conn.execute(
        "CREATE TABLE IF NOT EXISTS quick_notes (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            content TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    ).map_err(|e| e.to_string())?;
    
    let mut stmt = conn.prepare("SELECT id, content, created_at FROM quick_notes ORDER BY created_at DESC LIMIT 10")
        .map_err(|e| e.to_string())?;
    
    let rows = stmt.query_map([], |row| {
        Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?, row.get::<_, String>(2)?))
    }).map_err(|e| e.to_string())?;
    
    let mut result = Vec::new();
    for row in rows {
        result.push(row.map_err(|e| e.to_string())?);
    }
    
    Ok(result)
}

#[tauri::command]
async fn delete_note(id: i64, app: tauri::AppHandle) -> Result<(), String> {
    let app_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let db_path = app_dir.join("smu.db");
    
    let conn = rusqlite::Connection::open(&db_path).map_err(|e| e.to_string())?;
    conn.execute("DELETE FROM quick_notes WHERE id = ?", [id])
        .map_err(|e| e.to_string())?;
    
    Ok(())
}

#[tauri::command]
async fn increment_use_count(plugin_id: String, app: tauri::AppHandle) -> Result<(), String> {
    let app_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    std::fs::create_dir_all(&app_dir).map_err(|e| e.to_string())?;
    let db_path = app_dir.join("smu.db");
    
    let conn = rusqlite::Connection::open(&db_path).map_err(|e| e.to_string())?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS plugin_use_count (
            plugin_id TEXT PRIMARY KEY,
            use_count INTEGER DEFAULT 0
        )",
        [],
    ).map_err(|e| e.to_string())?;
    
    conn.execute(
        "INSERT INTO plugin_use_count (plugin_id, use_count) VALUES (?, 1)
         ON CONFLICT(plugin_id) DO UPDATE SET use_count = use_count + 1",
        [&plugin_id],
    ).map_err(|e| e.to_string())?;
    
    Ok(())
}

#[tauri::command]
async fn get_use_counts(app: tauri::AppHandle) -> Result<Vec<(String, i64)>, String> {
    let app_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let db_path = app_dir.join("smu.db");
    
    let conn = rusqlite::Connection::open(&db_path).map_err(|e| e.to_string())?;
    let mut stmt = conn.prepare("SELECT plugin_id, use_count FROM plugin_use_count")
        .map_err(|e| e.to_string())?;
    
    let rows = stmt.query_map([], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?))
    }).map_err(|e| e.to_string())?;
    
    let mut result = Vec::new();
    for row in rows {
        result.push(row.map_err(|e| e.to_string())?);
    }
    
    Ok(result)
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_sql::Builder::new().build())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .invoke_handler(tauri::generate_handler![
            encode_md5,
            decode_md5,
            encode_sha1,
            encode_sha256,
            encode_sha512,
            encode_base64,
            decode_base64,
            encode_url,
            decode_url,
            encode_hex,
            decode_hex,
            exit_app,
            increment_use_count,
            get_use_counts,
            add_note,
            get_notes,
            delete_note
        ])
        .setup(|app| {
            let show_item = MenuItem::with_id(app, "show", "显示", true, None::<&str>)?;
            let hide_item = MenuItem::with_id(app, "hide", "隐藏", true, None::<&str>)?;
            let quit_item = MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
            let menu = Menu::with_items(app, &[&show_item, &hide_item, &quit_item])?;
            
            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .tooltip("SMU 工具箱")
                .on_menu_event(|app, event| {
                    match event.id.as_ref() {
                        "show" => {
                            if let Some(window) = app.get_webview_window("main") {
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                        "hide" => {
                            if let Some(window) = app.get_webview_window("main") {
                                let _ = window.hide();
                            }
                        }
                        "quit" => {
                            app.exit(0);
                        }
                        _ => {}
                    }
                })
                .on_tray_icon_event(|tray, event| {
                    if let TrayIconEvent::Click { button: MouseButton::Left, button_state: MouseButtonState::Up, .. } = event {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    }
                })
                .build(app)?;
            
            Ok(())
        })
        .on_window_event(|window, event| {
            if let WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();
                let _ = window.hide();
            }
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}