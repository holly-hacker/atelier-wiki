import { CRS, LatLngTuple } from "leaflet";
import { MapContainer } from "react-leaflet/MapContainer";
import { Marker } from "react-leaflet/Marker";
import { Popup } from "react-leaflet/Popup";
import { TileLayer } from "react-leaflet/TileLayer";
import map_data from "@/data/ryza3/map_data.json";
import field_map from "@/data/ryza3/field_map.json";
import { useState } from "react";
import { Rectangle } from "react-leaflet";

// for now, this is hardcoded. I don't know how to derive it from the game files.
const region_map = new Map<number, string>([
  [0, "ISLAND_TOWN"],
  // [1, "ISLAND_FIELD"], // TODO: figure out offset
  [2, "MOUNTAIN_FIELD"],
  // [3, "FOREST_FIELD"],
  // [4, "ANOTHER_FIELD"],
  [5, "LAST_DUNGEON"],
  // [7, "DESERT_FIELD"], // TODO: allow filtering, there are 3 overlapping regions
  // [9, "DESERT_TOWER"], // TODO: unsure
  [10, "FRONTIER_FIELD"],
]);

export default function MapTest() {
  const [mapId, setMapId] = useState(0);

  const map = map_data.maps[mapId];
  const region_name = region_map.get(mapId);
  const fields = field_map.field_maps.filter(
    (map) => map.load_region && map.load_region == region_name,
  );

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

  // TODO: how to derive this scale?
  const scale = 1 / 15.6;
  const region_bounds = (
    min_x: number,
    min_z: number,
    max_x: number,
    max_z: number,
  ): [LatLngTuple, LatLngTuple] => [
    xy_to_map(min_x * scale, min_z * scale),
    xy_to_map(max_x * scale, max_z * scale),
  ];

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
          url={`https://atelier-wiki-data.variant9.dev/game-data/ryza3/maps/${mapId}/{z}/{y}_{x}.webp`}
          tileSize={map.tile_size}
          noWrap={true}
          tms={true}
          minNativeZoom={0}
          maxNativeZoom={map.max_zoom_level}
          minZoom={0}
          maxZoom={map.max_zoom_level}
          keepBuffer={10}
          bounds={[
            // TODO: this should be map width and height, but that causes tiles to not show up
            xy_to_map(0, 0),
            xy_to_map(padded_dim, padded_dim),
          ]}
        />
        {fields.map((region) => (
          <>
            <Rectangle
              key={region.data_file_name + "_range"}
              bounds={region_bounds(
                region.range_min_x,
                region.range_min_z,
                region.range_max_x,
                region.range_max_z,
              )}
              pathOptions={{ color: "black" }}
            />
            {region.navi_range_min_x == null ? null : (
              <Rectangle
                key={region.data_file_name + "_navi_range"}
                bounds={region_bounds(
                  region.navi_range_min_x,
                  region.navi_range_min_z!,
                  region.navi_range_max_x!,
                  region.navi_range_max_z!,
                )}
                pathOptions={{ color: "red" }}
              />
            )}
          </>
        ))}

        <Marker position={xy_to_map(100, 100)}>
          <Popup>Popup at 100,100</Popup>
        </Marker>
      </MapContainer>
    </>
  );
}
