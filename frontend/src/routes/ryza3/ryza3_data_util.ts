import types from "@/data/types/ryza3";
import items from "@/data/ryza3/items.json";

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

export function findItemByTag(tag: string): types.Item | undefined {
  return items.find((item) => item.tag === tag);
}
