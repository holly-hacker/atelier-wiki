# Object storage data layout

This is the file layout for data stored in object storage. The public instance of this wiki uses data stored at `atelier-wiki-data.variant9.dev`, so you can find data eg. https://atelier-wiki-data.variant9.dev/game-data/ryza3/items/804.png.

- `game-data/`
  - `ryza3/`
    - `enemies/`
      - `0.png`
      - `1.png`
      - ...
    - `items/`
      - `0.png`
      - `1.png`
      - ...
    - `maps/`
      - `0/0/0_0.webp`
      - `0/1/0_0.webp`
      - `0/1/0_1.webp`
      - `0/2/0_0.webp`
      - `0/2/0_1.webp`
      - `0/2/0_2.webp`
      - `0/2/1_0.webp`
      - ...
      - `1/0/0_0.webp`
      - ...

Map data is in the format `maps/{map_index}/{zoom}/{y}_{x}.webp`, with `x`/`y`/`zoom` as expected by [Leaflet](https://leafletjs.com/). Zoom-level 0 indicates that the entire map fits in a single tile.
