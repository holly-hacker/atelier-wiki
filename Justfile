prep-data GAME_DIR:
    cd data-prepper && cargo run -- type-defs -o ../frontend/src/atelier-data-types.d.ts
    cd data-prepper && cargo run -- extract -i {{GAME_DIR}} -o ../frontend/src/data/
