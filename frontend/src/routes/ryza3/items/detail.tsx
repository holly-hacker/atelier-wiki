import { Link, useParams } from "react-router-dom";
import items from "@/data/ryza3/items.json";
import enemies from "@/data/ryza3/enemies.json";
import { getImageLink, itemDisplayName } from "../ryza3_data_util";
import types from "@/data/types/ryza3";
import { EnemyLink } from "../utility_components/links";

export default function ItemDetail() {
  const { id } = useParams();

  let item;
  if (id && !isNaN(Number(id))) {
    // id is a number
    item = items[Number(id)];
  } else if (id) {
    // try to find by item tag
    let tag = `ITEM_${id}`;
    item = items.find((v) => v.tag == tag);
  }

  if (!item) {
    return <>Item not found</>;
  }

  const drops = getDrops(item);

  return (
    <>
      <h1>{itemDisplayName(item)}</h1>
      {item.library_note && <p>{item.library_note}</p>}
      {item.img_no !== null && item.img_no >= 0 && (
        <img src={getImageLink(`items/${item.img_no}.png`)}></img>
      )}
      <ul>
        {item.tag && (
          <li>
            Item tag: <code>{item.tag}</code>
          </li>
        )}
        <li>Price: {item.price}</li>
        <li>Level: {item.lv}</li>
        <li>
          Elements: {item.elem_fire && "🔥 "}
          {item.elem_ice && "❄️ "}
          {item.elem_thunder && "⚡ "}
          {item.elem_air && "🍃 "}
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
                  <EnemyLink enemy={enemy}>
                    {enemy.name} (lv {status.lv})
                  </EnemyLink>
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
        <p>This item does not drop from monsters</p>
      )}
    </>
  );
}

function getDrops(
  item: types.Item,
): { drop: types.EnemyDrop; status: types.EnemyStatus; enemy: types.Enemy }[] {
  if (!item.tag) return [];

  const drops = [];

  for (const enemy of enemies) {
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
