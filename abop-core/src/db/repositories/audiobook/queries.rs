//! SQL query definitions for audiobook operations

/// Audiobook table column names for consistent querying
#[allow(missing_docs)]
pub mod columns {
    pub const ID: &str = "id";
    pub const LIBRARY_ID: &str = "library_id";
    pub const PATH: &str = "path";
    pub const TITLE: &str = "title";
    pub const AUTHOR: &str = "author";
    pub const NARRATOR: &str = "narrator";
    pub const DESCRIPTION: &str = "description";
    pub const DURATION_SECONDS: &str = "duration_seconds";
    pub const SIZE_BYTES: &str = "size_bytes";
    pub const COVER_ART: &str = "cover_art";
    pub const CREATED_AT: &str = "created_at";
    pub const UPDATED_AT: &str = "updated_at";
    pub const SELECTED: &str = "selected";
}

/// Complete audiobook SELECT query with all columns
pub const SELECT_ALL_COLUMNS: &str = "
    SELECT id, library_id, path, title, author, narrator, description,
           duration_seconds, size_bytes, cover_art, created_at, updated_at, selected
    FROM audiobooks
";

/// UPSERT query for inserting or updating audiobooks
pub const UPSERT_AUDIOBOOK: &str = "
    INSERT INTO audiobooks (
        id, library_id, path, title, author, narrator, description,
        duration_seconds, size_bytes, cover_art, created_at, updated_at, selected
    ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)
    ON CONFLICT(id) DO UPDATE SET
        library_id = excluded.library_id,
        path = excluded.path,
        title = excluded.title,
        author = excluded.author,
        narrator = excluded.narrator,
        description = excluded.description,
        duration_seconds = excluded.duration_seconds,
        size_bytes = excluded.size_bytes,
        cover_art = excluded.cover_art,
        updated_at = excluded.updated_at,
        selected = excluded.selected
";

/// UPDATE query for existing audiobooks
pub const UPDATE_AUDIOBOOK: &str = "
    UPDATE audiobooks SET
        library_id = ?2,
        path = ?3,
        title = ?4,
        author = ?5,
        narrator = ?6,
        description = ?7,
        duration_seconds = ?8,
        size_bytes = ?9,
        cover_art = ?10,
        updated_at = ?11,
        selected = ?12
    WHERE id = ?1
";

/// DELETE queries
#[allow(missing_docs)]
pub const DELETE_BY_ID: &str = "DELETE FROM audiobooks WHERE id = ?1";
#[allow(missing_docs)]
pub const DELETE_BY_LIBRARY_ID: &str = "DELETE FROM audiobooks WHERE library_id = ?1";

/// WHERE clause conditions for common queries
#[allow(missing_docs)]
pub mod where_clauses {
    pub const BY_ID: &str = "WHERE id = ?1";
    pub const BY_LIBRARY_ID: &str = "WHERE library_id = ?1";
    pub const BY_PATH: &str = "WHERE path = ?1";
    pub const BY_AUTHOR_LIKE: &str = "WHERE author LIKE ?1";
    pub const EXISTS_BY_ID: &str = "WHERE id = ?1";
}

/// ORDER BY clauses for consistent sorting
#[allow(missing_docs)]
pub mod order_by {
    pub const BY_TITLE_ASC: &str = "ORDER BY title ASC";
    pub const BY_TITLE_ASC_LIMIT_OFFSET: &str = "ORDER BY title ASC LIMIT ?2 OFFSET ?3";
    // SQLite requires LIMIT before OFFSET; -1 means "no limit"
    pub const BY_TITLE_ASC_OFFSET: &str = "ORDER BY title ASC LIMIT -1 OFFSET ?2";
}

/// COUNT queries
#[allow(missing_docs)]
pub const COUNT_BY_LIBRARY_ID: &str = "SELECT COUNT(*) FROM audiobooks WHERE library_id = ?1";
#[allow(missing_docs)]
pub const EXISTS_BY_ID: &str = "SELECT EXISTS(SELECT 1 FROM audiobooks WHERE id = ?1)";
