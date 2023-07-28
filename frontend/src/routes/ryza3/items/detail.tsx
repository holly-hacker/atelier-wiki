import { Link, useParams } from "react-router-dom";
import { ryza3 } from "@/data.ts";
import { item_display_name } from "../ryza3_data_util";
import types from "@/atelier-data-types";

export default function ItemDetail() {
  const { id } = useParams();

  const id_num = Number(id);

  if (id_num == null || id_num == undefined) {
    return <>Item not found</>;
  }

  const item = ryza3.item_data[id_num];

  const drops = get_drops(item);

  return (
    <>
      <h1>{item_display_name(item)}</h1>
      <ul>
        <li>Price: {item.price}</li>
        <li>Level: {item.lv}</li>
        <li>
          Elements: {item.elem_fire && "🔥 "}
          {item.elem_ice && "❄️ "}
          {item.elem_thunder && "⚡ "}
          {item.elem_air && "🍃 "}
        </li>
        <li>
          Item tag: <code>{item.tag}</code>
        </li>
        <li>
          Use tag:{" "}
          <Link to={`/ryza3/item_use_tags/${item.use_tag}`}>
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

      <h2>Monster drops</h2>
      {drops.length > 0 ? (
        <>
          <ul>
            {drops.map(({ drop, status, enemy }, i) => {
              return (
                <li key={i}>
                  {/* TODO: Fix up this link */}
                  <Link to={`/ryza3/enemy/${enemy.monster_tag}`}>
                    {enemy.name} (lv {status.lv})
                  </Link>
                  : Drop rate: {drop.num}x {drop.rate}%
                </li>
              );
            })}
          </ul>
          <details>
            <summary>Json data</summary>
            <pre>{JSON.stringify(drops, null, 4)}</pre>
          </details>
        </>
      ) : (
        <p>Does not drop from monsters</p>
      )}
    </>
  );
}

function get_drops(
  item: types.Item,
): { drop: types.EnemyDrop; status: types.EnemyStatus; enemy: types.Enemy }[] {
  if (!item.tag) return [];

  const drops = [];

  for (const enemy of ryza3.enemy_data) {
    for (const status of enemy.statusses) {
      for (const drop of status.drops) {
        if (drop.item_tag == item.tag) {
          drops.push({ drop, status, enemy });
        }
      }
    }
  }

  return drops;
}
