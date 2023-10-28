import types from "@/data/types/ryza3";
import { EnemyLink } from "../../utility_components/links";
import { useContext } from "react";
import { Ryza3Context } from "@/data/ryza3_data";
import { Link } from "react-router-dom";

export function ItemDropSourcesSection({ item }: { item: types.Item }) {
  const ryza3Data = useContext(Ryza3Context);

  const monsters = MonsterSources(item);
  const puniFeeding = PuniFeedingSources(item);

  const noKnownSources = !monsters && !puniFeeding;
  const canBeCrafted = ryza3Data.recipes.recipes.some(
    (r) => r.item_tag == item.tag,
  );

  return (
    <>
      <h2>Sources</h2>
      {monsters}
      {puniFeeding}

      {noKnownSources && (
        <p>
          There are no known sources for this item
          {canBeCrafted && " besides crafting"}.
        </p>
      )}
    </>
  );
}

function MonsterSources(item: types.Item) {
  const ryza3Data = useContext(Ryza3Context);

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

  if (drops.length == 0) {
    return null;
  }

  return (
    <>
      <h3>Monster drops</h3>
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
  );
}

function PuniFeedingSources(item: types.Item) {
  const ryza3Data = useContext(Ryza3Context);

  const events = ryza3Data.puni_feeding.unique_events;
  const filteredEvents = events.filter((e) => e.item_tag == item.tag);

  if (filteredEvents.length == 0) {
    return null;
  }

  return (
    <>
      <h3>Puni feeding</h3>
      {filteredEvents.length == 1 && (
        <>
          <p>
            This item drops as a special item through puni feeding when the
            following condition is met:
          </p>
          <ul>
            {filteredEvents.map((e, i) => {
              return (
                <li key={i}>
                  <code>{JSON.stringify(e.condition)}</code>
                </li>
              );
            })}
          </ul>
          <p>
            See <Link to="/ryza3/puni_feeding">Puni feeding</Link> for more
            info.
          </p>
        </>
      )}
    </>
  );
}
