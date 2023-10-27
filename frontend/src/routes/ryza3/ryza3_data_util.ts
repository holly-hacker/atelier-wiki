import types from "@/data/types/ryza3";
import { Ryza3Data } from "@/data/ryza3_data";

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

export function itemCategoryDisplayName(ryza3Data: Ryza3Data, category_tag: string): string {
  // falling back to tag because we should never encounter unknown categories
  return ryza3Data.item_categories.categories[category_tag] || category_tag;
}

const ryza3_tag_aliases: Record<string, string> = {
  'ITEM_FURNITURE_ARTICLE_044': 'ITEM_MAT_RESERVE_005'
};
export function findItemByTag(ryza3Data: Ryza3Data, tag: string): types.Item | undefined {
  const real_tag = ryza3_tag_aliases[tag] ?? tag;
  return ryza3Data.items.find((item) => item.tag === real_tag);
}

export function getImageLink(path: string): string {
  return `${import.meta.env.VITE_DATA_URL}/ryza3/${path}`;
}
