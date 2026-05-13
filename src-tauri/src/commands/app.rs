use rusqlite::Connection;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Manager};

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoItem {
    pub id: i64,
    pub event_date: String,
    pub event_time: String,
    pub event_desc: String,
    pub notified: i32,
}

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
        }
    }
}

#[tauri::command]
pub async fn init_calendar_table(app: AppHandle) -> Result<(), String> {
    let app_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    std::fs::create_dir_all(&app_dir).map_err(|e| e.to_string())?;
    let db_path = app_dir.join("smu.db");
    
    let conn = Connection::open(&db_path).map_err(|e| e.to_string())?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS calendar_events (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            event_date TEXT NOT NULL,
            event_time TEXT DEFAULT '',
            event_desc TEXT DEFAULT '',
            notified INTEGER DEFAULT 0,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP
        )",
        [],
    ).map_err(|e| e.to_string())?;
    
    println!("Calendar table initialized");
    Ok(())
}

#[tauri::command]
pub async fn add_calendar_todo(
    event_date: String,
    event_time: String,
    event_desc: String,
    app: AppHandle,
) -> Result<i64, String> {
    let app_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    std::fs::create_dir_all(&app_dir).map_err(|e| e.to_string())?;
    let db_path = app_dir.join("smu.db");
    
    let conn = Connection::open(&db_path).map_err(|e| e.to_string())?;
    
    conn.execute(
        "INSERT INTO calendar_events (event_date, event_time, event_desc, notified) VALUES (?, ?, ?, 0)",
        rusqlite::params![event_date, event_time, event_desc],
    ).map_err(|e| e.to_string())?;
    
    let id = conn.last_insert_rowid();
    println!("Added todo with id: {}", id);
    Ok(id)
}

#[tauri::command]
pub async fn get_calendar_todos(
    event_date: String,
    app: AppHandle,
) -> Result<Vec<TodoItem>, String> {
    let app_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let db_path = app_dir.join("smu.db");
    
    let conn = Connection::open(&db_path).map_err(|e| e.to_string())?;
    
    let mut stmt = conn.prepare(
        "SELECT id, event_date, event_time, event_desc, notified FROM calendar_events WHERE event_date = ? ORDER BY event_time ASC"
    ).map_err(|e| e.to_string())?;
    
    let todos = stmt.query_map([&event_date], |row| {
        Ok(TodoItem {
            id: row.get(0)?,
            event_date: row.get(1)?,
            event_time: row.get(2)?,
            event_desc: row.get(3)?,
            notified: row.get(4)?,
        })
    }).map_err(|e| e.to_string())?;
    
    let mut result = Vec::new();
    for todo in todos {
        result.push(todo.map_err(|e| e.to_string())?);
    }
    
    println!("Loaded {} todos for date {}", result.len(), event_date);
    Ok(result)
}

#[tauri::command]
pub async fn update_calendar_todo(
    id: i64,
    event_time: String,
    event_desc: String,
    app: AppHandle,
) -> Result<(), String> {
    let app_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let db_path = app_dir.join("smu.db");
    
    let conn = Connection::open(&db_path).map_err(|e| e.to_string())?;
    
    conn.execute(
        "UPDATE calendar_events SET event_time = ?, event_desc = ? WHERE id = ?",
        rusqlite::params![event_time, event_desc, id],
    ).map_err(|e| e.to_string())?;
    
    println!("Updated todo id: {}", id);
    Ok(())
}

#[tauri::command]
pub async fn delete_calendar_todo(
    id: i64,
    app: AppHandle,
) -> Result<(), String> {
    let app_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let db_path = app_dir.join("smu.db");
    
    let conn = Connection::open(&db_path).map_err(|e| e.to_string())?;
    
    conn.execute(
        "DELETE FROM calendar_events WHERE id = ?",
        [id],
    ).map_err(|e| e.to_string())?;
    
    println!("Deleted todo id: {}", id);
    Ok(())
}

#[tauri::command]
pub async fn check_due_reminders(app: AppHandle) -> Result<Vec<TodoItem>, String> {
    let app_dir = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let db_path = app_dir.join("smu.db");
    
    let conn = Connection::open(&db_path).map_err(|e| e.to_string())?;
    
    let now = chrono::Local::now();
    let today = now.format("%Y-%m-%d").to_string();
    let current_time = now.format("%H:%M").to_string();
    
    let mut stmt = conn.prepare(
        "SELECT id, event_date, event_time, event_desc, notified FROM calendar_events WHERE event_date = ? AND notified = 0 AND event_time <= ? AND event_desc != ''"
    ).map_err(|e| e.to_string())?;
    
    let todos = stmt.query_map(rusqlite::params![today, current_time], |row| {
        Ok(TodoItem {
            id: row.get(0)?,
            event_date: row.get(1)?,
            event_time: row.get(2)?,
            event_desc: row.get(3)?,
            notified: row.get(4)?,
        })
    }).map_err(|e| e.to_string())?;
    
    let mut result = Vec::new();
    for todo in todos {
        result.push(todo.map_err(|e| e.to_string())?);
    }
    
    for todo in &result {
        conn.execute(
            "UPDATE calendar_events SET notified = 1 WHERE id = ?",
            [todo.id],
        ).map_err(|e| e.to_string())?;
    }
    
    Ok(result)}