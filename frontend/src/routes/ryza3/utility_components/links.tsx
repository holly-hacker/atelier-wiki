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
  const itemIndex = ryza3.item_data.findIndex((v) => v === item);

  return (
    <Link to={`/ryza3/items/${itemIndex}`}>
      {children || itemDisplayName(item)}
    </Link>
  );
}

export function EnemyLink({
  enemy,
  children,
}: {
  enemy: types.Enemy;
  children?: React.ReactNode;
}) {
  const ememyIndex = ryza3.enemy_data.findIndex((v) => v === enemy);

  return (
    <Link to={`/ryza3/enemy/${ememyIndex}`}>
      {children || enemyDisplayName(enemy)}
    </Link>
  );
}
