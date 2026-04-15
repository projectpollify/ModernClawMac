ALTER TABLE messages ADD COLUMN feedback TEXT CHECK (feedback IN ('up', 'down'));

INSERT OR IGNORE INTO schema_version (version, applied_at)
VALUES (4, datetime('now'));
