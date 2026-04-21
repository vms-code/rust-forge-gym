-- Correct schema: ONLY user progress, NO quiz content ever stored here
CREATE TABLE user_progress (
    quiz_id INTEGER PRIMARY KEY,
    correct_attempts INTEGER DEFAULT 0,
    total_attempts INTEGER DEFAULT 0,
    last_attempt_at TEXT,
    streak INTEGER DEFAULT 0,
    ease REAL DEFAULT 2.5,
    next_review_at TEXT
);