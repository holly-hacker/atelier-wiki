import types from "@/atelier-data-types";
import { Link } from "react-router-dom";
import { enemyDisplayName, itemDisplayName } from "../ryza3_data_util";
import { ryza3 } from "@/data";

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
    id = String(ryza3.item_data.findIndex((v) => v === item));
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
