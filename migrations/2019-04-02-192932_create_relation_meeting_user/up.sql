-- Your SQL goes here
CREATE TABLE relation_meeting_user (
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    -- ID of the meeting
    meeting_id INTEGER NOT NULL,
    -- ID of the user who attended the meeting
    user_id INTEGER NOT NULL,
    FOREIGN KEY (meeting_id) REFERENCES meetings (id),
    FOREIGN KEY (user_id) REFERENCES users (id)
);