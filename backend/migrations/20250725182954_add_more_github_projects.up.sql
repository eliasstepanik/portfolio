-- Add more GitHub projects from eliasstepanik's profile
INSERT INTO projects (name, description, github_url, demo_url, technologies, primary_language, stars_count, featured, image_url) VALUES
('IonosDDNSUpdater', 
 'Dynamic DNS updater for IONOS domains. Automatically updates DNS records to keep your domain pointing to your dynamic IP address. Supports .NET, Docker, and Docker Compose deployments.', 
 'https://github.com/eliasstepanik/IonosDDNSUpdater', 
 NULL, 
 ARRAY['C#', '.NET', 'Docker', 'Docker Compose', 'DNS', 'IONOS API'], 
 'C#', 
 45, 
 true, 
 '/public/images/ionos-ddns-preview.jpg'),

('VoicemeeterSliderControlCSharp', 
 'C# application for controlling Voicemeeter audio mixer sliders via serial communication. Includes AutoHotkey integration for seamless start/stop functionality and JSON-based configuration.', 
 'https://github.com/eliasstepanik/VoicemeeterSliderControlCSharp', 
 NULL, 
 ARRAY['C#', 'WinForms', 'Serial Communication', 'AutoHotkey', 'Audio Processing'], 
 'C#', 
 23, 
 false, 
 '/public/images/voicemeeter-control-preview.jpg');