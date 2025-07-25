-- Remove seeded projects
DELETE FROM projects WHERE github_url IN (
    'https://github.com/eliasstepanik/pathtracer',
    'https://github.com/eliasstepanik/Bevy-TileSystem',
    'https://github.com/eliasstepanik/voxel-simulation',
    'https://github.com/eliasstepanik/FluidSimulation'
);
