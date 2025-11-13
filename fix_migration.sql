-- SQLx Migration Fix Script
-- This script fixes the VersionMismatch error for migration 20251102070417
--
-- The error occurs because the checksum in the database doesn't match the migration file.
-- This usually happens when a migration file is modified after it has been applied.

-- First, check what's currently in the migrations table
SELECT * FROM _sqlx_migrations WHERE version = 20251102070417;

-- Option 1: Delete the migration record and let SQLx re-apply it
-- This is the safest option as it will re-run the migration with the correct checksum
DELETE FROM _sqlx_migrations WHERE version = 20251102070417;

-- After running this script, restart your auth-service container
-- It will detect the missing migration and re-apply it with the correct checksum
