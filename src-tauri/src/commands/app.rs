use rusqlite::Connection;
use tauri::{AppHandle, Manager};

#[tauri::command]
pub fn exit_app(app: AppHandle) {
    app.exit(0);
}

#[tauri::command]
pub fn get_app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

#[tauri::command]
pub async fn increment_use_count(plugin_id: String, app: AppHandle) -> Result<(), String> {
    let app_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    std::fs::create_dir_all(&app_dir).map_err(|e| e.to_string())?;
    let db_path = app_dir.join("smu.db");
    
    let conn = Connection::open(&db_path).map_err(|e| e.to_string())?;
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
pub async fn get_use_counts(app: AppHandle) -> Result<Vec<(String, i64)>, String> {
    let app_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let db_path = app_dir.join("smu.db");
    
    let conn = Connection::open(&db_path).map_err(|e| e.to_string())?;
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

#[tauri::command]
pub async fn save_settings(settings_json: String, app: AppHandle) -> Result<(), String> {
    println!("Saving settings: {}", settings_json);
    let app_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    std::fs::create_dir_all(&app_dir).map_err(|e| e.to_string())?;
    let db_path = app_dir.join("smu.db");
    println!("Database path: {:?}", db_path);
    
    let conn = Connection::open(&db_path).map_err(|e| e.to_string())?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS app_settings (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        )",
        [],
    ).map_err(|e| e.to_string())?;
    
    conn.execute(
        "INSERT OR REPLACE INTO app_settings (key, value) VALUES ('settings', ?)",
        [&settings_json],
    ).map_err(|e| e.to_string())?;
    
    println!("Settings saved successfully");
    Ok(())
}

#[tauri::command]
pub async fn load_settings(app: AppHandle) -> Result<String, String> {
    println!("Loading settings...");
    let app_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let db_path = app_dir.join("smu.db");
    println!("Database path: {:?}", db_path);
    
    let conn = Connection::open(&db_path).map_err(|e| e.to_string())?;
    
    let result: Result<String, rusqlite::Error> = conn.query_row(
        "SELECT value FROM app_settings WHERE key = 'settings'",
        [],
        |row| row.get(0),
    );
    
    match result {
        Ok(settings) => {
            println!("Settings loaded: {}", settings);
            Ok(settings)
        },
        Err(e) => {
            println!("No settings found: {}", e);
            Ok("null".to_string())
        },
    }
}