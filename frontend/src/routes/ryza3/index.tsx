import { Link } from "react-router-dom";

export default function Ryza3Index() {
  return (
    <>
      <h1>Overview for Atelier Ryza 3</h1>
      <ul>
        <li>
          <Link to="/ryza3/items">Items</Link>
        </li>
      </ul>
    </>
  );
}
