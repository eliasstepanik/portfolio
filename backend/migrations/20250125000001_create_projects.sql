-- Drop old audiobooks table
DROP TABLE IF EXISTS audiobooks;

-- Create projects table
CREATE TABLE projects (
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
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);

-- Insert sample projects
INSERT INTO projects (name, description, github_url, demo_url, technologies, primary_language, stars_count, featured, image_url) VALUES
('Pathtracer', 'A high-performance ray tracing renderer written in Rust, implementing physically-based rendering techniques', 'https://github.com/eliasstepanik/pathtracer', NULL, ARRAY['Rust', 'Ray Tracing', 'Computer Graphics', 'SIMD'], 'Rust', 42, true, '/public/images/pathtracer-preview.jpg'),
('Bevy-TileSystem', 'A flexible tile-based game system for the Bevy game engine, supporting multiple tile types and efficient rendering', 'https://github.com/eliasstepanik/bevy-tilesystem', NULL, ARRAY['Rust', 'Bevy', 'Game Development', 'ECS'], 'Rust', 28, true, '/public/images/bevy-tilesystem-preview.jpg'),
('voxel-simulation', 'Voxel-based physics simulation with fluid dynamics and particle effects', 'https://github.com/eliasstepanik/voxel-simulation', NULL, ARRAY['Rust', 'WebGPU', 'Physics', 'Simulation'], 'Rust', 15, true, '/public/images/voxel-simulation-preview.jpg'),
('FluidSimulation', 'Real-time fluid dynamics simulation using SPH (Smoothed-particle hydrodynamics) implemented in C#', 'https://github.com/eliasstepanik/FluidSimulation', NULL, ARRAY['C#', 'Unity', 'Physics', 'Simulation', 'HLSL'], 'C#', 63, true, '/public/images/fluid-simulation-preview.jpg');