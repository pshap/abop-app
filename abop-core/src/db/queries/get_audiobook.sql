SELECT 
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
FROM 
    audiobooks 
WHERE 
    id = ?1
