import type { ItemTypes } from "@/data/types/sophie";
import { SophieData } from "@/data/sophie_data";

export function itemDisplayName(item: ItemTypes.Item): string {
  return item.name;
}

export function itemCategoryDisplayName(_sophieData: SophieData, category_tag: string): string {
  // TODO
  return category_tag;
  // return sophieData.item_categories.categories[category_tag] || category_tag;
}

export function findItemByTag(sophieData: SophieData, tag: string): ItemTypes.Item | undefined {
  return sophieData.items.find((item) => item.tag === tag);
}

export function getImageLink(path: string): string {
  return `${import.meta.env.VITE_DATA_URL}/sophie/${path}`;
}
