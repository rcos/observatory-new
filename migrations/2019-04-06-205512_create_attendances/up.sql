-- Your SQL goes here
CREATE TABLE attendances (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    -- Was this an event attendance
    is_event BOOLEAN NOT NULL DEFAULT 0,
    -- ID of the user that is a member of the group
    user_id INTEGER NOT NULL,
    -- ID of the meeting
    meeting_id INTEGER,
    -- ID of the event
    event_id INTEGER,
    FOREIGN KEY (user_id) REFERENCES users (id),
    FOREIGN KEY (event_id) REFERENCES events (id),
    FOREIGN KEY (meeting_id) REFERENCES meetings (id)
)