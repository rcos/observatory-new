-- Your SQL goes here
CREATE TABLE events (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    -- Date and time the event is happening at
    happening_at DATE NOT NULL DEFAULT (datetime('now','localtime')),
    -- Title of the event
    title TEXT NOT NULL,
    -- Optional description of the event
    description TEXT
    -- ID of the user who is hosting the event
    hosted_by INTEGER NOT NULL,
    -- Optional location of the event
    location TEXT
);