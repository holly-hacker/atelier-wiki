import { SophieContext } from "@/data/sophie_data";
import { ItemTypes } from "@/data/types/sophie";
import typesManual from "@/data/types/sophie-manual";
import { containsJapaneseDigit } from "@/util";
import { Graph, alg } from "@dagrejs/graphlib";
import { useContext, useMemo, useState } from "react";
import { ItemLink, TextureAtlasImage } from "../utility_components/links";

export function SynthGraph() {
  const sophieData = useContext(SophieContext);
  const ingredients = sophieData.ingredients;
  const items = sophieData.items.filter((v) => !containsJapaneseDigit(v.name));
  const categories = sophieData.categories;

  const itemsSorted = useMemo(
    () => [...items].sort((a, b) => a.name.localeCompare(b.name)),
    [items],
  );

  const [startItem, setStartItem] = useState("ITEM_MAT_FORGOTTEN_JEWEL"); // useState(itemsSorted[0].tag);
  const [endItem, setEndItem] = useState(itemsSorted[0].tag);
  const [disallowedItems, setDisallowedItems] = useState<string[]>([]);

  const g = useMemo(
    () => createGraph(ingredients, items, categories, disallowedItems),
    [ingredients, items, categories, disallowedItems],
  );

  const result = alg.dijkstra(
    g,
    startItem,
    (e) => (e.w.startsWith("ITEM_CATEGORY_") ? 0 : 1),
    undefined,
  );

  console.log("path found", result[endItem].distance !== Infinity);

  let path: string[] = [];
  if (result[endItem].distance !== Infinity) {
    path.push(endItem);
    let current = endItem;
    while (current !== startItem) {
      current = result[current].predecessor;
      path.push(current);
    }
    path.reverse();
    path = path.filter((x) => !x.startsWith("ITEM_CATEGORY_"));
  }

  return (
    <>
      <h1>Trait transfer path finder</h1>
      <p>
        This tool allows you to find paths to carry traits between items, which
        can be useful for completing the recipe book.
      </p>

      <ul>
        <li>
          Start item:{" "}
          <select
            value={startItem}
            onChange={(e) => setStartItem(e.target.value)}
          >
            {itemsSorted.map((v, i) => (
              <option key={i} value={v.tag}>
                {v.name}
              </option>
            ))}
          </select>
        </li>
        <li>
          End item:{" "}
          <select value={endItem} onChange={(e) => setEndItem(e.target.value)}>
            {itemsSorted.map((v, i) => (
              <option key={i} value={v.tag}>
                {v.name}
              </option>
            ))}
          </select>
        </li>
      </ul>

      <h2>Disallowed items</h2>
      <p>
        These items will not be used in the search. This can be useful to
        eliminate items that are too expensive to synthesize.
      </p>

      {disallowedItems.length > 0 && (
        <button onClick={() => setDisallowedItems([])}>Clear all</button>
      )}
      <ul>
        {disallowedItems.length === 0 && (
          <li>None yet, add items with the buttons below</li>
        )}
        {disallowedItems.map((v, i) => (
          <li key={i}>
            <button
              onClick={() =>
                setDisallowedItems(disallowedItems.filter((x) => x !== v))
              }
            >
              Remove
            </button>{" "}
            <ItemLink item={items.find((x) => x.tag === v)!} />
          </li>
        ))}
      </ul>

      {path.length > 0 ? (
        <>
          <h2>Found path</h2>
          <ol>
            {path.map((item_tag, i) => {
              const item = items.find((v) => v.tag === item_tag)!;
              return (
                <li key={i}>
                  <TextureAtlasImage
                    texture_atlas={sophieData.items_texture_atlas}
                    texture_atlas_name="items"
                    name={String(item.image_no)}
                  />
                  <button
                    onClick={() =>
                      setDisallowedItems([...disallowedItems, item_tag])
                    }
                  >
                    Ignore
                  </button>{" "}
                  <ItemLink item={item}>{item.name}</ItemLink> (
                  <code>{item_tag}</code>)
                </li>
              );
            })}
          </ol>
        </>
      ) : (
        "No path found"
      )}
    </>
  );
}

function createGraph(
  ingredients: typesManual.Ingredients,
  items: ItemTypes.Item[],
  categories: typesManual.Categories,
  disallowedItems: string[],
) {
  const g = new Graph({
    directed: true,
    multigraph: true,
  });

  // add all items and categories
  for (const item of items) {
    for (const category of Object.keys(categories)) {
      g.setNode(category, categories[category].name);
    }
    g.setNode(item.tag, item.name);
  }

  // add edges for item to category
  for (const item of items) {
    if (disallowedItems.includes(item.tag)) {
      continue;
    }
    for (const category of item.categories) {
      g.setEdge(item.tag, category);
    }
  }

  // add edges from each ingredient to the item
  for (const item_tag of Object.keys(ingredients)) {
    const ingredientsArray = ingredients[item_tag];
    for (const ingredient of ingredientsArray) {
      g.setEdge(ingredient.ingredient, item_tag);
    }
  }

  console.log(`Graph has ${g.nodeCount()} nodes and ${g.edgeCount()} edges.`);

  return g;
}
