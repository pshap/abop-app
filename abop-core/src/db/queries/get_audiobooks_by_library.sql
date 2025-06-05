SELECT 
    a.id, 
    a.library_id, 
    a.path, 
    a.title, 
    a.author, 
    a.narrator, 
    a.description, 
    a.duration_seconds, 
    a.size_bytes, 
    a.cover_art,
    a.created_at,
    a.updated_at
FROM 
    audiobooks a
WHERE 
    a.library_id = ?1
ORDER BY 
    COALESCE(a.title, a.path)
