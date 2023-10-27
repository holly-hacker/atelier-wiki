import { useParams } from "react-router-dom";
import { ItemLink } from "../utility_components/links";
import { useContext } from "react";
import { Ryza3Context } from "@/data/ryza3_data";

export default function ItemKindDetail() {
  const ryza3Data = useContext(Ryza3Context);
  const { kind } = useParams();

  if (!kind) {
    return <>No kind selected.</>;
  }

  const items = ryza3Data.items
    .map((item, idx) => ({ item, idx }))
    .filter(({ item: v }) => v.kind_tag == kind);

  if (!items.length) {
    return <>No items found for kind {kind}.</>;
  }

  return (
    <>
      <h1>{kind}</h1>
      All items of the {kind} kind.
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
