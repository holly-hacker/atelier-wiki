prep-data GAME_DIR:
    cd data-prepper && cargo run -- type-defs -o ../atelier-wiki/app/atelier-data-types.d.ts
    cd data-prepper && cargo run -- extract -i {{GAME_DIR}} -o ../game-data
