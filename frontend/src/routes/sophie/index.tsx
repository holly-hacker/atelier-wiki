import { Link } from "react-router-dom";

export default function SophieIndex() {
  return (
    <>
      <h1>Overview for Atelier Sophie</h1>
      <ul>
        <li>
          <Link to="/sophie/items">Items</Link>
        </li>
        <li>
          <Link to="/sophie/friends">Friend presents</Link>
        </li>
      </ul>
    </>
  );
}
