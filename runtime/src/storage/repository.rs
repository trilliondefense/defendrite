use rusqlite::{params, Connection};
use std::path::Path;

use super::models::Message;

pub fn open_db(path: &Path, key: &str) -> rusqlite::Result<Connection> {
    let conn = Connection::open(path)?;

    // Set the SQLCipher encryption key before querying the database.
    conn.pragma_update(None, "key", key)?;

    conn.execute_batch(
        "CREATE TABLE IF NOT EXISTS messages (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            role TEXT NOT NULL,
            content TEXT NOT NULL
        );",
    )?;

    Ok(conn)
}

pub fn load_history(conn: &Connection) -> rusqlite::Result<Vec<Message>> {
    let mut stmt = conn.prepare("SELECT role, content FROM messages ORDER BY id")?;

    let rows = stmt.query_map([], |row| {
        Ok(Message {
            role: row.get(0)?,
            content: row.get(1)?,
        })
    })?;

    rows.collect()
}

pub fn save_message(conn: &Connection, role: &str, content: &str) -> rusqlite::Result<()> {
    conn.execute(
        "INSERT INTO messages (role, content) VALUES (?1, ?2)",
        params![role, content],
    )?;

    Ok(())
}
