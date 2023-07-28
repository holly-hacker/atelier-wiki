import { useParams } from "react-router-dom";
import { ryza3 } from "@/data.ts";
import { ItemLink } from "../utility_components/links";

export default function ItemKindDetail() {
  const { kind } = useParams();

  if (!kind) {
    return <>No kind selected.</>;
  }

  const item = ryza3.item_data
    .map((item, idx) => ({ item, idx }))
    .filter(({ item: v }) => v.kind_tag == kind);

  if (!item.length) {
    return <>No items found for kind {kind}.</>;
  }

  return (
    <>
      <h1>{kind}</h1>
      All items of the {kind} kind.
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
