import { Link, useParams } from "react-router-dom";
import { ryza3 } from "@/data.ts";

export default function ItemCategoryDetail() {
  const { category } = useParams();

  if (!category) {
    return <>No category selected.</>;
  }

  const item = ryza3.item_data
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
              {/* TODO: include an id in this list */}
              <Link to={`/ryza3/items/${idx}`}>
                {item.name ?? `<<item ${idx}>>`}
              </Link>
            </li>
          );
        })}
      </ul>
    </>
  );
}
