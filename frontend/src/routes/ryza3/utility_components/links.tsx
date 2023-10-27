import types from "@/data/types/ryza3";
import texture_atlas from "@/data/types/texture_atlas";
import { Link } from "react-router-dom";
import {
  enemyDisplayName,
  findItemByTag,
  itemCategoryDisplayName,
  itemDisplayName,
} from "../ryza3_data_util";
import { useContext } from "react";
import { Ryza3Context } from "@/data/ryza3_data";

export function ItemLink({
  item,
  children,
}: {
  item: types.Item | string;
  children?: React.ReactNode;
}) {
  const ryza3Data = useContext(Ryza3Context);

  if (typeof item === "string") {
    const found_item = findItemByTag(ryza3Data, item);

    if (found_item) {
      item = found_item;
    } else {
      return <code>{children ?? item}</code>;
    }
  }

  let id;
  if (item.tag) {
    // if the item has a tag, use that
    id = item.tag;

    // strip the 'ITEM_' prefix from the tag, it is present on all items
    id = id.replace(/^ITEM_/, "");
  } else {
    // use the item index
    id = String(ryza3Data.items.findIndex((v) => v === item));
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

export function CategoryLink({
  category_tag,
  children,
}: {
  category_tag: string;
  children?: React.ReactNode;
}) {
  const ryza3Data = useContext(Ryza3Context);
  const short_category_tag = category_tag.replace(/^ITEM_CATEGORY_/, "");

  return (
    <Link to={`/ryza3/item_categories/${short_category_tag}`}>
      {children || itemCategoryDisplayName(ryza3Data, category_tag)}
    </Link>
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
        backgroundImage: `url(${
          import.meta.env.VITE_DATA_URL
        }/ryza3/${texture_atlas_name}/packed.webp)`,
        backgroundPositionX: -x_index * texture_atlas.image_dimensions[0],
        backgroundPositionY: -y_index * texture_atlas.image_dimensions[1],
        backgroundRepeat: "no-repeat",
      }}
    ></span>
  );
}
