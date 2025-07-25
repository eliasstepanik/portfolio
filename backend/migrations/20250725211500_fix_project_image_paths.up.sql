-- Fix project image URLs to use correct file paths
UPDATE projects SET image_url = '/public/images/voicemeeter-control-preview.jpg' WHERE name = 'VoicemeeterSliderControlCSharp';
UPDATE projects SET image_url = '/public/images/ionos-ddns-preview.jpg' WHERE name = 'IonosDDNSUpdater';