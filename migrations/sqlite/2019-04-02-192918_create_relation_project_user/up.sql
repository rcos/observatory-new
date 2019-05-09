-- Your SQL goes here
CREATE TABLE relation_project_user(
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    -- ID of the project
    project_id INTEGER NOT NULL,
    -- ID of the user that is a member of the project
    user_id INTEGER NOT NULL,
    FOREIGN KEY (project_id) REFERENCES projects (id),
    FOREIGN KEY (user_id) REFERENCES users (id)
);