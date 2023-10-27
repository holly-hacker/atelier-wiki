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
        <li>
          <Link to="/ryza3/enemies">Enemies</Link>
        </li>
        <li>
          <Link to="/ryza3/puni_feeding">Puni feeding</Link>
        </li>
      </ul>
      <hr />
      <ul>
        <li>
          <Link to="/ryza3/map">Map test</Link>
        </li>
      </ul>
      <hr />
      <h2>External</h2>
      <ul>
        <li>
          <a href="https://barrelwisdom.com/ryza3/">Barrel Wisdom</a>
        </li>
        <li>
          <a href="https://pipplecultist.github.io/Ryza3MaterialMapFinder/">
            Ryza3 Material Map Finder
          </a>
        </li>
      </ul>
    </>
  );
}
