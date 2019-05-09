-- Your SQL goes here
CREATE TABLE projects (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    -- The name of the project
    name TEXT NOT NULL UNIQUE,
    -- Description of the project
    description TEXT NOT NULL,
    -- Optional homepage of the project
    homepage TEXT,
    -- Optional ID of the owner of the project
    owner_id INTEGER NOT NULL,
    -- Is the project active?
    active BOOLEAN NOT NULL DEFAULT 1,
    -- JSON array of repo URLs
    repos TEXT NOT NULL
);