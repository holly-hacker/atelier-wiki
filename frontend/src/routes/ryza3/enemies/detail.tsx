import { useParams } from "react-router-dom";
import {
  enemyDisplayName,
  findItemByTag,
  getImageLink,
} from "../ryza3_data_util";
import { ItemLink } from "../utility_components/links";
import types from "@/data/types/ryza3";
import { Ryza3Context } from "@/data/ryza3_data";
import { useContext } from "react";

export default function EnemyDetail() {
  const ryza3Data = useContext(Ryza3Context);
  const { id } = useParams();

  let enemy;
  if (id && !isNaN(Number(id))) {
    // id is a number
    enemy = ryza3Data.enemies[Number(id)];
  } else if (id) {
    // try to find by monster tag
    const tag = `MONSTER_${id}`;
    enemy = ryza3Data.enemies.find((v) => v.monster_tag == tag);
  }

  if (!enemy) {
    return <>Enemy not found</>;
  }

  return (
    <>
      <h1>{enemyDisplayName(enemy)}</h1>
      {enemy.library_note && <p>{enemy.library_note}</p>}
      {enemy.img_no !== null && enemy.img_no >= 0 && (
        <img src={getImageLink(`enemies/${enemy.img_no}.png`)}></img>
      )}

      <EnemyStats enemy={enemy} />
      <EnemyDetailSection enemy={enemy} />
      <EnemyInstanceSection enemy={enemy} />

      <details>
        <summary>Json data</summary>
        <pre>{JSON.stringify(enemy, null, 4)}</pre>
      </details>
    </>
  );
}

function EnemyStats({ enemy }: { enemy: types.Enemy }) {
  const stats: [string, number][] = [
    ["Health", enemy.library_rank_health],
    ["Attack", enemy.library_rank_attack],
    ["Speed", enemy.library_rank_speed],
    ["Defense", enemy.library_rank_defense],
  ];
  return (
    <table>
      <tbody>
        {stats.map(([name, value]) => (
          <tr key={name}>
            <th>{name}</th>
            <td>
              <StarRating value={value} />
            </td>
          </tr>
        ))}
      </tbody>
    </table>
  );
}

function StarRating({ value }: { value: number }) {
  return (
    <>
      {"⭐".repeat(value)}
      <span style={{ filter: "grayscale(100%)" }}>
        {"⭐".repeat(Math.max(5 - value, 0))}
      </span>
    </>
  );
}

function EnemyDetailSection({ enemy }: { enemy: types.Enemy }) {
  return (
    <>
      <h2>Details</h2>
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
      </ul>
    </>
  );
}

function EnemyInstanceSection({ enemy }: { enemy: types.Enemy }) {
  return (
    <>
      <h2>Instances</h2>
      <ul>
        {enemy.statusses.map((status, i) => (
          <li key={i}>
            <EnemyInstance status={status} />
          </li>
        ))}
      </ul>
    </>
  );
}

function EnemyInstance({ status }: { status: types.EnemyStatus }) {
  const ryza3Data = useContext(Ryza3Context);

  return (
    <ul>
      <li>Level: {status.lv}</li>
      <li>
        Exp: {status.exp}{" "}
        {status.exp != status.exp_rosca && <>(rosca: {status.exp_rosca})</>}
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
        Key: <code>{status.key_create_tag}</code>, {status.key_make}% base
        chance
      </li>
      <li>HP: {status.hp}</li>
      <li>Atk: {status.atk}</li>
      <li>Def: {status.def}</li>
      <li>Spd: {status.spd}</li>
      <li>Resistances: {status.bad_resist.map((r) => `${r}%`).join(", ")}</li>
      <li>Non-elemental resistance(?): {status.resist_non}%</li>
      <li>
        Attributes:
        <ul>
          {status.att.map(
            (att, i) =>
              att !== "ATT_NONE" && (
                <li key={i}>
                  <code>{att}</code>
                </li>
              ),
          )}
        </ul>
      </li>
      <li>
        Drops:
        <ul>
          {status.drops.map((drop, i) => {
            const item = findItemByTag(ryza3Data, drop.item_tag);
            return (
              <li key={i}>
                {drop.rate}% {drop.num}x{" "}
                {item ? <ItemLink item={item} /> : <code>{drop.item_tag}</code>}
                <ul>
                  <li>
                    Quality: {drop.quality_min} (x
                    {drop.quality_min_adj}) - {drop.quality_max} (x
                    {drop.quality_max_adj})
                  </li>
                  <li>
                    Trait: {drop.potential_min} (x
                    {drop.potential_min_adj}) - {drop.potential_max} (x
                    {drop.potential_max_adj})
                  </li>
                  <li>
                    Trait num: {drop.potential_num_min} (+
                    {drop.potential_num_min_adj}?) - {drop.potential_num_max} (+
                    {drop.potential_num_max_adj}?)
                  </li>
                  <li>
                    Trait level: {drop.potential_lv_min} (+
                    {drop.potential_lv_min_adj}?) - {drop.potential_lv_max} (+
                    {drop.potential_lv_max_adj}?)
                  </li>
                </ul>
              </li>
            );
          })}
        </ul>
      </li>
    </ul>
  );
}
