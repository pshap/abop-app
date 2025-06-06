use std::sync::Arc;
use tokio::sync::Mutex;
use rusqlite::{Connection, Transaction};
use tracing::{debug, error, info};

use crate::error::AppError;
use crate::models::audiobook::Audiobook;

pub struct AudiobookRepository {
    conn: Arc<Mutex<Connection>>,
}

impl AudiobookRepository {
    pub fn new(conn: Connection) -> Self {
        Self {
            conn: Arc::new(Mutex::new(conn)),
        }
    }

    pub async fn save(&self, audiobook: &Audiobook) -> Result<(), AppError> {
        let conn = self.conn.lock().await;
        let tx = conn.transaction()?;

        self.save_with_transaction(&tx, audiobook)?;
        tx.commit()?;

        Ok(())
    }

    pub async fn save_batch(&self, audiobooks: &[Audiobook]) -> Result<(), AppError> {
        let conn = self.conn.lock().await;
        let tx = conn.transaction()?;

        for audiobook in audiobooks {
            self.save_with_transaction(&tx, audiobook)?;
        }

        tx.commit()?;
        Ok(())
    }

    fn save_with_transaction(&self, tx: &Transaction, audiobook: &Audiobook) -> Result<(), AppError> {
        // Check if audiobook already exists
        let exists: bool = tx.query_row(
            "SELECT EXISTS(SELECT 1 FROM audiobooks WHERE path = ?)",
            [audiobook.path.to_string_lossy().to_string()],
            |row| row.get(0),
        )?;

        if exists {
            // Update existing audiobook
            tx.execute(
                "UPDATE audiobooks SET 
                    title = ?,
                    author = ?,
                    duration = ?,
                    file_size = ?,
                    last_modified = ?,
                    metadata = ?
                WHERE path = ?",
                (
                    audiobook.title,
                    audiobook.author,
                    audiobook.duration,
                    audiobook.file_size,
                    audiobook.last_modified,
                    serde_json::to_string(&audiobook.metadata)?,
                    audiobook.path.to_string_lossy().to_string(),
                ),
            )?;
        } else {
            // Insert new audiobook
            tx.execute(
                "INSERT INTO audiobooks (
                    path, title, author, duration, file_size, last_modified, metadata
                ) VALUES (?, ?, ?, ?, ?, ?, ?)",
                (
                    audiobook.path.to_string_lossy().to_string(),
                    audiobook.title,
                    audiobook.author,
                    audiobook.duration,
                    audiobook.file_size,
                    audiobook.last_modified,
                    serde_json::to_string(&audiobook.metadata)?,
                ),
            )?;
        }

        Ok(())
    }

    pub async fn get_by_path(&self, path: &str) -> Result<Option<Audiobook>, AppError> {
        let conn = self.conn.lock().await;
        let mut stmt = conn.prepare(
            "SELECT path, title, author, duration, file_size, last_modified, metadata 
             FROM audiobooks WHERE path = ?"
        )?;

        let audiobook = stmt.query_row([path], |row| {
            Ok(Audiobook {
                path: std::path::PathBuf::from(row.get::<_, String>(0)?),
                title: row.get(1)?,
                author: row.get(2)?,
                duration: row.get(3)?,
                file_size: row.get(4)?,
                last_modified: row.get(5)?,
                metadata: serde_json::from_str(&row.get::<_, String>(6)?).unwrap_or_default(),
            })
        }).optional()?;

        Ok(audiobook)
    }

    pub async fn get_all(&self) -> Result<Vec<Audiobook>, AppError> {
        let conn = self.conn.lock().await;
        let mut stmt = conn.prepare(
            "SELECT path, title, author, duration, file_size, last_modified, metadata 
             FROM audiobooks"
        )?;

        let audiobooks = stmt.query_map([], |row| {
            Ok(Audiobook {
                path: std::path::PathBuf::from(row.get::<_, String>(0)?),
                title: row.get(1)?,
                author: row.get(2)?,
                duration: row.get(3)?,
                file_size: row.get(4)?,
                last_modified: row.get(5)?,
                metadata: serde_json::from_str(&row.get::<_, String>(6)?).unwrap_or_default(),
            })
        })?
        .collect::<Result<Vec<_>, _>>()?;

        Ok(audiobooks)
    }

    pub async fn delete(&self, path: &str) -> Result<(), AppError> {
        let conn = self.conn.lock().await;
        conn.execute("DELETE FROM audiobooks WHERE path = ?", [path])?;
        Ok(())
    }

    pub async fn delete_batch(&self, paths: &[String]) -> Result<(), AppError> {
        let conn = self.conn.lock().await;
        let tx = conn.transaction()?;

        for path in paths {
            tx.execute("DELETE FROM audiobooks WHERE path = ?", [path])?;
        }

        tx.commit()?;
        Ok(())
    }
} 