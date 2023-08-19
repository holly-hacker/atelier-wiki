import types from "@/data/types/ryza3";
import items from "@/data/ryza3/items.json";
import item_categories from "@/data/ryza3/item_categories.json";

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

export function itemCategoryDisplayName(category_tag: string): string {
  // falling back to tag because we should never encounter unknown categories
  return item_categories.categories[category_tag] || category_tag;
}

export function findItemByTag(tag: string): types.Item | undefined {
  return items.find((item) => item.tag === tag);
}

export function getImageLink(path: string): string {
  // TODO: this should be configurable
  return `https://atelier-wiki-data.variant9.dev/game-data/${path}`;
}
