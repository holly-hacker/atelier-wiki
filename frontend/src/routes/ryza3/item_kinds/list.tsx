import items from "@/data/ryza3/items.json";
import { Link } from "react-router-dom";

export default function ItemKindsList() {
  const kinds = items.map((item) => item.kind_tag);
  const uniqueKinds = [...new Set(kinds)];

  return (
    <>
      <h1>Item kinds</h1>A list of all item kinds.
      <table>
        <thead>
          <tr>
            <th>Item kind</th>
            <th>Count</th>
          </tr>
        </thead>
        <tbody>
          {uniqueKinds.map((kind, i) => {
            return (
              <tr key={i}>
                <td>
                  <Link to={`/ryza3/item_kinds/${kind}`}>{kind}</Link>
                </td>
                <td>{kinds.filter((c) => c === kind).length}</td>
              </tr>
            );
          })}
        </tbody>
      </table>
    </>
  );
}
