import { CRS } from "leaflet";
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
        center={[-map.tile_size / 2, map.tile_size / 2]}
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
            [-map.tile_size, 0],
            [0, map.tile_size],
          ]}
        />
        <Marker position={[0, 0]}>
          <Popup>Popup at 0,0</Popup>
        </Marker>
        <Marker position={[-100, 100]}>
          <Popup>Popup at -100,100</Popup>
        </Marker>
        <Marker position={[-3000, 0]}>
          <Popup>Popup at -3000,0</Popup>
        </Marker>
      </MapContainer>
    </>
  );
}
