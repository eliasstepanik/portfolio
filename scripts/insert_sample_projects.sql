-- Insert sample projects
INSERT INTO projects (name, description, github_url, demo_url, technologies, primary_language, stars_count, featured, image_url)
VALUES 
    ('VoicemeeterSliderControlCSharp', 
     'A C# application for controlling Voicemeeter sliders programmatically',
     'https://github.com/eliasstepanik/VoicemeeterSliderControlCSharp',
     NULL,
     ARRAY['C#', 'WPF', '.NET'],
     'C#',
     5,
     true,
     NULL),
     
    ('IonosDDNSUpdater',
     'Automatic DDNS updater for IONOS domains',
     'https://github.com/eliasstepanik/IonosDDNSUpdater',
     NULL,
     ARRAY['C#', '.NET', 'DNS'],
     'C#',
     3,
     false,
     NULL),
     
    ('voxel-simulation',
     'A voxel-based simulation engine built with Rust',
     'https://github.com/eliasstepanik/voxel-simulation',
     NULL,
     ARRAY['Rust', 'WebGPU', 'Graphics'],
     'Rust',
     12,
     true,
     '/public/images/voxel-simulation.jpg'),
     
    ('portfolio',
     'Personal developer portfolio built with Rust (Leptos + Axum)',
     'https://github.com/eliasstepanik/portfolio',
     'https://sailehd.de',
     ARRAY['Rust', 'Leptos', 'Axum', 'PostgreSQL', 'WebAssembly'],
     'Rust',
     8,
     true,
     NULL)
ON CONFLICT DO NOTHING;