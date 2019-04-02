-- Your SQL goes here
CREATE TABLE relation_project_user(
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    -- ID of the project
    project_ID INTEGER NOT NULL,
    -- ID of the user that is a member of the project
    user_id INTEGER NOT NULL
);