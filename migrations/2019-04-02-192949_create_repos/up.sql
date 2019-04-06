-- Your SQL goes here
CREATE TABLE repos (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    -- ID of the project this repo belongs to
    project_id INTEGER NOT NULL,
    -- URL of the repo
    url TEXT NOT NULL,
    FOREIGN KEY (project_id) REFERENCES projects (id)
);