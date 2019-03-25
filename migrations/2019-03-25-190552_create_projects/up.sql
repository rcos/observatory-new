-- Your SQL goes here
CREATE TABLE projects (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    -- The name of the project
    name TEXT NOT NULL,
    -- Optional homepage of the project
    homepage TEXT,
    -- Repository of the project
    repo TEXT NOT NULL,
    -- Optional ID of the owner of the project
    owner_id INTEGER NOT NULL
);