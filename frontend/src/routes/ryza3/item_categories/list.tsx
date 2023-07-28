import { ryza3 } from "@/data.ts";
import { Link } from "react-router-dom";

export default function ItemCategoriesList() {
  const categories = ryza3.item_data.flatMap((item) => item.cat);
  const uniqueCategories = [...new Set(categories)];

  return (
    <>
      <h1>Item categories</h1>A list of all item categories.
      <table>
        <tr>
          <th>Category</th>
          <th>Count</th>
        </tr>
        {uniqueCategories.map((category, i) => {
          return (
            <tr key={i}>
              <td>
                <Link to={`/ryza3/item_categories/${category}`}>
                  {category}
                </Link>
              </td>
              <td>{categories.filter((c) => c === category).length}</td>
            </tr>
          );
        })}
      </table>
    </>
  );
}
