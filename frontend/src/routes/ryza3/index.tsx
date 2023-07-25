import { Link } from "react-router-dom";

export default function Ryza3Index() {
  return (
    <>
      <h1>Overview for Atelier Ryza 3</h1>
      <ul>
        <li>
          <Link to="/ryza3/items">Items</Link>
        </li>
        <li>
          <Link to="/ryza3/item_categories">Item categories</Link>
        </li>
        <li>
          <Link to="/ryza3/item_kinds">Item kinds</Link>
        </li>
        <li>
          <Link to="/ryza3/item_use_tags">Item use tags</Link>
        </li>
      </ul>
    </>
  );
}
