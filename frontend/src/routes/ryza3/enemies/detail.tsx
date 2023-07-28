import { useParams } from "react-router-dom";
import { ryza3 } from "@/data.ts";
import { enemyDisplayName, findItemByTag } from "../ryza3_data_util";
import { ItemLink } from "../utility_components/links";

export default function EnemyDetail() {
  const { id } = useParams();

  let enemy;
  if (id && !isNaN(Number(id))) {
    // id is a number
    enemy = ryza3.enemy_data[Number(id)];
  } else if (id) {
    // try to find by monster tag
    let tag = `MONSTER_${id}`;
    enemy = ryza3.enemy_data.find((v) => v.monster_tag == tag);
  }

  if (!enemy) {
    return <>Enemy not found</>;
  }

  return (
    <>
      <h1>{enemyDisplayName(enemy)}</h1>
      <ul>
        <li>Is big: {enemy.is_big}</li>
        <li>
          Monster tag: <code>{enemy.monster_tag}</code>
        </li>
        <li>
          Race tag: <code>{enemy.race_tag}</code>
        </li>
        <li>
          Size: <code>{enemy.size}</code>
        </li>
        <li>
          Instances
          <ul>
            {enemy.statusses.map((status, i) => {
              return (
                <li key={i}>
                  <ul>
                    <li>Level: {status.lv}</li>
                    <li>
                      Exp: {status.exp}{" "}
                      {status.exp != status.exp_rosca && (
                        <>(rosca: {status.exp_rosca})</>
                      )}
                    </li>
                    <li>
                      Money: {status.money}{" "}
                      {status.money != status.money_rosca && (
                        <>(rosca: {status.money_rosca})</>
                      )}
                    </li>
                    <li>
                      Gold coins:{" "}
                      {status.gold_coin && status.gold_coin_rate ? (
                        <>
                          {status.gold_coin} ({status.gold_coin_rate}%)
                        </>
                      ) : (
                        <>0</>
                      )}
                    </li>
                    <li>Stun: {status.stun}</li>
                    <li>
                      Key: <code>{status.key_create_tag}</code>,{" "}
                      {status.key_make}% base chance
                    </li>
                    <li>HP: {status.hp}</li>
                    <li>Atk: {status.atk}</li>
                    <li>Def: {status.def}</li>
                    <li>Spd: {status.spd}</li>
                    <li>
                      Resistances:{" "}
                      {status.bad_resist.map((r) => `${r}%`).join(", ")}
                    </li>
                    <li>Resistance &quot;non&quot;: {status.resist_non}</li>
                    <li>
                      Attributes:
                      <ul>
                        {status.att.map((att, i) => (
                          <li key={i}>
                            <code>{att}</code>
                          </li>
                        ))}
                      </ul>
                    </li>
                    <li>
                      Drops:
                      <ul>
                        {status.drops.map((drop, i) => {
                          let item = findItemByTag(drop.item_tag);
                          return (
                            <li key={i}>
                              {drop.rate}% {drop.num}x{" "}
                              {item ? (
                                <ItemLink item={item} />
                              ) : (
                                <code>{drop.item_tag}</code>
                              )}
                              <ul>
                                <li>
                                  Quality: {drop.quality_min} (x
                                  {drop.quality_min_adj}) - {drop.quality_max}{" "}
                                  (x
                                  {drop.quality_max_adj})
                                </li>
                                <li>
                                  Trait: {drop.potential_min} (x
                                  {drop.potential_min_adj}) -{" "}
                                  {drop.potential_max} (x
                                  {drop.potential_max_adj})
                                </li>
                                <li>
                                  Trait num: {drop.potential_num_min} (+
                                  {drop.potential_num_min_adj}?) -{" "}
                                  {drop.potential_num_max} (+
                                  {drop.potential_num_max_adj}?)
                                </li>
                                <li>
                                  Trait level: {drop.potential_lv_min} (+
                                  {drop.potential_lv_min_adj}?) -{" "}
                                  {drop.potential_lv_max} (+
                                  {drop.potential_lv_max_adj}?)
                                </li>
                              </ul>
                            </li>
                          );
                        })}
                      </ul>
                    </li>
                  </ul>
                </li>
              );
            })}
          </ul>
        </li>
      </ul>
      <details>
        <summary>Json data</summary>
        <pre>{JSON.stringify(enemy, null, 4)}</pre>
      </details>
    </>
  );
}
