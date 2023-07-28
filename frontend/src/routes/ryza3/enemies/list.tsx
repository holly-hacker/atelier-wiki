import { ryza3 } from "@/data.ts";
import { Link } from "react-router-dom";
import { enemyDisplayName } from "../ryza3_data_util";

export default function EnemyList() {
  const enemy_data = ryza3.enemy_data;

  return (
    <>
      <h1>Ryza 3 enemy list</h1>
      <div>
        {enemy_data.length} enemies found.
        <table>
          <tr>
            <th>Name</th>
            <th>Race tag</th>
            <th>Tag</th>
            <th>Size</th>
            <th>Instance count</th>
            <th>DLC</th>
          </tr>
          {enemy_data.map((enemy, i) => {
            return (
              <tr key={i}>
                <td>
                  <Link to={`/ryza3/enemy/${i}`}>
                    {enemyDisplayName(enemy)}
                  </Link>
                </td>
                <td>
                  <code>{enemy.race_tag}</code>
                </td>
                <td>
                  <code>{enemy.monster_tag}</code>
                </td>
                <td>
                  <code>{enemy.size}</code>
                </td>
                <td>{enemy.statusses.length} instance(s)</td>
                {/* NOTE: Ryza3 does not contain enemies that require multiple DLC */}
                <td>
                  <code>{enemy.dlc && enemy.dlc[0]}</code>
                </td>
              </tr>
            );
          })}
        </table>
      </div>
    </>
  );
}