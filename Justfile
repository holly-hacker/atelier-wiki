# Extract game info and generate typescript definitions
prep-data GAME_DIR:
    cd data-prepper && cargo run -- type-defs -o ../frontend/src/data/types/
    cd data-prepper && cargo run -- extract -i {{GAME_DIR}} -o ../frontend/src/data/

# Extract game textures and upload them to object storage
upload-textures GAME_DIR:
    # this needs a release build because it uses oxipng to improve file sizes
    cd data-prepper && cargo run --release -- -v extract-images -i {{GAME_DIR}} --upload -c 1
