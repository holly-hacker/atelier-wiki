# Print list of available commands
default:
  just --list

# Extract game info and generate typescript definitions
extract-data GAME_DIR:
    cd data-prepper && cargo run -- type-defs -o ../frontend/src/data/types/
    cd data-prepper && cargo run -- extract -i {{GAME_DIR}} -o ../frontend/src/data/

# Extract game textures and upload them to object storage
extract-textures GAME_DIR:
    # this needs a release build because it uses oxipng to improve file sizes
    cd data-prepper && cargo run --release -- -v extract-images -i {{GAME_DIR}} -o ../object-storage-data/game-data -c 1

    # ryza3
    cp object-storage-data/game-data/ryza3/texture-atlasses/* frontend/src/data/ryza3/texture-atlasses/
    cp object-storage-data/game-data/ryza3/maps/map_data.json frontend/src/data/ryza3/
    rm -r object-storage-data/game-data/ryza3/texture-atlasses
    rm object-storage-data/game-data/ryza3/maps/map_data.json

# Upload the content of the `object-storage-data` directory to object storage
upload:
    rclone copy object-storage-data/ atelier-wiki-data:atelier-wiki-data --progress --checksum --transfers 16
