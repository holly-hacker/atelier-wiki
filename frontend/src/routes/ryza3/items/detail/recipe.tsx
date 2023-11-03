import { findItemByTag } from "../../ryza3_data_util";
import type {
  ItemEffectTypes,
  ItemTypes,
  RecipeTypes,
} from "@/data/types/ryza3";
import { CategoryLink, ItemLink } from "../../utility_components/links";
import RecipeDisplay from "../../utility_components/recipe_display";
import { useContext } from "react";
import { Ryza3Context, Ryza3Data } from "@/data/ryza3_data";

export function ItemRecipeSection({ item }: { item: ItemTypes.Item }) {
  const ryza3Data = useContext(Ryza3Context);
  const recipe = getRecipe(ryza3Data, item);

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
                <ItemLink
                  item={findItemByTag(ryza3Data, target.new_item_tag)!}
                />
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
                <CategoryLink category_tag={ingredient.tag} />
              ) : (
                <ItemLink item={ingredient.tag} />
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

                  const effect =
                    ryza3Data.item_effects.item_effects[effect_tag];

                  const formatEffectAttributes = (
                    attr: ItemEffectTypes.EffectAttribute,
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

                    const trimmed_action = attr.action.replace(/^ACT_/, "");
                    return `${trimmed_action}(${args})`;
                  };

                  return (
                    <li key={i}>
                      <b>
                        {ryza3Data.item_effects.item_effects[effect_tag].name}
                      </b>
                      {" - "}
                      {
                        ryza3Data.item_effects.item_effects[effect_tag]
                          .description
                      }{" "}
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
                  <ItemLink item={item_tag} />
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

function getRecipe(
  ryza3Data: Ryza3Data,
  item: ItemTypes.Item,
): RecipeTypes.Recipe | null {
  if (!item.tag) return null;

  for (const recipe of ryza3Data.recipes.recipes) {
    if (recipe.item_tag == item.tag) {
      return recipe;
    }
  }

  return null;
}
