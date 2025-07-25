-- Update project image URLs to match actual uploaded files
UPDATE projects SET image_url = '/public/images/BevyTileSystem.png' WHERE name = 'Bevy-TileSystem';
UPDATE projects SET image_url = '/public/images/FluidSimulation.mp4' WHERE name = 'FluidSimulation';
UPDATE projects SET image_url = '/public/images/Pathtracer.png' WHERE name = 'Pathtracer';
UPDATE projects SET image_url = '/public/images/voxel-simulation-demo.png' WHERE name = 'voxel-simulation';

-- Fix voxel-simulation demo URL (remove fake URL)
UPDATE projects SET demo_url = NULL WHERE name = 'voxel-simulation';

-- Add placeholder images for missing projects
UPDATE projects SET image_url = '/public/images/topic-1.jpg' WHERE name = 'VoicemeeterSliderControlCSharp';
UPDATE projects SET image_url = '/public/images/topic-1.jpg' WHERE name = 'IonosDDNSUpdater';