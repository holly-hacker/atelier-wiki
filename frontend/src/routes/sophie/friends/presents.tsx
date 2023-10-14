import Grid from "@/components/grid";
import { SophieContext } from "@/data/sophie_data";
import { createColumnHelper } from "@tanstack/react-table";
import { useContext, useState } from "react";
import { ItemLink } from "../utility_components/links";

export default function FriendPresentList() {
  const sophieData = useContext(SophieContext);
  const [showBadGifts, setShowBadGifts] = useState(false);

  return (
    <>
      <h1>Friends</h1>
      <p>
        <input
          type="checkbox"
          checked={showBadGifts}
          onChange={() => setShowBadGifts(!showBadGifts)}
        />
        <label>Show gifts with negative values</label>
      </p>
      {Object.keys(sophieData.present_info.friend_present_info).map(
        (friend, i) => {
          return (
            <FriendPresentInfoDisplay
              key={i}
              friend={friend}
              showBadGifts={showBadGifts}
            />
          );
        },
      )}
    </>
  );
}

function FriendPresentInfoDisplay({
  friend,
  showBadGifts,
}: {
  friend: string;
  showBadGifts: boolean;
}) {
  const sophieData = useContext(SophieContext);
  const present_info = sophieData.present_info.friend_present_info[friend];

  const items = Object.entries(present_info.item_points)
    .map(([item_tag, score]) => {
      return { item_tag, score };
    })
    .filter((i) => showBadGifts || i.score >= 0);
  items.sort((a, b) => b.score - a.score);
  const itemColumnHelper = createColumnHelper<(typeof items)[0]>();
  const itemColumns = [
    itemColumnHelper.accessor("item_tag", {
      header: "Name",
      cell: (i) => {
        const item = sophieData.items.find((v) => v.tag == i.getValue())!;
        return <ItemLink item={item} />;
      },
    }),
    itemColumnHelper.accessor("score", { header: "Score" }),
  ];

  return (
    <>
      <h2>{friend.substring("FRIEND_".length)}</h2>
      <p>
        Friendship level starts at {present_info.default_points} points and can
        go up to {present_info.default_limit}.
      </p>
      {present_info.unlockable_limits.length > 0 && (
        <p>
          The following friendship level limits can be unlocked through certain
          events:
          <ul>
            {present_info.unlockable_limits.map(([level, event], i) => (
              <li key={i}>
                <code>{event}</code>: {level}
              </li>
            ))}
          </ul>
        </p>
      )}
      <h3>Items</h3>
      <p>
        <table>
          <tr>
            <th>Attack</th>
            <th>Heal</th>
            <th>Support</th>
            <th>Field</th>
            <th>Mix</th>
            <th>Machine</th>
            <th>Weapon</th>
            <th>Armor</th>
            <th>Accessory</th>
            <th>Material</th>
          </tr>
          <tr>
            <td>{present_info.base_points.attack}</td>
            <td>{present_info.base_points.heal}</td>
            <td>{present_info.base_points.support}</td>
            <td>{present_info.base_points.field}</td>
            <td>{present_info.base_points.mix}</td>
            <td>{present_info.base_points.machine}</td>
            <td>{present_info.base_points.weapon}</td>
            <td>{present_info.base_points.armor}</td>
            <td>{present_info.base_points.accessory}</td>
            <td>{present_info.base_points.material}</td>
          </tr>
        </table>
      </p>
      <p>The following items give additional points when gifted:</p>
      <Grid columns={itemColumns} data={items} />
    </>
  );
}
