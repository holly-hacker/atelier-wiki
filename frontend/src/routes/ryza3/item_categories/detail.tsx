import { useParams } from "react-router-dom";
import items from "@/data/ryza3/items.json";
import { ItemLink } from "../utility_components/links";

export default function ItemCategoryDetail() {
  const { category } = useParams();

  if (!category) {
    return <>No category selected.</>;
  }

  const item = items
    .map((item, idx) => ({ item, idx }))
    .filter(({ item: v }) => v.cat.includes(category));

  if (!item.length) {
    return <>No items found for category {category}.</>;
  }

  return (
    <>
      <h1>{category}</h1>
      All items of the {category} category.
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
