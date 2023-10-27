import Grid from "@/components/grid";
import { SophieContext, SophieData } from "@/data/sophie_data";
import { createColumnHelper } from "@tanstack/react-table";
import { useContext, useState } from "react";
import { ItemLink } from "../utility_components/links";
import type SophieTypes from "@/data/types/sophie.d.ts";

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

// this should be extracted from the game, but it seems to be embedded in the executable
function mapFriendName(name: string) {
  switch (name) {
    case "CANARIA":
      return "Corneria";
    case "ELISE": // Librarian
      return "Elise Phulie";
    case "FILON":
      return "Fritz Weissberg";
    case "HELLMUT": // Cafe owner
      return "Horst Basler";
    case "HENRI":
      return "Amelia Leonmeyer (Leon)";
    case "JULIO":
      return "Julio Sebald Leidenschaft";
    case "LOGIX": // Blacksmith
      return "Logix Ficsario (Logy)";
    case "MARGRIT": // Grocery store owner
      return "Marguerite Behlmer";
    case "MONIKA":
      return "Monika Ellmenreich";
    case "OTTO":
      return "Oskar Behlmer";
    case "PAMELA": // Nun
      return "Pamela Ibis";
    case "PAUL":
      return "Harol Simens";
    case "PLACHTA":
      return "Plachta";
    case "TESS": // Bunny girl
      return "Tess Heitzmann";
    default:
      return name;
  }
}

function getWeightedScore(
  {
    item_tag,
    score,
  }: {
    item_tag: string;
    score: number;
  },
  sophieData: SophieData,
  present_info: SophieTypes.FriendPresentInfo,
): number {
  const item = sophieData.items.find((v) => v.tag == item_tag)!;
  const base_points = present_info.base_points as Record<string, number>;
  const base_score = base_points[
    item.base.substring("ITEM_KIND_".length).toLowerCase()
  ] as number;
  return base_score * score;
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
  const itemColumnHelper = createColumnHelper<(typeof items)[0]>();
  const itemColumns = [
    itemColumnHelper.accessor("item_tag", {
      header: "Name",
      cell: (i) => <ItemLink item={i.getValue()} />,
    }),
    itemColumnHelper.accessor("item_tag", {
      header: "Item Type",
      cell: (i) => {
        const item = sophieData.items.find((v) => v.tag == i.getValue())!;
        return <code>{item.base.substring("ITEM_KIND_".length)}</code>;
      },
    }),
    itemColumnHelper.accessor("score", { header: "Raw score" }),
    itemColumnHelper.accessor(
      (x) => getWeightedScore(x, sophieData, present_info),
      {
        header: "Score",
        id: "score_weighted",
      },
    ),
  ];

  return (
    <>
      <h2>{mapFriendName(friend.substring("FRIEND_".length))}</h2>
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
      <Grid
        columns={itemColumns}
        data={items}
        initialSortingState={[{ id: "score_weighted", desc: true }]}
      />
    </>
  );
}
