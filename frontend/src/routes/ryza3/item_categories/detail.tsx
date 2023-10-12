import { useParams } from "react-router-dom";
import { ItemLink } from "../utility_components/links";
import { itemCategoryDisplayName } from "../ryza3_data_util";
import { Ryza3Context } from "@/data/ryza3_data";
import { useContext } from "react";

export default function ItemCategoryDetail() {
  const ryza3Data = useContext(Ryza3Context);
  const { category } = useParams();

  if (!category) {
    return <>No category selected.</>;
  }

  const category_tag = `ITEM_CATEGORY_${category}`;
  const category_name = itemCategoryDisplayName(ryza3Data, category_tag);

  const item = ryza3Data.items
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
