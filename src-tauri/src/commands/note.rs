use tauri::State;

use crate::db::Db;
use crate::error::AppResult;

#[tauri::command]
pub async fn add_note(content: String, db: State<'_, Db>) -> AppResult<i64> {
    let conn = db.conn()?;
    conn.execute("INSERT INTO quick_notes (content) VALUES (?)", [&content])?;
    Ok(conn.last_insert_rowid())
}

#[tauri::command]
pub async fn get_notes(db: State<'_, Db>) -> AppResult<Vec<(i64, String, String)>> {
    let conn = db.conn()?;

    let mut stmt = conn.prepare(
        "SELECT id, content, created_at FROM quick_notes ORDER BY created_at DESC LIMIT 10",
    )?;

    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i64>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, String>(2)?,
        ))
    })?;

    let mut result = Vec::new();
    for row in rows {
        result.push(row?);
    }

    Ok(result)
}

#[tauri::command]
pub async fn delete_note(id: i64, db: State<'_, Db>) -> AppResult<()> {
    let conn = db.conn()?;
    conn.execute("DELETE FROM quick_notes WHERE id = ?", [id])?;
    Ok(())
}
