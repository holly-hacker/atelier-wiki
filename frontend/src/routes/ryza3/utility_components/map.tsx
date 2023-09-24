import { CRS, LatLngTuple } from "leaflet";
import { MapContainer } from "react-leaflet/MapContainer";
import { Marker } from "react-leaflet/Marker";
import { Popup } from "react-leaflet/Popup";
import { TileLayer } from "react-leaflet/TileLayer";
import map_data from "@/data/ryza3/map_data.json";
import { useState } from "react";

export default function Map() {
  const [mapId, setMapId] = useState(0);

  const map = map_data.maps[mapId];

  const url = `https://atelier-wiki-data.variant9.dev/game-data/ryza3/maps/${mapId}/{z}/{y}_{x}.webp`;

  // transforms to convert height/width to map coordinates

  const padded_dim = (1 << map.max_zoom_level) * map.tile_size;
  const x_to_map = (x: number): number => (x / padded_dim) * map.tile_size;
  const y_to_map = (y: number): number => (-y / padded_dim) * map.tile_size;
  const xy_to_map = (x: number, y: number): LatLngTuple => [
    y_to_map(y),
    x_to_map(x),
  ];

  console.log("map", map);
  console.log("padded_dim", padded_dim);
  console.log("0,0", xy_to_map(0, 0));
  console.log("w,h", xy_to_map(map.width, map.height));

  return (
    <>
      <select value={mapId} onChange={(e) => setMapId(Number(e.target.value))}>
        {Object.entries(map_data.maps).map(([id, map]) => (
          <option key={id} value={id}>
            Map {id} (zoom levels: {map.max_zoom_level + 1})
          </option>
        ))}
      </select>
      <MapContainer
        center={xy_to_map(map.width / 2, map.height / 2)}
        zoom={2}
        scrollWheelZoom={true}
        crs={CRS.Simple}
      >
        <TileLayer
          attribution="&copy; KOEI TECMO GAMES CO., LTD."
          url={url}
          tileSize={map.tile_size}
          noWrap={true}
          tms={true}
          minNativeZoom={0}
          maxNativeZoom={map.max_zoom_level}
          minZoom={0}
          maxZoom={map.max_zoom_level}
          keepBuffer={10}
          bounds={[
            // this is kinda weird, I don't know why x is on the second index
            xy_to_map(0, map.height),
            xy_to_map(map.width, 0),
          ]}
        />
        <Marker position={xy_to_map(0, 0)}>
          <Popup>Popup at 0,0</Popup>
        </Marker>
        <Marker position={xy_to_map(100, 100)}>
          <Popup>Popup at 100,100</Popup>
        </Marker>
        <Marker position={xy_to_map(200, 200)}>
          <Popup>Popup at 100,100</Popup>
        </Marker>
        <Marker position={xy_to_map(3000, 0)}>
          <Popup>Popup at 3000,0</Popup>
        </Marker>
        <Marker position={xy_to_map(map.width / 2, map.height / 2)}>
          <Popup>
            Popup at center ({map.width / 2}x{map.height / 2})
          </Popup>
        </Marker>
        <Marker position={xy_to_map(map.width, map.height)}>
          <Popup>
            Popup at end ({map.width}x{map.height})
          </Popup>
        </Marker>
      </MapContainer>
    </>
  );
}
