import { SophieContext } from "@/data/sophie_data";
import types from "@/data/types/sophie";
import { useContext } from "react";
import { useParams } from "react-router-dom";
import { CategoryLink } from "../utility_components/links";
import { getImageLink, itemDisplayName } from "../sophie_data_util";

export default function ItemDetail(): JSX.Element {
  const sophieData = useContext(SophieContext);
  const { id } = useParams();

  let item: types.Item | undefined;
  if (id && !isNaN(Number(id))) {
    // id is a number
    item = sophieData.items[Number(id)];
  } else if (id) {
    // try to find by item tag
    const tag = `ITEM_${id}`;
    item = sophieData.items.find((v) => v.tag == tag);
  }

  if (!item) {
    return <>Item not found</>;
  }

  return (
    <>
      <h1>{itemDisplayName(item)}</h1>
      {item.image_no !== null && item.image_no >= 0 && (
        <img src={getImageLink(`items/${item.image_no}.png`)}></img>
      )}
      <ItemDetailSection item={item} />
    </>
  );
}

function ItemDetailSection({ item }: { item: types.Item }) {
  return (
    <>
      <h2>Details</h2>
      <ul>
        {item.tag && (
          <li>
            Item tag: <code>{item.tag}</code>
          </li>
        )}
        <li>Price: {item.cost}</li>
        <li>Level: {item.level}</li>
        <li>Element: {item.color}</li>
        <li>
          Use tag: <code>{item.use_type}</code>
        </li>
        <li>
          Kind tag: <code>{item.base}</code>
        </li>
        <li>
          Categories:
          <ul>
            {item.category.map((cat, i) => {
              return (
                <li key={i}>
                  <CategoryLink category_tag={cat} />
                </li>
              );
            })}
          </ul>
        </li>
      </ul>
      <details>
        <summary>Json data</summary>
        <pre>{JSON.stringify(item, null, 4)}</pre>
      </details>
    </>
  );
}
