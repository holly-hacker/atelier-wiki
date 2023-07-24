import { ryza3 } from "@/data.ts";
import { Link } from "react-router-dom";

export default function ItemKindsList() {
  const kinds = ryza3.item_data.map((item) => item.kind_tag);
  const unique_kinds = [...new Set(kinds)];

  return (
    <>
      <h1>Item kinds</h1>A list of all item kinds.
      <table>
        <tr>
          <th>Item kind</th>
          <th>Count</th>
        </tr>
        {unique_kinds.map((kind, i) => {
          return (
            <tr key={i}>
              <td>
                <Link to={`/ryza3/item_kinds/${kind}`}>{kind}</Link>
              </td>
              <td>{kinds.filter((c) => c === kind).length}</td>
            </tr>
          );
        })}
      </table>
    </>
  );
}
