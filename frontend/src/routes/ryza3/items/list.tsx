import items from "@/data/ryza3/items.json";
import items_texture from "@/data/ryza3/texture-atlasses/items.json";
import { ItemLink, TextureAtlasImage } from "../utility_components/links";

export default function ItemList() {
  return (
    <>
      <h1>Ryza 3 item list</h1>
      <div>
        List of all items should come here.
        <br />
        {items.length} items found.
        <table>
          <thead>
            <tr>
              <th></th>
              <th>Name</th>
              <th>Price</th>
              <th>Level</th>
              <th>HP</th>
              <th>Atk</th>
              <th>Def</th>
              <th>Spd</th>
              <th>Tag</th>
              <th>use_tag</th>
              <th>kind_tag</th>
              <th>DLC</th>
            </tr>
          </thead>
          <tbody>
            {items.map((item, i) => {
              return (
                <tr key={i}>
                  <td>
                    <ItemLink item={item}>
                      <TextureAtlasImage
                        texture_atlas={items_texture}
                        texture_atlas_name="items"
                        name={String(item.img_no)}
                      />
                    </ItemLink>
                  </td>
                  <td>
                    <ItemLink item={item} />
                  </td>
                  <td>{item.price}</td>
                  <td>{item.lv}</td>
                  <td>{item.hp}</td>
                  <td>{item.atk}</td>
                  <td>{item.def}</td>
                  <td>{item.spd}</td>
                  <td>
                    <code>{item.tag}</code>
                  </td>
                  <td>
                    <code>{item.use_tag}</code>
                  </td>
                  <td>
                    <code>{item.kind_tag}</code>
                  </td>
                  {/* NOTE: Ryza3 does not contain items that require multiple DLC */}
                  <td>
                    <code>{item.dlc && item.dlc[0]}</code>
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
