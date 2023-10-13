import { CRS, LatLngTuple } from "leaflet";
import { MapContainer } from "react-leaflet/MapContainer";
import { Marker } from "react-leaflet/Marker";
import { Popup } from "react-leaflet/Popup";
import { TileLayer } from "react-leaflet/TileLayer";
import { useContext, useState } from "react";
import { Rectangle } from "react-leaflet";
import { Ryza3Context } from "@/data/ryza3_data";

// for now, this is hardcoded. I don't know how to derive it from the game files.
// TODO: how to derive these values?
const region_scale = 1 / 15.6;
const map_offset_scale = 3;
const object_position_scale = 6.4;
const region_map_names = new Map([
  [0, "ISLAND_TOWN"],
  [1, "ISLAND_FIELD"],
  [2, "MOUNTAIN_FIELD"],
  [3, "FOREST_FIELD"],
  [4, "ANOTHER_FIELD"],
  [5, "LAST_DUNGEON"],
  [7, "DESERT_FIELD"], // TODO: allow filtering, there are 3 overlapping regions
  [9, "DESERT_TOWER"],
  [10, "FRONTIER_FIELD"],
]);

// TODO: consider clustering overlapping markers, https://github.com/Leaflet/Leaflet.markercluster

export default function GameMap() {
  const ryza3Data = useContext(Ryza3Context);
  const [mapId, setMapId] = useState(0);

  const map = ryza3Data.map_data.maps[mapId];
  const region_name = region_map_names.get(mapId);
  const region_map = ryza3Data.field_map.region_maps[mapId];
  const fields = ryza3Data.field_map.field_maps.filter(
    (map) => map.load_region && map.load_region == region_name,
  );

  // transforms to convert height/width to map coordinates
  const padded_dim = (1 << map.max_zoom_level) * map.tile_size;
  const x_to_map = (x: number): number =>
    ((x - region_map.pos[0] * map_offset_scale) / padded_dim) * map.tile_size;
  const y_to_map = (y: number): number =>
    (-(y - region_map.pos[1] * map_offset_scale) / padded_dim) * map.tile_size;
  const xy_to_map = (x: number, y: number): LatLngTuple => [
    y_to_map(y),
    x_to_map(x),
  ];

  console.log("map", map);
  console.log("padded_dim", padded_dim);
  console.log("0,0", xy_to_map(0, 0));
  console.log("w,h", xy_to_map(map.width, map.height));

  const region_bounds = (
    min_x: number,
    min_z: number,
    max_x: number,
    max_z: number,
  ): [LatLngTuple, LatLngTuple] => [
    xy_to_map(min_x * region_scale, min_z * region_scale),
    xy_to_map(max_x * region_scale, max_z * region_scale),
  ];

  return (
    <>
      <select value={mapId} onChange={(e) => setMapId(Number(e.target.value))}>
        {Object.entries(ryza3Data.map_data.maps).map(([id]) => (
          <option key={id} value={id}>
            {ryza3Data.field_map.region_maps[Number(id)].image_name} (map {id})
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
          url={`${
            import.meta.env.VITE_DATA_URL
          }/ryza3/maps/${mapId}/{z}/{y}_{x}.webp`}
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
        {fields.map((region, field_idx) => (
          <>
            <Rectangle
              key={`${region.data_file_name}_${field_idx}_range`}
              bounds={region_bounds(
                region.range_min_x,
                region.range_min_z,
                region.range_max_x,
                region.range_max_z,
              )}
              pathOptions={{ color: "#00000080" }}
            />
            {region.navi_range_min_x == null ? null : (
              <Rectangle
                key={`${region.data_file_name}_${field_idx}_navi_range`}
                bounds={region_bounds(
                  region.navi_range_min_x,
                  region.navi_range_min_z!,
                  region.navi_range_max_x!,
                  region.navi_range_max_z!,
                )}
                pathOptions={{ color: "red" }}
              />
            )}
            {
              // TODO: move to layer that can be toggled, clean up
              region.data_file_name == null
                ? null
                : ryza3Data.field_data[
                    region.data_file_name.toLowerCase() + ".xml"
                  ].cut_down_tree.map((tree, tree_idx) => (
                    <Marker
                      key={`${region.data_file_name}_${field_idx}_tree_${tree_idx}`}
                      position={xy_to_map(
                        tree.position[0] * object_position_scale,
                        tree.position[2] * object_position_scale,
                      )}
                    >
                      <Popup>Tree, {tree.rate}% chance</Popup>
                    </Marker>
                  ))
            }
            {region.data_file_name == null
              ? null
              : ryza3Data.field_data[
                  region.data_file_name.toLowerCase() + ".xml"
                ].enemy_random_spawner.map((enemy, enemy_idx) => (
                  <Marker
                    key={`${region.data_file_name}_${field_idx}_enemy_${enemy_idx}`}
                    position={xy_to_map(
                      enemy.position[0] * object_position_scale,
                      enemy.position[2] * object_position_scale,
                    )}
                  >
                    <Popup>
                      Random enemy spawn, {enemy.rate}% chance,{" "}
                      {enemy.symbol_group_1 ??
                        enemy.symbol_group_2 ??
                        enemy.symbol_group_3 ??
                        enemy.symbol_group_4 ??
                        enemy.symbol_group_5}
                    </Popup>
                  </Marker>
                ))}
            {region.data_file_name == null
              ? null
              : ryza3Data.field_data[
                  region.data_file_name.toLowerCase() + ".xml"
                ].instant_enemy_spawner.map((enemy, enemy_idx) => {
                  console.log("instant", enemy_idx, enemy);
                  return (
                    <Marker
                      key={`${region.data_file_name}_${field_idx}_enemy_instant_${enemy_idx}`}
                      position={xy_to_map(
                        enemy.position[0] * object_position_scale,
                        enemy.position[2] * object_position_scale,
                      )}
                    >
                      <Popup>
                        Instant enemy spawn, {enemy.rate}% chance,{" "}
                        <code>{enemy.symbol_group}</code>
                      </Popup>
                    </Marker>
                  );
                })}
          </>
        ))}

        <Marker position={xy_to_map(region_map.pos[0], region_map.pos[1])}>
          <Popup>Map offset</Popup>
        </Marker>
      </MapContainer>
    </>
  );
}
