import { Link, useParams } from "react-router-dom";
import { ryza3 } from "@/data.ts";

export default function ItemDetail() {
  const { id } = useParams();

  const id_num = Number(id);

  if (id_num == null || id_num == undefined) {
    return <>Item not found</>;
  }

  const item = ryza3.item_data[id_num];

  return (
    <>
      <h1>{item.name ?? `Unnamed item (item #${id})`}</h1>
      <ul>
        {item.price && <li>Price: {item.price}</li>}
        {item.lv && <li>Level: {item.lv}</li>}
        <li>
          Elements:
          <ul>{item.elem_fire != null && <li>Fire: {item.elem_fire}</li>}</ul>
          <ul>{item.elem_ice != null && <li>Ice: {item.elem_ice}</li>}</ul>
          <ul>
            {item.elem_thunder != null && <li>Thunder: {item.elem_thunder}</li>}
          </ul>
          <ul>{item.elem_air != null && <li>Air: {item.elem_air}</li>}</ul>
        </li>
        <li>
          Use tag:{" "}
          <Link to={`/ryza3/item_use_kinds/${item.use_tag}`}>
            <code>{item.use_tag}</code>
          </Link>
        </li>
        <li>
          Kind tag:{" "}
          <Link to={`/ryza3/item_kinds/${item.kind_tag}`}>
            <code>{item.kind_tag}</code>
          </Link>
        </li>
        <li>
          Categories:
          <ul>
            {item.cat.map((cat, i) => {
              return (
                <li key={i}>
                  <Link to={`/ryza3/item_categories/${cat}`}>
                    <code>{cat}</code>
                  </Link>
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
