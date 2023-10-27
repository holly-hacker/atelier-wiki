import { useParams } from "react-router-dom";
import { ItemLink } from "../utility_components/links";
import { useContext } from "react";
import { Ryza3Context } from "@/data/ryza3_data";

export default function ItemUseTagDetail() {
  const ryza3Data = useContext(Ryza3Context);
  const { tag } = useParams();

  if (!tag) {
    return <>No tag selected.</>;
  }

  const items = ryza3Data.items
    .map((item, idx) => ({ item, idx }))
    .filter(({ item: v }) => v.use_tag == tag);

  if (!items.length) {
    return <>No items found for use tag {tag}.</>;
  }

  return (
    <>
      <h1>{tag}</h1>
      All items of the {tag} tag.
      <ul>
        {items.map(({ item, idx }) => {
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
