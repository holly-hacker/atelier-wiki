import { Link, useParams } from "react-router-dom";
import { ryza3 } from "@/data.ts";
import { enemy_display_name } from "../ryza3_data_util";

export default function EnemyDetail() {
  const { id } = useParams();

  const id_num = Number(id);

  if (id_num == null || id_num == undefined) {
    return <>Enemy not found</>;
  }

  const enemy = ryza3.enemy_data[id_num];

  return (
    <>
      <h1>{enemy_display_name(enemy)}</h1>
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
                    <li>Resistance "non": {status.resist_non}</li>
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
                        {status.drops.map((drop, i) => (
                          <li key={i}>
                            {/* TODO: link to item */}
                            {drop.rate}% {drop.num}x{" "}
                            <code>{drop.item_tag}</code>
                            <ul>
                              <li>
                                Quality: {drop.quality_min} (x
                                {drop.quality_min_adj}) - {drop.quality_max} (x
                                {drop.quality_max_adj})
                              </li>
                              <li>
                                Trait: {drop.potential_min} (x
                                {drop.potential_min_adj}) - {drop.potential_max}{" "}
                                (x
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
                        ))}
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
