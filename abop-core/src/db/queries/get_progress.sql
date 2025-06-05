SELECT 
    id,
    audiobook_id,
    position_seconds,
    completed,
    last_played,
    created_at,
    updated_at
FROM 
    progress 
WHERE 
    audiobook_id = ?1
