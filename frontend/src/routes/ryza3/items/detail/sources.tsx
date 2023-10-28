import types from "@/data/types/ryza3";
import { EnemyLink } from "../../utility_components/links";
import { useContext } from "react";
import { Ryza3Context, Ryza3Data } from "@/data/ryza3_data";

export function ItemDropSourcesSection({ item }: { item: types.Item }) {
  const ryza3Data = useContext(Ryza3Context);
  const drops = getDrops(ryza3Data, item);

  return (
    <>
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
        <p>This item does not drop from monsters.</p>
      )}
    </>
  );
}

function getDrops(
  ryza3Data: Ryza3Data,
  item: types.Item,
): { drop: types.EnemyDrop; status: types.EnemyStatus; enemy: types.Enemy }[] {
  if (!item.tag) return [];

  const drops = [];

  for (const enemy of ryza3Data.enemies) {
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
