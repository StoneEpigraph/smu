use std::sync::{Mutex, MutexGuard};

use rusqlite::Connection;
use tauri::{AppHandle, Manager};

use crate::error::{AppError, AppResult};

/// 全局 SQLite 连接，setup 时创建并 manage，所有命令共享
pub struct Db(Mutex<Connection>);

impl Db {
    pub fn conn(&self) -> AppResult<MutexGuard<'_, Connection>> {
        self.0.lock().map_err(|_| AppError::Lock)
    }
}

const SCHEMA: &str = "
CREATE TABLE IF NOT EXISTS plugin_use_count (
    plugin_id TEXT PRIMARY KEY,
    use_count INTEGER DEFAULT 0
);
CREATE TABLE IF NOT EXISTS app_settings (
    key TEXT PRIMARY KEY,
    value TEXT NOT NULL
);
CREATE TABLE IF NOT EXISTS calendar_events (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    event_date TEXT NOT NULL,
    event_time TEXT DEFAULT '',
    event_desc TEXT DEFAULT '',
    notified INTEGER DEFAULT 0,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
CREATE TABLE IF NOT EXISTS quick_notes (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    content TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);
CREATE TABLE IF NOT EXISTS md5_lookup (
    hash TEXT PRIMARY KEY,
    plaintext TEXT NOT NULL
);
";

pub fn init(app: &AppHandle) -> AppResult<Db> {
    let dir = app.path().app_data_dir()?;
    std::fs::create_dir_all(&dir)?;
    let conn = Connection::open(dir.join("smu.db"))?;
    conn.execute_batch(SCHEMA)?;
    Ok(Db(Mutex::new(conn)))
}
