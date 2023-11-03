# Print list of available commands
default:
  just --list

copy-data:
    mkdir -p object-storage-data/game-data/sophie
    mkdir -p frontend/src/data/types/sophie
    rm -r frontend/src/data/types/sophie/*.d.ts
    cp atelier-data/sophie/manually_extracted/*.json object-storage-data/game-data/sophie/
    cp atelier-data/sophie/*.json object-storage-data/game-data/sophie/
    cp atelier-data/sophie/*.d.ts frontend/src/data/types/sophie/

    mkdir -p object-storage-data/game-data/ryza3
    mkdir -p frontend/src/data/types/ryza3
    rm -r frontend/src/data/types/ryza3/*.d.ts
    cp atelier-data/ryza3/*.json object-storage-data/game-data/ryza3/
    cp atelier-data/ryza3/*.d.ts frontend/src/data/types/ryza3/

# Extract game textures for upload to object storage
extract-textures GAME_DIR:
    rm -r frontend/src/data/types/common/*.d.ts
    cd data-extractor && cargo run --release -- typedefs -o ../frontend/src/data/types/common -c common
    cd data-extractor && cargo run --release -- -v extract-images -i {{GAME_DIR}} -o ../object-storage-data/game-data -c 1

# Upload the content of the `object-storage-data` directory to object storage
upload:
    rclone copy object-storage-data/ atelier-wiki-data:atelier-wiki-data --progress --checksum --transfers 16
