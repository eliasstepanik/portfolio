-- Create projects table
CREATE TABLE IF NOT EXISTS projects (
    id SERIAL PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    github_url VARCHAR(500) NOT NULL,
    demo_url VARCHAR(500),
    technologies TEXT[] NOT NULL,
    primary_language VARCHAR(100) NOT NULL,
    stars_count INTEGER DEFAULT 0,
    featured BOOLEAN DEFAULT false,
    image_url VARCHAR(500),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create index on primary_language for filtering
CREATE INDEX idx_projects_primary_language ON projects(primary_language);

-- Create index on featured for sorting
CREATE INDEX idx_projects_featured ON projects(featured);

-- Create updated_at trigger function
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Create trigger to auto-update updated_at
CREATE TRIGGER update_projects_updated_at BEFORE UPDATE
    ON projects FOR EACH ROW EXECUTE FUNCTION update_updated_at_column();
