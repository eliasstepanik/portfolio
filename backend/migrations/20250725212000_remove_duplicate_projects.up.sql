-- Remove duplicate projects (keep the ones with lower IDs)
DELETE FROM projects WHERE id IN (
    SELECT id FROM (
        SELECT id, 
               ROW_NUMBER() OVER (PARTITION BY name ORDER BY id) as rn
        FROM projects
    ) t WHERE rn > 1
);