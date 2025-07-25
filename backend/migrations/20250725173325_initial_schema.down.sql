-- Drop trigger
DROP TRIGGER IF EXISTS update_projects_updated_at ON projects;

-- Drop function
DROP FUNCTION IF EXISTS update_updated_at_column();

-- Drop indexes
DROP INDEX IF EXISTS idx_projects_featured;
DROP INDEX IF EXISTS idx_projects_primary_language;

-- Drop table
DROP TABLE IF EXISTS projects;
