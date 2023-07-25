import { ryza3 } from "@/data.ts";
import { Link } from "react-router-dom";

export default function ItemUseTagsList() {
  const tags = ryza3.item_data.map((item) => item.use_tag);
  const unique_tags = [...new Set(tags)];

  return (
    <>
      <h1>Item tags</h1>A list of all item tags.
      <table>
        <tr>
          <th>Item kind</th>
          <th>Count</th>
        </tr>
        {unique_tags.map((kind, i) => {
          return (
            <tr key={i}>
              <td>
                <Link to={`/ryza3/item_use_tags/${kind}`}>{kind}</Link>
              </td>
              <td>{tags.filter((c) => c === kind).length}</td>
            </tr>
          );
        })}
      </table>
    </>
  );
}