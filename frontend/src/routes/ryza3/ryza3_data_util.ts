import types from "@/atelier-data-types";
import { ryza3 } from "@/data";

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
  return ryza3.item_data.find((item) => item.tag === tag);
}
