INSERT OR REPLACE INTO audiobooks (
    id, 
    library_id, 
    path, 
    title, 
    author, 
    narrator, 
    description, 
    duration_seconds, 
    size_bytes, 
    cover_art,
    created_at,
    updated_at
) VALUES (
    ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10,
    COALESCE((SELECT created_at FROM audiobooks WHERE id = ?1), CURRENT_TIMESTAMP),
    CURRENT_TIMESTAMP
)
