import { useParams } from "react-router-dom";
import items from "@/data/ryza3/items.json";
import { ItemLink } from "../utility_components/links";

export default function ItemUseTagDetail() {
  const { tag } = useParams();

  if (!tag) {
    return <>No tag selected.</>;
  }

  const item = items
    .map((item, idx) => ({ item, idx }))
    .filter(({ item: v }) => v.use_tag == tag);

  if (!item.length) {
    return <>No items found for use tag {tag}.</>;
  }

  return (
    <>
      <h1>{tag}</h1>
      All items of the {tag} tag.
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
