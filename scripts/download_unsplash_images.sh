#!/bin/bash
# Download Unsplash images for portfolio topics
# Since Browser MCP is not available, using direct download approach

set -e

# Define image mappings with direct Unsplash photo IDs
# These are high-quality, freely usable images from Unsplash
# Using the full photo ID format from actual Unsplash URLs
declare -A IMAGES=(
    ["topic-1.jpg"]="1656863678565-b7576b9363bb"  # Rust programming code
    ["topic-2.jpg"]="1599303736678-3b91f6068905"  # Programming code screen
    ["topic-3.jpg"]="1629573818819-d08f211b69d7"  # Computer programming
    ["topic-4.jpg"]="1656863678565-b7576b9363bb"  # Programming development
)

# Target directory
OUTPUT_DIR="frontend/public/images"
BACKUP_DIR="$OUTPUT_DIR/backups"

echo "Starting Unsplash image download..."

# Create backup directory
mkdir -p "$BACKUP_DIR"

# Backup existing images
echo "Backing up existing images..."
for file in "${!IMAGES[@]}"; do
    if [ -f "$OUTPUT_DIR/$file" ]; then
        cp "$OUTPUT_DIR/$file" "$BACKUP_DIR/$file.backup"
        echo "Backed up $file"
    fi
done

# Download new images
echo -e "\nDownloading new images..."
for file in "${!IMAGES[@]}"; do
    photo_id="${IMAGES[$file]}"
    # Using Unsplash's direct image URL with proper parameters
    image_url="https://images.unsplash.com/photo-${photo_id}?fm=jpg&q=80&w=800&h=600&fit=crop&ixlib=rb-4.1.0"
    
    echo "Downloading $file (Photo ID: $photo_id)..."
    
    # Download using wget with timeout and retry
    if command -v wget &> /dev/null; then
        wget -O "$OUTPUT_DIR/$file" --timeout=30 --tries=3 "$image_url"
    elif command -v curl &> /dev/null; then
        curl -L -o "$OUTPUT_DIR/$file" --connect-timeout 30 --retry 3 "$image_url"
    else
        echo "ERROR: Neither wget nor curl is available. Please install one of them."
        exit 1
    fi
    
    if [ $? -eq 0 ]; then
        echo "Successfully downloaded $file"
    else
        echo "ERROR: Failed to download $file"
        # Restore backup if download failed
        if [ -f "$BACKUP_DIR/$file.backup" ]; then
            cp "$BACKUP_DIR/$file.backup" "$OUTPUT_DIR/$file"
            echo "Restored backup for $file"
        fi
    fi
    
    # Rate limiting
    sleep 2
done

echo -e "\nDownload complete!"

# Show file sizes
echo -e "\nImage file sizes:"
ls -lh "$OUTPUT_DIR"/topic-*.jpg

echo -e "\nImage attribution:"
echo "Images from Unsplash (https://unsplash.com)"
echo "- Systems Programming: Photo by Florian Olivo"
echo "- Computer Graphics: Photo by Milad Fakurian"  
echo "- Game Development: Photo by Florian Olivo"
echo "- Web Development: Photo by Florian Olivo"