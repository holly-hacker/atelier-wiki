import types from "@/atelier-data-types";

export function itemDisplayName(item: types.Item): string {
  // NOTE: sort is not always correct, there are some items that share a `sort` value
  if (item.name) {
    return item.name;
  }
  return item.tag ?? `Unnamed item (#${item.sort})`;
}

export function enemyDisplayName(enemy: types.Enemy): string {
  return enemy.name;
}
