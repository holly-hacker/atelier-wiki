import { SophieContext } from "@/data/sophie_data";
import types from "@/data/types/sophie";
import { useContext } from "react";
import { useParams } from "react-router-dom";
import { CategoryLink } from "../utility_components/links";
import { getImageLink, itemDisplayName } from "../sophie_data_util";

export default function ItemDetail(): JSX.Element {
  const sophieData = useContext(SophieContext);
  const { id } = useParams();

  let item: types.Item | undefined;
  if (id && !isNaN(Number(id))) {
    // id is a number
    item = sophieData.items[Number(id)];
  } else if (id) {
    // try to find by item tag
    const tag = `ITEM_${id}`;
    item = sophieData.items.find((v) => v.tag == tag);
  }

  if (!item) {
    return <>Item not found</>;
  }

  return (
    <>
      <h1>{itemDisplayName(item)}</h1>
      {item.image_no !== null && item.image_no >= 0 && (
        <img src={getImageLink(`items/${item.image_no}.png`)}></img>
      )}

      <h2>Details</h2>
      <ItemDetailSection item={item} />

      <h2>Recipe</h2>
      <RecipeSection item={item} />
    </>
  );
}

function ItemDetailSection({ item }: { item: types.Item }) {
  return (
    <>
      <ul>
        {item.tag && (
          <li>
            Item tag: <code>{item.tag}</code>
          </li>
        )}
        <li>Price: {item.cost}</li>
        <li>Level: {item.level}</li>
        <li>
          Element:{" "}
          <code>{colorToEmoji(item.color.split("_").slice(-1)[0])}</code>
        </li>
        <li>
          Use tag: <code>{item.use_type}</code>
        </li>
        <li>
          Kind tag: <code>{item.base}</code>
        </li>
        <li>
          Categories:
          <ul>
            {item.categories.map((cat, i) => {
              return (
                <li key={i}>
                  <CategoryLink category_tag={cat} />
                </li>
              );
            })}
          </ul>
        </li>
      </ul>
      <h3>Shape</h3>
      <ItemShape item={item} />
      <details>
        <summary>Json data</summary>
        <pre>{JSON.stringify(item, null, 4)}</pre>
      </details>
    </>
  );
}

function ItemShape({ item }: { item: types.Item }) {
  const sophieData = useContext(SophieContext);

  if (item.shape_type == "ITEM_SHAPE_TYPE_DAMMY")
    return <p>Item does not have a shape</p>;

  const shape = sophieData.shapes[item.shape_type];

  if (!shape)
    return (
      <>
        No shape found (<code>{item.shape_type}</code>)
      </>
    );

  const shapes = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

  return (
    <table>
      <tr>
        {shapes.map((size) => (
          <th key={size}>{size}</th>
        ))}
      </tr>
      <tr>
        {shapes.map((size) => (
          <td key={size}>
            <Shape shape={shape} size={size} />
          </td>
        ))}
      </tr>
    </table>
  );
}

function Shape({ shape, size }: { shape: number[]; size: number }) {
  const shapeSubset = shape.slice(0, size);

  const on = "◼";
  const off = "◻";
  return (
    <pre style={{ margin: 0, lineHeight: 0.8 }}>
      {shapeSubset.indexOf(0) !== -1 ? on : off}
      {shapeSubset.indexOf(1) !== -1 ? on : off}
      {shapeSubset.indexOf(2) !== -1 ? on : off}
      {"\n"}
      {shapeSubset.indexOf(3) !== -1 ? on : off}
      {shapeSubset.indexOf(4) !== -1 ? on : off}
      {shapeSubset.indexOf(5) !== -1 ? on : off}
      {"\n"}
      {shapeSubset.indexOf(6) !== -1 ? on : off}
      {shapeSubset.indexOf(7) !== -1 ? on : off}
      {shapeSubset.indexOf(8) !== -1 ? on : off}
    </pre>
  );
}

function RecipeSection({ item }: { item: types.Item }) {
  const sophieData = useContext(SophieContext);
  const board = sophieData.item_boards[item.tag];

  if (!board) return <>No recipe found</>;

  return (
    <>
      <h3>Colors</h3>
      <pre>
        {board.colors.map(
          (color) => [...color].map((c) => colorToEmoji(c)).join("") + "\n",
        )}
      </pre>
      <table>
        <tr>
          {[0, 1, 2].map((i) => (
            <th key={i}>Bonus Level {i + 1}</th>
          ))}
        </tr>
        <tr>
          {[0, 1, 2].map((i) => (
            <td key={i}>
              <pre style={{ margin: 0 }}>
                {board.bonus_levels[i].map((level) => (
                  <>
                    {[...level].map((l, x) =>
                      l == " " ? (
                        <span key={x} style={{ color: "#888888" }}>
                          0
                        </span>
                      ) : (
                        <span key={x}>{l}</span>
                      ),
                    )}
                    {"\n"}
                  </>
                ))}
              </pre>
            </td>
          ))}
        </tr>
      </table>
    </>
  );
}

function colorToEmoji(color: string) {
  switch (color) {
    case "R":
    case "RED":
      return "🟥";
    case "G":
    case "GREEN":
      return "🟩";
    case "B":
    case "BLUE":
      return "🟦";
    case "Y":
    case "YELLOW":
      return "🟨";
    case "W":
    case "WHITE":
      return "⬜";
    case " ":
      return "⬛";
    default:
      return color;
  }
}
