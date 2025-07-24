-- Create audiobooks table
CREATE TABLE IF NOT EXISTS audiobooks (
    id SERIAL PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    author VARCHAR(255) NOT NULL,
    duration VARCHAR(50) NOT NULL,
    audio_url VARCHAR(500) NOT NULL,
    cover_url VARCHAR(500) NOT NULL,
    description TEXT NOT NULL
);

-- Insert sample data matching frontend placeholders
INSERT INTO audiobooks (title, author, duration, audio_url, cover_url, description) VALUES
('Meditations', 'Marcus Aurelius', '3:45:00', '/public/audio/meditations.mp3', '/images/book-cover-1.jpg', 'Stoic philosophy and personal reflections from the Roman Emperor'),
('The Art of War', 'Sun Tzu', '2:30:00', '/public/audio/art-of-war.mp3', '/images/book-cover-2.jpg', 'Ancient Chinese military treatise on strategy and tactics'),
('The Republic', 'Plato', '11:00:00', '/public/audio/republic.mp3', '/images/book-cover-3.jpg', 'Philosophical dialogue concerning justice and the ideal state'),
('The Prince', 'Niccolò Machiavelli', '4:15:00', '/public/audio/prince.mp3', '/images/book-cover-4.jpg', 'Political treatise on power, leadership, and statecraft'),
('Walden', 'Henry David Thoreau', '7:30:00', '/public/audio/walden.mp3', '/images/book-cover-5.jpg', 'Reflections on simple living in natural surroundings'),
('On Liberty', 'John Stuart Mill', '5:00:00', '/public/audio/on-liberty.mp3', '/images/book-cover-6.jpg', 'Philosophical work on the nature and limits of power over the individual');