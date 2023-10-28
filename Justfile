# Print list of available commands
default:
  just --list

copy-data:
    cp atelier-data/sophie/manually_extracted/item_boards.json object-storage-data/game-data/sophie/
    cp atelier-data/sophie/manually_extracted/shapes.json object-storage-data/game-data/sophie/

# Extract game info and generate typescript definitions
extract-data GAME_DIR:
    cd data-prepper && cargo run -- type-defs -o ../frontend/src/data/types/
    cd data-prepper && cargo run -- extract -i {{GAME_DIR}} -o ../object-storage-data/game-data/

# Extract game textures and upload them to object storage
extract-textures GAME_DIR:
    # this needs a release build because it uses oxipng to improve file sizes
    cd data-prepper && cargo run --release -- -v extract-images -i {{GAME_DIR}} -o ../object-storage-data/game-data -c 1

# Upload the content of the `object-storage-data` directory to object storage
upload:
    rclone copy object-storage-data/ atelier-wiki-data:atelier-wiki-data --progress --checksum --transfers 16
