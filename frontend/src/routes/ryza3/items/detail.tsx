import { Link, useParams } from "react-router-dom";
import items from "@/data/ryza3/items.json";
import item_effects from "@/data/ryza3/item_effects.json";
import enemies from "@/data/ryza3/enemies.json";
import recipes from "@/data/ryza3/recipes.json";
import {
  findItemByTag,
  getImageLink,
  itemDisplayName,
} from "../ryza3_data_util";
import types from "@/data/types/ryza3";
import { EnemyLink, ItemLink } from "../utility_components/links";
import RecipeDisplay from "../utility_components/recipe_display";

export default function ItemDetail() {
  const { id } = useParams();

  let item: types.Item | undefined;
  if (id && !isNaN(Number(id))) {
    // id is a number
    item = items[Number(id)];
  } else if (id) {
    // try to find by item tag
    const tag = `ITEM_${id}`;
    item = items.find((v) => v.tag == tag);
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

function ItemDetailSection({ item }: { item: types.Item }) {
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
                  <Link to={`/ryza3/item_categories/${cat}`}>
                    <code>{cat}</code>
                  </Link>
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

function ItemRecipeSection({ item }: { item: types.Item }) {
  const recipe = getRecipe(item);

  if (!recipe) {
    return <></>;
  }

  const explicit_recipe_items = recipe && [
    ...new Set(
      recipe.fields
        .flatMap((r) => r)
        .map((r) => r.explicit_material!)
        .filter((r) => r != null),
    ),
  ];

  const morph_targets = recipe.fields.flatMap((field, f_idx) =>
    field
      .filter((ring) => ring.effect_type == 6)
      .map((ring) => ({
        mat_tag: ring.explicit_material!,
        new_item_tag: ring.effect_parameters[0].value.substring(
          "ITEM_RECIPE_".length,
        ),
        requires_link_morph: f_idx != 0,
      })),
  );

  return (
    <>
      <h2>Recipe</h2>
      <ul>
        <li>
          Category: <code>{recipe.recipe_category}</code>
        </li>
        <li>Amount crafted: {recipe.make_num}</li>
        <li>
          Time to craft: {recipe.hour} hour{recipe.hour != 1 && "s"}
        </li>
      </ul>

      <h3>Morph targets</h3>
      {morph_targets.length > 0 ? (
        <ul>
          {morph_targets.map((target, i) => {
            return (
              <li key={i}>
                <ItemLink item={findItemByTag(target.new_item_tag)!} />
                {target.requires_link_morph && " (requires link morph)"}
              </li>
            );
          })}
        </ul>
      ) : (
        <>This item does not morph into other items.</>
      )}
      <h3>Core items with effects</h3>
      <ul>
        {recipe.ingredients.map((ingredient, i) => {
          return (
            <li key={i}>
              {ingredient.is_category ? (
                <Link to={`/ryza3/item_categories/${ingredient.tag}`}>
                  <code>{ingredient.tag}</code>
                </Link>
              ) : (
                <ItemLink item={findItemByTag(ingredient.tag)!} />
              )}
              <ul>
                {[
                  ingredient.initial_effect,
                  ...ingredient.additional_effects,
                ].map((effect_tag, i) => {
                  if (!effect_tag) {
                    return (
                      <li key={i}>
                        <em>No initial effect</em>
                      </li>
                    );
                  }

                  const effect = item_effects.item_effects[effect_tag];

                  const formatEffectAttributes = (
                    attr: types.EffectAttribute,
                  ) => {
                    const formatMinMax = (
                      min: string | null,
                      max: string | null,
                    ) => {
                      if (!min && !max) return null;
                      if (min == max) return `${min}`;
                      if (min && !max) return `${min}+`;
                      if (!min && max) return `<=${max}`;
                      return `${min}-${max}`;
                    };

                    const args = [
                      formatMinMax(attr.min_1, attr.max_1),
                      formatMinMax(attr.min_2, attr.max_2),
                    ]
                      .filter((v) => v != null)
                      .join(", ");
                    return `${attr.action}(${args})`;
                  };

                  return (
                    <li key={i}>
                      <b>{item_effects.item_effects[effect_tag].name}</b>
                      {" - "}
                      {item_effects.item_effects[effect_tag].description}{" "}
                      <ul>
                        {effect.attributes.map((a, i) => (
                          <li key={i}>
                            <code>{formatEffectAttributes(a)}</code>
                          </li>
                        ))}
                      </ul>
                    </li>
                  );
                })}
              </ul>
            </li>
          );
        })}
      </ul>
      {explicit_recipe_items && explicit_recipe_items.length != 0 && (
        <>
          <h3>Additional required materials</h3>
          <ul>
            {explicit_recipe_items.map((item_tag, i) => {
              return (
                <li key={i}>
                  <ItemLink item={findItemByTag(item_tag!)!} />
                </li>
              );
            })}
          </ul>
        </>
      )}
      <h3>Recipe grid</h3>
      <RecipeDisplay recipe={recipe} />
      <details>
        <summary>Json data</summary>
        <pre>{JSON.stringify(recipe, null, 4)}</pre>
      </details>
    </>
  );
}

function ItemReverseRecipeSection({ item }: { item: types.Item }) {
  // typescript is a bit buggy, it doesn't know that item cannot be undefined due to the guard
  // earlier. See microsoft/TypeScript#9998
  const reverse_recipes = recipes.recipes.filter(
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
              <ItemLink item={findItemByTag(recipe.item_tag)!} />
              {is_recipe_upgrade === true && " (recipe morph)"}
              {is_recipe_upgrade === null && " (not actually used)"}
            </li>
          );
        })}
      </ul>
    </>
  );
}

function ItemDropSourcesSection({ item }: { item: types.Item }) {
  const drops = getDrops(item);

  return (
    <>
      <h2>Monster drops</h2>
      {drops.length > 0 ? (
        <>
          <ul>
            {drops.map(({ drop, status, enemy }, i) => {
              return (
                <li key={i}>
                  <EnemyLink enemy={enemy}>
                    {enemy.name} (lv {status.lv})
                  </EnemyLink>
                  : Drop rate: {drop.num}x {drop.rate}%
                </li>
              );
            })}
          </ul>
          <details>
            <summary>Json data</summary>
            <pre>{JSON.stringify(drops, null, 4)}</pre>
          </details>
        </>
      ) : (
        <p>This item does not drop from monsters.</p>
      )}
    </>
  );
}

function getDrops(
  item: types.Item,
): { drop: types.EnemyDrop; status: types.EnemyStatus; enemy: types.Enemy }[] {
  if (!item.tag) return [];

  const drops = [];

  for (const enemy of enemies) {
    for (const status of enemy.statusses) {
      for (const drop of status.drops) {
        if (drop.item_tag == item.tag) {
          drops.push({ drop, status, enemy });
        }
      }
    }
  }

  return drops;
}

function getRecipe(item: types.Item): types.Recipe | null {
  if (!item.tag) return null;

  for (const recipe of recipes.recipes) {
    if (recipe.item_tag == item.tag) {
      return recipe;
    }
  }

  return null;
}
