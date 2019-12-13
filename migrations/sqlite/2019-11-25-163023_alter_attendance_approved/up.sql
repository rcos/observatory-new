-- Add approval flag
ALTER TABLE attendances ADD approved BOOLEAN NOT NULL DEFAULT 1;
