-- Add former member flag
ALTER TABLE users ADD former boolean NOT NULL DEFAULT 0;
-- Add external member flag
ALTER TABLE users ADD extrn boolean NOT NULL DEFAULT 0;
