INSERT INTO progress (
    id,
    audiobook_id,
    position_seconds,
    completed,
    last_played,
    created_at,
    updated_at
) VALUES (
    ?1, ?2, ?3, ?4, ?5, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP
)
ON CONFLICT(audiobook_id) DO UPDATE SET
    position_seconds = excluded.position_seconds,
    completed = excluded.completed,
    last_played = excluded.last_played,
    updated_at = CURRENT_TIMESTAMP
WHERE audiobook_id = ?2
