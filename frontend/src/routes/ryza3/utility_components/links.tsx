import types from "@/data/types/ryza3";
import texture_atlas from "@/data/types/texture_atlas";
import { Link } from "react-router-dom";
import { enemyDisplayName, itemDisplayName } from "../ryza3_data_util";
import items from "@/data/ryza3/items.json";

export function ItemLink({
  item,
  children,
}: {
  item: types.Item;
  children?: React.ReactNode;
}) {
  let id;
  if (item.tag) {
    // if the item has a tag, use that
    id = item.tag;

    // strip the 'ITEM_' prefix from the tag, it is present on all items
    id = id.replace(/^ITEM_/, "");
  } else {
    // use the item index
    id = String(items.findIndex((v) => v === item));
  }

  return (
    <Link to={`/ryza3/items/${id}`}>{children || itemDisplayName(item)}</Link>
  );
}

export function EnemyLink({
  enemy,
  children,
}: {
  enemy: types.Enemy;
  children?: React.ReactNode;
}) {
  // each enemy has a valid monster tag, so we can always use that in our links
  let id = enemy.monster_tag;

  // strip 'MONSTER_' prefix from the tag
  id = id.replace(/^MONSTER_/, "");

  return (
    <Link to={`/ryza3/enemy/${id}`}>{children || enemyDisplayName(enemy)}</Link>
  );
}

export function TextureAtlasImage({
  texture_atlas,
  texture_atlas_name,
  name, // TODO: accept dimensions as parameter too
}: {
  texture_atlas: texture_atlas.UniformTextureAtlasInfo;
  texture_atlas_name: string;
  name: string;
}) {
  const index = texture_atlas.stored_images.indexOf(name);
  const x_index = index % texture_atlas.columns;
  const y_index = Math.floor(index / texture_atlas.columns);

  return (
    <span
      style={{
        display: "inline-block",
        height: texture_atlas.image_dimensions[0],
        width: texture_atlas.image_dimensions[1],
        backgroundImage: `url(https://atelier-wiki-data.variant9.dev/game-data/ryza3/${texture_atlas_name}/packed.png)`,
        backgroundPositionX: -x_index * texture_atlas.image_dimensions[0],
        backgroundPositionY: -y_index * texture_atlas.image_dimensions[1],
        backgroundRepeat: "no-repeat",
      }}
    ></span>
  );
}
