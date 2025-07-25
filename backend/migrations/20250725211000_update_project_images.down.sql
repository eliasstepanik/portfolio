-- Revert project image URLs
UPDATE projects SET image_url = '/public/images/bevy-tilesystem-preview.jpg' WHERE name = 'Bevy-TileSystem';
UPDATE projects SET image_url = '/public/images/fluid-simulation-preview.jpg' WHERE name = 'FluidSimulation';
UPDATE projects SET image_url = '/public/images/pathtracer-preview.jpg' WHERE name = 'Pathtracer';
UPDATE projects SET image_url = '/public/images/voxel-simulation-preview.jpg' WHERE name = 'voxel-simulation';

-- Restore voxel-simulation demo URL
UPDATE projects SET demo_url = 'https://voxel-demo.example.com' WHERE name = 'voxel-simulation';

-- Revert placeholder images
UPDATE projects SET image_url = '/public/images/voicemeeter-control-preview.jpg' WHERE name = 'VoicemeeterSliderControlCSharp';
UPDATE projects SET image_url = '/public/images/ionos-ddns-preview.jpg' WHERE name = 'IonosDDNSUpdater';