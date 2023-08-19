import { useParams } from "react-router-dom";
import items from "@/data/ryza3/items.json";
import { ItemLink } from "../utility_components/links";
import { itemCategoryDisplayName } from "../ryza3_data_util";

export default function ItemCategoryDetail() {
  const { category } = useParams();

  if (!category) {
    return <>No category selected.</>;
  }

  const category_tag = `ITEM_CATEGORY_${category}`;
  const category_name = itemCategoryDisplayName(category_tag);

  const item = items
    .map((item, idx) => ({ item, idx }))
    .filter(({ item: v }) => v.cat.includes(category_tag));

  if (!item.length) {
    return <>No items found for category {category_name}.</>;
  }

  return (
    <>
      <h1>{category_name}</h1>
      All items of the {category_name} category.
      <ul>
        {item.map(({ item, idx }) => {
          return (
            <li key={idx}>
              <ItemLink item={item} />
            </li>
          );
        })}
      </ul>
    </>
  );
}
