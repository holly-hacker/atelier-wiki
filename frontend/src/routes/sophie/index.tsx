import { Link } from "react-router-dom";

export default function SophieIndex() {
  return (
    <>
      <h1>Overview for Atelier Sophie</h1>
      <ul>
        <li>
          <Link to="/sophie/items">Items</Link>
        </li>
      </ul>
    </>
  );
}
