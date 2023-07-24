import { ryza3 } from "@/data.ts";
import { Link } from "react-router-dom";

export default function ItemList() {
  let item_data = ryza3.item_data;

  return (
    <>
      <h1>Ryza 3 item list</h1>
      <div>
        List of all items should come here.
        <br />
        {item_data.length} items found.
        <table>
          <tr>
            <th>Name</th>
            <th>Price</th>
            <th>Level</th>
            <th>HP</th>
            <th>Atk</th>
            <th>Def</th>
            <th>Spd</th>
            <th>use_tag</th>
            <th>kind_tag</th>
            <th>DLC</th>
          </tr>
          {item_data.map((item, i) => {
            return (
              <tr key={i}>
                <td>
                  <Link to={`/ryza3/items/${i}`}>
                    {item.name ?? `<<item ${i}>>`}
                  </Link>
                </td>
                <td>{item.price}</td>
                <td>{item.lv}</td>
                <td>{item.hp}</td>
                <td>{item.atk}</td>
                <td>{item.def}</td>
                <td>{item.spd}</td>
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
        </table>
      </div>
    </>
  );
}
