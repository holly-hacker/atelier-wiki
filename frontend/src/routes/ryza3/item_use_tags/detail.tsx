import { Link, useParams } from "react-router-dom";
import { ryza3 } from "@/data.ts";

export default function ItemUseTagDetail() {
  const { tag } = useParams();

  if (!tag) {
    return <>No tag selected.</>;
  }

  const item = ryza3.item_data
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
