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
        <li>
          <Link to="/sophie/rumors">Rumors</Link>
        </li>
        <li>
          <Link to="/sophie/dolls">Doll Making</Link>
        </li>
      </ul>
      <h2>Tools</h2>
      <ul>
        <li>
          <Link to="/sophie/tools/synth-graph">Trait transfer path finder</Link>
        </li>
      </ul>
    </>
  );
}
