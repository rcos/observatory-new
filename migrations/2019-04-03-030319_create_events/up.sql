-- Your SQL goes here
CREATE TABLE events (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    -- Date and time the event starts at
    start DATETIME NOT NULL,
    -- Date and time the event ends at
    end DATETIME NOT NULL,
    -- Title of the event
    title TEXT NOT NULL,
    -- Optional description of the event
    description TEXT,
    -- ID of the user who is hosting the event
    hosted_by INTEGER NOT NULL,
    -- Optional location of the event
    location TEXT,
    -- The attendance code of the meeting
    code TEXT NOT NULL UNIQUE
);