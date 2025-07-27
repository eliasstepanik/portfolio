-- Add WebGPU Engine project
INSERT INTO projects (name, description, github_url, demo_url, technologies, primary_language, stars_count, featured, image_url) VALUES
('WebGPU Engine', 
 'A high-performance rendering engine built with WebGPU and WebAssembly. Features a custom scene graph, PBR materials, post-processing effects, and an intuitive editor interface for real-time 3D graphics.', 
 'https://github.com/eliasstepanik/webgpu-engine', 
 NULL, 
 ARRAY['Rust', 'WebGPU', 'WebAssembly', 'WGSL', 'TypeScript', '3D Graphics'], 
 'Rust', 
 0, 
 true, 
 '/public/images/webgpu-engine.png');