
use rusqlite::{params, Connection};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use uuid::Uuid;

use super::models::{Conversation, Message};

fn current_timestamp() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
}

pub fn open_db(path: &Path, key: &str) -> rusqlite::Result<Connection> {
    let mut conn = Connection::open(path)?;

    // Configure SQLCipher before accessing database tables.
    conn.pragma_update(None, "key", key)?;

    conn.pragma_update(None, "foreign_keys", "ON")?;

    // Create the conversation table.
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS conversations (
            id TEXT PRIMARY KEY,
            title TEXT NOT NULL,
            chat_number INTEGER NOT NULL UNIQUE,
            created_at INTEGER NOT NULL,
            updated_at INTEGER NOT NULL
        );
        ",
    )?;

    // Create a new messages table if this is a fresh database.
    // Existing databases retain their original table until migration.
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS messages (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            conversation_id TEXT NOT NULL,
            role TEXT NOT NULL,
            content TEXT NOT NULL,
            created_at INTEGER NOT NULL,
            FOREIGN KEY (conversation_id)
                REFERENCES conversations(id)
                ON DELETE CASCADE
        );
        ",
    )?;

    // Check whether the existing messages table needs migration.
    let mut columns = Vec::new();

    {
        let mut stmt = conn.prepare("PRAGMA table_info(messages)")?;

        let rows = stmt.query_map([], |row| {
            row.get::<_, String>(1)
        })?;

        for column in rows {
            columns.push(column?);
        }
    }

    let has_conversation_id = columns
        .iter()
        .any(|column| column == "conversation_id");

    let has_created_at = columns
        .iter()
        .any(|column| column == "created_at");

    // Perform the legacy migration transactionally.
    {
        let tx = conn.transaction()?;

        if !has_conversation_id {
            tx.execute_batch(
                "
                ALTER TABLE messages
                ADD COLUMN conversation_id TEXT
                REFERENCES conversations(id)
                ON DELETE CASCADE;
                ",
            )?;
        }

        if !has_created_at {
            tx.execute_batch(
                "
                ALTER TABLE messages
                ADD COLUMN created_at INTEGER NOT NULL DEFAULT 0;
                ",
            )?;
        }

        let existing_count: i64 = tx.query_row(
            "SELECT COUNT(*) FROM messages",
            [],
            |row| row.get(0),
        )?;

        // Move all legacy messages into one conversation.
        if existing_count > 0 {
            let unassigned_count: i64 = tx.query_row(
                "SELECT COUNT(*) FROM messages
                 WHERE conversation_id IS NULL",
                [],
                |row| row.get(0),
            )?;

            if unassigned_count > 0 {
                let now = current_timestamp();

                let legacy_id = Uuid::new_v4().to_string();

                tx.execute(
                    "
                    INSERT INTO conversations
                        (id, title, chat_number, created_at, updated_at)
                    VALUES (?1, 'Chat 001', 1, ?2, ?2)
                    ON CONFLICT(chat_number) DO NOTHING
                    ",
                    params![legacy_id, now],
                )?;

                let conversation_id: String = tx.query_row(
                    "SELECT id FROM conversations
                     WHERE chat_number = 1",
                    [],
                    |row| row.get(0),
                )?;

                tx.execute(
                    "
                    UPDATE messages
                    SET conversation_id = ?1
                    WHERE conversation_id IS NULL
                    ",
                    params![conversation_id],
                )?;

                tx.execute(
                    "
                    UPDATE messages
                    SET created_at = ?1
                    WHERE created_at = 0
                    ",
                    params![now],
                )?;
            }
        }

        tx.commit()?;
    }

    // Index messages for efficient conversation-specific retrieval.
    conn.execute_batch(
        "
        CREATE INDEX IF NOT EXISTS idx_messages_conversation
        ON messages(conversation_id, id);

        CREATE INDEX IF NOT EXISTS idx_conversations_updated
        ON conversations(updated_at DESC);
        ",
    )?;

    // Persistent chat numbering sequence.
    // Deleted conversations will not cause their numbers to be reused.
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS chat_sequence (
            id INTEGER PRIMARY KEY CHECK (id = 1),
            next_number INTEGER NOT NULL
        );

        INSERT OR IGNORE INTO chat_sequence (id, next_number)
        SELECT 1, COALESCE(MAX(chat_number), 0) + 1
        FROM conversations;
        ",
    )?;

    Ok(conn)
}

// Create a new independent conversation.
pub fn create_conversation(
    conn: &mut Connection,
) -> rusqlite::Result<Conversation> {
    let tx = conn.transaction()?;

    let chat_number: i64 = tx.query_row(
        "SELECT next_number FROM chat_sequence WHERE id = 1",
        [],
        |row| row.get(0),
    )?;

    tx.execute(
        "UPDATE chat_sequence
         SET next_number = next_number + 1
         WHERE id = 1",
        [],
    )?;

    let now = current_timestamp();

    let id = Uuid::new_v4().to_string();

    let title = format!("Chat {:03}", chat_number);

    tx.execute(
        "
        INSERT INTO conversations
            (id, title, chat_number, created_at, updated_at)
        VALUES (?1, ?2, ?3, ?4, ?4)
        ",
        params![id, title, chat_number, now],
    )?;

    tx.commit()?;

    Ok(Conversation {
        id,
        title,
        chat_number,
        created_at: now,
        updated_at: now,
    })
}

// Retrieve all conversations, most recently active first.
pub fn list_conversations(
    conn: &Connection,
) -> rusqlite::Result<Vec<Conversation>> {
    let mut stmt = conn.prepare(
        "
        SELECT id, title, chat_number, created_at, updated_at
        FROM conversations
        ORDER BY updated_at DESC, chat_number DESC
        ",
    )?;

    let rows = stmt.query_map([], |row| {
        Ok(Conversation {
            id: row.get(0)?,
            title: row.get(1)?,
            chat_number: row.get(2)?,
            created_at: row.get(3)?,
            updated_at: row.get(4)?,
        })
    })?;

    rows.collect()
}

// Retrieve messages belonging to one specific conversation.
pub fn load_history(
    conn: &Connection,
    conversation_id: &str,
) -> rusqlite::Result<Vec<Message>> {
    let mut stmt = conn.prepare(
        "
        SELECT role, content
        FROM messages
        WHERE conversation_id = ?1
        ORDER BY id ASC
        ",
    )?;

    let rows = stmt.query_map(params![conversation_id], |row| {
        Ok(Message {
            role: row.get(0)?,
            content: row.get(1)?,
        })
    })?;

    rows.collect()
}

// Save one message to a particular conversation.
pub fn save_message(
    conn: &mut Connection,
    conversation_id: &str,
    role: &str,
    content: &str,
) -> rusqlite::Result<()> {
    let tx = conn.transaction()?;

    let now = current_timestamp();

    tx.execute(
        "
        INSERT INTO messages
            (conversation_id, role, content, created_at)
        VALUES (?1, ?2, ?3, ?4)
        ",
        params![conversation_id, role, content, now],
    )?;

    let updated = tx.execute(
        "
        UPDATE conversations
        SET updated_at = ?1
        WHERE id = ?2
        ",
        params![now, conversation_id],
    )?;

    if updated == 0 {
        return Err(rusqlite::Error::QueryReturnedNoRows);
    }

    tx.commit()?;

    Ok(())
}

// Save the complete user/model exchange atomically.
pub fn save_exchange(
    conn: &mut Connection,
    conversation_id: &str,
    user_content: &str,
    model_content: &str,
) -> rusqlite::Result<()> {
    let tx = conn.transaction()?;

    let now = current_timestamp();

    // Verify that the conversation exists.
    let exists: bool = tx.query_row(
        "SELECT EXISTS(
            SELECT 1 FROM conversations WHERE id = ?1
        )",
        params![conversation_id],
        |row| row.get(0),
    )?;

    if !exists {
        return Err(rusqlite::Error::QueryReturnedNoRows);
    }

    // Save user message.
    tx.execute(
        "
        INSERT INTO messages
            (conversation_id, role, content, created_at)
        VALUES (?1, 'user', ?2, ?3)
        ",
        params![conversation_id, user_content, now],
    )?;

    // Save model response.
    tx.execute(
        "
        INSERT INTO messages
            (conversation_id, role, content, created_at)
        VALUES (?1, 'model', ?2, ?3)
        ",
        params![conversation_id, model_content, now],
    )?;

    // Update conversation activity.
    tx.execute(
        "
        UPDATE conversations
        SET updated_at = ?1
        WHERE id = ?2
        ",
        params![now, conversation_id],
    )?;

    tx.commit()?;

    Ok(())
}

// Delete a conversation and its associated messages.
pub fn delete_conversation(
    conn: &Connection,
    conversation_id: &str,
) -> rusqlite::Result<()> {
    conn.execute(
        "DELETE FROM conversations WHERE id = ?1",
        params![conversation_id],
    )?;

    Ok(())
}
