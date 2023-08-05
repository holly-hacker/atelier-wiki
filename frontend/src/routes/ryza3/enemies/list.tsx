import enemies from "@/data/ryza3/enemies.json";
import enemies_texture from "@/data/ryza3/texture-atlasses/enemies.json";
import { EnemyLink, TextureAtlasImage } from "../utility_components/links";

export default function EnemyList() {
  return (
    <>
      <h1>Ryza 3 enemy list</h1>
      <div>
        {enemies.length} enemies found.
        <table>
          <thead>
            <tr>
              <th></th>
              <th>Name</th>
              <th>Race tag</th>
              <th>Tag</th>
              <th>Size</th>
              <th>Instance count</th>
              <th>DLC</th>
            </tr>
          </thead>
          <tbody>
            {enemies.map((enemy, i) => {
              return (
                <tr key={i}>
                  <td>
                    <EnemyLink enemy={enemy}>
                      <TextureAtlasImage
                        texture_atlas={enemies_texture}
                        texture_atlas_name="enemies"
                        name={String(enemy.img_no)}
                      />
                    </EnemyLink>
                  </td>
                  <td>
                    <EnemyLink enemy={enemy} />
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
          </tbody>
        </table>
      </div>
    </>
  );
}
