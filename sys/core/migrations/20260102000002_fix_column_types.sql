-- Fix source_agent column type to match Rust struct (UUID instead of VARCHAR)
-- And fix checksum to be NOT NULL

-- Drop and recreate memory table with correct types (PostgreSQL doesn't support easy type changes for this case)
-- We'll use a cast approach instead

-- First, add a new UUID column
ALTER TABLE memory ADD COLUMN IF NOT EXISTS source_agent_uuid UUID;

-- Copy data if any exists (convert varchar to uuid where valid)
UPDATE memory SET source_agent_uuid = source_agent::uuid WHERE source_agent IS NOT NULL AND source_agent ~ '^[0-9a-fA-F-]{36}$';

-- Drop old column and rename new one
ALTER TABLE memory DROP COLUMN IF EXISTS source_agent;
ALTER TABLE memory RENAME COLUMN source_agent_uuid TO source_agent;

-- Make checksum NOT NULL with default empty string for existing rows
UPDATE memory SET checksum = '' WHERE checksum IS NULL;
ALTER TABLE memory ALTER COLUMN checksum SET DEFAULT '';
ALTER TABLE memory ALTER COLUMN checksum SET NOT NULL;
