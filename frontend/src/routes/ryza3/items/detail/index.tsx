import { Link, useParams } from "react-router-dom";
import { getImageLink, itemDisplayName } from "../../ryza3_data_util";
import type { ItemTypes } from "@/data/types/ryza3";
import { CategoryLink, ItemLink } from "../../utility_components/links";
import { useContext } from "react";
import { Ryza3Context } from "@/data/ryza3_data";
import { ItemRecipeSection } from "./recipe";
import { ItemDropSourcesSection } from "./sources";

export default function ItemDetail() {
  const ryza3Data = useContext(Ryza3Context);
  const { id } = useParams();

  let item: ItemTypes.Item | undefined;
  if (id && !isNaN(Number(id))) {
    // id is a number
    item = ryza3Data.items[Number(id)];
  } else if (id) {
    // try to find by item tag
    const tag = `ITEM_${id}`;
    item = ryza3Data.items.find((v) => v.tag == tag);
  }

  if (!item) {
    return <>Item not found</>;
  }

  return (
    <>
      <h1>{itemDisplayName(item)}</h1>
      {item.library_note && <p>{item.library_note}</p>}
      {item.img_no !== null && item.img_no >= 0 && (
        <img src={getImageLink(`items/${item.img_no}.png`)}></img>
      )}

      <ItemDetailSection item={item} />
      {/* the "Usage In Recipe" section is usually way smaller than the recipe section, so show it first */}
      <ItemReverseRecipeSection item={item} />
      <ItemRecipeSection item={item} />
      <ItemDropSourcesSection item={item} />
    </>
  );
}

function ItemDetailSection({ item }: { item: ItemTypes.Item }) {
  return (
    <>
      <h2>Details</h2>
      <ul>
        {item.tag && (
          <li>
            Item tag: <code>{item.tag}</code>
          </li>
        )}
        <li>Price: {item.price}</li>
        <li>Level: {item.lv}</li>
        <li>
          Elements: {item.elem_fire && "🔥 "}
          {item.elem_ice && "❄️ "}
          {item.elem_thunder && "⚡ "}
          {item.elem_air && "🍃 "}
        </li>
        <li>
          Use tag:{" "}
          <Link to={`/ryza3/item_use_tags/${item.use_tag}`}>
            <code>{item.use_tag}</code>
          </Link>
        </li>
        <li>
          Kind tag:{" "}
          <Link to={`/ryza3/item_kinds/${item.kind_tag}`}>
            <code>{item.kind_tag}</code>
          </Link>
        </li>
        <li>
          Categories:
          <ul>
            {item.cat.map((cat, i) => {
              return (
                <li key={i}>
                  <CategoryLink category_tag={cat} />
                </li>
              );
            })}
          </ul>
        </li>
      </ul>
      <details>
        <summary>Json data</summary>
        <pre>{JSON.stringify(item, null, 4)}</pre>
      </details>
    </>
  );
}

function ItemReverseRecipeSection({ item }: { item: ItemTypes.Item }) {
  const ryza3Data = useContext(Ryza3Context);

  // typescript is a bit buggy, it doesn't know that item cannot be undefined due to the guard
  // earlier. See microsoft/TypeScript#9998
  const reverse_recipes = ryza3Data.recipes.recipes.filter(
    (r) =>
      r.ingredients.some((i) => i.tag == item!.tag) ||
      r.fields.flatMap((r) => r).some((r) => r.explicit_material == item!.tag),
  );

  const header = <h2>Usage in recipes</h2>;

  if (reverse_recipes.length == 0) {
    return (
      <>
        {header}
        <p>This item is not directly used in any recipes.</p>
      </>
    );
  }

  return (
    <>
      {header}
      <ul>
        {reverse_recipes.map((recipe, i) => {
          // detect items that are only used with type 6, which is recipe morph (called Imagined recipe) by the game.
          // if an item is only used for recipe morphs, show an indication becsause usually you wont be using them.

          // NOTE: it seems that some items (eg. `ITEM_MIX_WIND_SHOES`) incorrectly list their core ingredients to
          // include items they don't use (`ITEM_MIX_MATERIAL_056`/Spirit Bottle), so we need to account for cases where
          // the item is not used at all in the recipe.
          const is_recipe_upgrade = recipe.fields
            .flatMap((field) => field)
            .filter(
              (ring) =>
                (ring.explicit_material !== null &&
                  ring.explicit_material == item.tag) ||
                (ring.restrict !== null &&
                  recipe.ingredients[ring.restrict].tag == item.tag),
            )
            .reduce(
              (acc, r) => (acc ?? false) && r.effect_type == 6,
              null as boolean | null,
            );

          return (
            <li key={i}>
              <ItemLink item={recipe.item_tag} />
              {is_recipe_upgrade === true && " (recipe morph)"}
              {is_recipe_upgrade === null && " (not actually used)"}
            </li>
          );
        })}
      </ul>
    </>
  );
}
