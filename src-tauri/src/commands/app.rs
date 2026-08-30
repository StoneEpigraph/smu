use serde::{Deserialize, Serialize};
use tauri::{AppHandle, State};
use tauri_plugin_notification::NotificationExt;

use crate::db::Db;
use crate::error::AppResult;

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
pub async fn increment_use_count(plugin_id: String, db: State<'_, Db>) -> AppResult<()> {
    let conn = db.conn()?;
    conn.execute(
        "INSERT INTO plugin_use_count (plugin_id, use_count) VALUES (?, 1)
         ON CONFLICT(plugin_id) DO UPDATE SET use_count = use_count + 1",
        [&plugin_id],
    )?;
    Ok(())
}

#[tauri::command]
pub async fn get_use_counts(db: State<'_, Db>) -> AppResult<Vec<(String, i64)>> {
    let conn = db.conn()?;
    let mut stmt = conn.prepare("SELECT plugin_id, use_count FROM plugin_use_count")?;

    let rows = stmt.query_map([], |row| {
        Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?))
    })?;

    let mut result = Vec::new();
    for row in rows {
        result.push(row?);
    }

    Ok(result)
}

#[tauri::command]
pub async fn save_settings(settings_json: String, db: State<'_, Db>) -> AppResult<()> {
    let conn = db.conn()?;
    conn.execute(
        "INSERT OR REPLACE INTO app_settings (key, value) VALUES ('settings', ?)",
        [&settings_json],
    )?;
    Ok(())
}

#[tauri::command]
pub async fn load_settings(db: State<'_, Db>) -> AppResult<String> {
    let conn = db.conn()?;

    let result: Result<String, rusqlite::Error> = conn.query_row(
        "SELECT value FROM app_settings WHERE key = 'settings'",
        [],
        |row| row.get(0),
    );

    match result {
        Ok(settings) => Ok(settings),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok("null".to_string()),
        Err(e) => Err(e.into()),
    }
}

#[tauri::command]
pub async fn add_calendar_todo(
    event_date: String,
    event_time: String,
    event_desc: String,
    db: State<'_, Db>,
) -> AppResult<i64> {
    let conn = db.conn()?;

    conn.execute(
        "INSERT INTO calendar_events (event_date, event_time, event_desc, notified) VALUES (?, ?, ?, 0)",
        rusqlite::params![event_date, event_time, event_desc],
    )?;

    Ok(conn.last_insert_rowid())
}

#[tauri::command]
pub async fn get_calendar_todos(
    event_date: String,
    db: State<'_, Db>,
) -> AppResult<Vec<TodoItem>> {
    let conn = db.conn()?;

    let mut stmt = conn.prepare(
        "SELECT id, event_date, event_time, event_desc, notified FROM calendar_events WHERE event_date = ? ORDER BY event_time ASC"
    )?;

    let todos = stmt.query_map([&event_date], |row| {
        Ok(TodoItem {
            id: row.get(0)?,
            event_date: row.get(1)?,
            event_time: row.get(2)?,
            event_desc: row.get(3)?,
            notified: row.get(4)?,
        })
    })?;

    let mut result = Vec::new();
    for todo in todos {
        result.push(todo?);
    }

    Ok(result)
}

#[tauri::command]
pub async fn update_calendar_todo(
    id: i64,
    event_time: String,
    event_desc: String,
    db: State<'_, Db>,
) -> AppResult<()> {
    let conn = db.conn()?;

    conn.execute(
        "UPDATE calendar_events SET event_time = ?, event_desc = ? WHERE id = ?",
        rusqlite::params![event_time, event_desc, id],
    )?;

    Ok(())
}

#[tauri::command]
pub async fn delete_calendar_todo(id: i64, db: State<'_, Db>) -> AppResult<()> {
    let conn = db.conn()?;
    conn.execute("DELETE FROM calendar_events WHERE id = ?", [id])?;
    Ok(())
}

#[tauri::command]
pub async fn check_due_reminders(app: AppHandle, db: State<'_, Db>) -> AppResult<Vec<TodoItem>> {
    let conn = db.conn()?;

    let now = chrono::Local::now();
    let today = now.format("%Y-%m-%d").to_string();
    let current_time = now.format("%H:%M").to_string();

    let mut stmt = conn.prepare(
        "SELECT id, event_date, event_time, event_desc, notified FROM calendar_events WHERE event_date = ? AND notified = 0 AND event_time <= ? AND event_desc != ''"
    )?;

    let todos = stmt.query_map(rusqlite::params![today, current_time], |row| {
        Ok(TodoItem {
            id: row.get(0)?,
            event_date: row.get(1)?,
            event_time: row.get(2)?,
            event_desc: row.get(3)?,
            notified: row.get(4)?,
        })
    })?;

    let mut result = Vec::new();
    for todo in todos {
        result.push(todo?);
    }

    for todo in &result {
        conn.execute(
            "UPDATE calendar_events SET notified = 1 WHERE id = ?",
            [todo.id],
        )?;

        if let Err(e) = app.notification()
            .builder()
            .title("日历提醒")
            .body(&todo.event_desc)
            .show() {
            eprintln!("Failed to send notification: {}", e);
        }
    }

    Ok(result)
}
