//! Files of interest:
//! - [x] item\itemrecipedata.xml: Contains item recipes. Uses a weird structure.
//! - [x] mix\mixfielddata.xml: Contains actual mix data

use std::collections::{BTreeMap, BTreeSet};

use anyhow::Context;
use serde::Serialize;
use tracing::debug;
use typescript_type_def::TypeDef;

use crate::utils::PakIndex;

use self::{
    itemrecipedata::{IngredientData, RecipeHeader},
    mixfielddata::Field,
};

use super::strings_table::StringsTable;

mod itemrecipedata;
mod mixfielddata;

#[derive(Serialize, TypeDef)]
pub struct RecipeData {
    pub recipes: Vec<Recipe>,

    /// A lookup table for feature descriptions.
    pub feature_descriptions: BTreeMap<u32, FeatureDescription>,
}

#[derive(Serialize, TypeDef)]
pub struct Recipe {
    /// The item that this recipe crafts.
    pub item_tag: String,
    /// The sorting order of this recipe in the alchemy menu, if it is present there.
    pub sort: Option<i32>,
    /// The base amount of the item that is crafted.
    pub make_num: u32,
    /// The amount of time it takes to craft the item.
    pub hour: u32,
    /// The category that this recipe is in
    pub recipe_category: String,

    /// The core ingredients for this recipe.
    ///
    /// At most 4 ingredients will be present, while the lower limit is likely 3.
    pub ingredients: Vec<RecipeIngredient>,

    /// The secret key info for this recipe, if it is applicable.
    pub secret_key_info: Option<[SecretKeyInfo; 3]>,

    /// The fields of this recipe, each containing a set of rings/material loops..
    pub fields: Vec<Vec<Ring>>,
}

#[derive(Serialize, TypeDef)]
pub struct RecipeIngredient {
    /// The item or category tag of this ingredient.
    pub tag: String,
    /// Whether the tag refers to a category or an item.
    pub is_category: bool,
    /// The effect that is added by default, even if no material loops are filled in.
    pub initial_effect: Option<String>,
    /// The effect tags that this item adds.
    pub additional_effects: Vec<String>,
}

/// The secret key info for a recipe.
#[derive(Serialize, TypeDef, Default)]
pub struct SecretKeyInfo {
    /// The requirement for unlocking this level.
    ///
    /// The type of requirement can be determined by the prefix of the string:
    /// - `ITEM_ELEM_`: Element of the key
    /// - `SECRET_KEY_MOTIF_`: Motif of the key
    /// - `SECRET_KEY_RARITY_`: Rarity of the key
    pub requirement: String,

    /// The reward for unlocking this level
    pub reward: String,
}

/// A ring in a recipe, also called a material loop.
// TODO: currently this model is built on the assumption that every ring is part of the normal
// layout. In reality, there are many rings that have incomplete data that are probably still
// important but I don't know how they're used.
#[derive(Serialize, TypeDef)]
pub struct Ring {
    /// The element.
    ///
    /// 0: Fire
    /// 1: Ice
    /// 2: Bolt
    /// 3: Wind
    pub element: u32,
    /// The effect type of the ring.
    ///
    /// See `str_mix_feature_description` for more info.
    pub effect_type: u32,
    /// Whether this is a core loop and must be filled in for the recipe to be completed.
    pub required: bool,
    /// The X coordinate, where negative is left and positive is right.
    pub x: i32,
    /// The Y coordinate, where negative is up and positive is down.
    pub y: i32,
    /// The pre-defined item or category to use, as defined in `itemrecipedata.xml`. Mutually
    /// exclusive with `explicit_material`.
    pub restrict: Option<u32>,
    /// The explicit material to use. This will be a different material than the 4 that are
    /// pre-defined in the recipe. Mutually exclusive with `restrict`.
    pub explicit_material: Option<String>,

    /// The predecessor ring that this ring is connected to.
    ///
    /// Each ring has at most 1 predecessor, and this predecessor must be unlocked before this ring
    /// can be unlocked. There may also be additional requirements before it can be unlocked.
    pub predecessor: Option<RingPredecessor>,

    /// The parameters for the effects of this ring. Related to the `type`.
    pub effect_parameters: Vec<RingParameter>,
}

#[derive(Serialize, TypeDef)]
pub struct RingPredecessor {
    /// The direction of this node's predecessor.
    ///
    /// This is an index in a clockwise direction, like so:
    /// - 0: above (y - 2)
    /// - 1: top right (x + 1, y - 1)
    /// - 2: bottom right (x + 1, y + 1)
    /// - 3: below (y + 2)
    /// - 4: bottom left (x - 1, y + 1)
    /// - 5: top left (x - 1, y - 1)
    pub direction: u32,

    /// The requirement element value in the predecessor before this ring can be unlocked.
    pub required_value: Option<u32>,

    /// The requirement element in the predecessor before this ring can be unlocked.
    pub required_element: Option<u32>,

    /// The requirement quality in the recipe before this ring can be unlocked.
    pub required_quality: Option<u32>,
}

#[derive(Serialize, TypeDef)]
pub struct RingParameter {
    /// The value of the parameter. This could be a numeric value or a string that refers to an
    /// item.
    pub value: String,

    /// The element value required for this tier to be met. This is additive with the previous
    /// tiers.
    pub element_value: u32,

    /// Whether this element is hidden.
    pub hidden: bool,
}

#[derive(Serialize, TypeDef)]
pub struct FeatureDescription {
    /// The name of the effect.
    short_name: Option<String>,
    short_description: Option<String>,
    /// The format string for the effect, as shown in the Loop Info window.
    ///
    /// For effects 1-4, this will be `null` and the item effect's descripion will be used instead.
    loop_info_format: Option<String>,
    description: Option<String>,
}

impl RecipeData {
    pub fn read(pak_index: &mut PakIndex, strings: &StringsTable) -> anyhow::Result<Self> {
        debug!("Reading item recipe data");
        let recipe_data =
            itemrecipedata::ItemRecipeData::read(pak_index).context("read item recipe data")?;

        debug!("Reading mix field data");
        let mix_field_data =
            mixfielddata::ExtendedFieldData::read(pak_index).context("read item recipe data")?;

        debug!("Mapping feature descriptions");
        let feature_descriptions = get_feature_descriptions(&mix_field_data, strings)
            .context("read mix feature descriptions")?;

        debug!("Mapping recipe data");
        let recipes = map_recipes(recipe_data, mix_field_data).context("map recipes")?;

        Ok({
            Self {
                recipes,
                feature_descriptions,
            }
        })
    }
}

fn map_recipes(
    recipe_data: Vec<itemrecipedata::ItemRecipeData>,
    mix_field_data: mixfielddata::ExtendedFieldData,
) -> anyhow::Result<Vec<Recipe>> {
    let mut ret = vec![];

    let mut iter = recipe_data.into_iter().peekable();

    while iter.peek().is_some() {
        let header = match iter.next().unwrap() {
            itemrecipedata::ItemRecipeData::Header(header) => header,
            _ => panic!(
                "Expected header but found ingredient. Does recipe data start with ingredient?"
            ),
        };

        // NOTE: can be optimized, but this wont be a bottleneck
        let mut other_ingredients = vec![];
        while let Some(ingredient) =
            iter.next_if(|d| matches!(d, itemrecipedata::ItemRecipeData::Ingredient(_)))
        {
            let ingredient = match ingredient {
                itemrecipedata::ItemRecipeData::Ingredient(ingredient) => ingredient,
                _ => unreachable!(),
            };
            other_ingredients.push(ingredient);
        }

        let fields = mix_field_data
            .0
            .get(&header.item_tag)
            .map(|f| f.as_slice())
            .unwrap_or_default();

        ret.push(map_recipe_data(header, other_ingredients, fields)?);
    }

    Ok(ret)
}

fn map_recipe_data(
    header: RecipeHeader,
    other_ingredients: Vec<IngredientData>,
    recipe_fields: &[Field],
) -> anyhow::Result<Recipe> {
    let mut secret_key_info = if header.first_ingredient.rl_reward.is_some() {
        Some([
            SecretKeyInfo::default(),
            SecretKeyInfo::default(),
            SecretKeyInfo::default(),
        ])
    } else {
        None
    };

    let chained_ingredients = [header.first_ingredient]
        .into_iter()
        .chain(other_ingredients);

    let mut ingredients = vec![];
    for (i, ingredient) in chained_ingredients.enumerate() {
        let mut recipe_ingredient = RecipeIngredient {
            tag: ingredient.mat_tag,
            is_category: ingredient.is_category,
            additional_effects: ingredient.add_eff,
            initial_effect: ingredient
                .mass_effect
                .filter(|e| e != "ITEM_EFF_EFFECT_NONE"),
        };

        // trim `ITEM_EFF_EFFECT_NONE` entries from the end of additional_effects
        // these entries don't add information and are likely just padding
        while let Some(last) = recipe_ingredient.additional_effects.last() {
            if last == "ITEM_EFF_EFFECT_NONE" {
                recipe_ingredient.additional_effects.pop();
            } else {
                break;
            }
        }

        ingredients.push(recipe_ingredient);

        // update secret key, if applicable
        if let Some(key) = &mut secret_key_info {
            if i < key.len() {
                key[i].requirement = ingredient
                    .rl_cond_sk_elem
                    .or(ingredient.rl_cond_sk_motif)
                    .or(ingredient.rl_cond_sk_rarity)
                    .expect("at least 1 requirement must be found");
                key[i].reward = ingredient.rl_reward.unwrap();
            }
        }
    }

    // calculate the fields/rings
    let mut fields = vec![];

    for recipe_field in recipe_fields {
        let mut rings = vec![];
        for ring in recipe_field.rings.iter() {
            let Some(ring) = ring else {
                continue;
            };

            let Some(element) = ring.elem else {
                continue;
            };

            let Some(effect_type) = ring.r#type else {
                continue;
            };

            if ring.x.is_none() && ring.y.is_none() && !ring.is_essential {
                // skip rings that are not essential at position 0,0, they are likely wrong
                continue;
            }

            let predecessor = ring.connect.as_ref().and_then(|c| {
                let direction = match (c.dir, c.idx) {
                    (Some(dir), _) => Some(dir),
                    (None, Some(idx)) => {
                        // if there is no direction but we have the index of the other node,
                        // calculate the direction instead.
                        let curr_pos = (ring.x.unwrap_or_default(), ring.y.unwrap_or_default());
                        let prev = recipe_field.rings[idx as usize]
                            .as_ref()
                            .expect("reference to non-existent ring");
                        let prev_pos = (prev.x.unwrap_or_default(), prev.y.unwrap_or_default());

                        Some(match (prev_pos.0 - curr_pos.0, prev_pos.1 - curr_pos.1) {
                            (0, -2) => 0,
                            (1, -1) => 1,
                            (1, 1) => 2,
                            (0, 2) => 3,
                            (-1, 1) => 4,
                            (-1, -1) => 5,
                            _ => panic!("invalid direction"),
                        })
                    }
                    (None, None) => {
                        // neither dir nor idx is present, assume we can't place it
                        None
                    }
                };

                direction.map(|direction| RingPredecessor {
                    direction,
                    required_value: c.val,
                    required_element: c.elem,
                    required_quality: c.qual,
                })
            });

            let effect_parameters = ring
                .param
                .as_ref()
                .map(|p| {
                    let max_index = p.v.len().max(p.e.len()).max(p.n.len());

                    // scan serves as take_while_some here
                    // we want to take values until we get a None for value or element value
                    // those are likely errors, so skip them
                    (0..max_index)
                        .scan((), |_, i| {
                            let value = p.v.get(i).cloned().flatten();
                            let element_value = p.e.get(i).cloned().flatten();

                            let Some(value) = value else {
                                return None;
                            };
                            let Some(element_value) = element_value else {
                                return None;
                            };

                            Some(RingParameter {
                                value,
                                element_value,
                                hidden: p.n.get(i).cloned().unwrap_or_default(),
                            })
                        })
                        .collect::<Vec<_>>()
                })
                .unwrap_or_default();

            rings.push(Ring {
                element,
                effect_type,
                required: ring.is_essential,
                x: ring.x.unwrap_or_default(),
                y: ring.y.unwrap_or_default(),
                restrict: ring.restrict,
                explicit_material: ring.ex_material.clone(),
                predecessor,
                effect_parameters,
            })
        }

        fields.push(rings);
    }

    Ok(Recipe {
        item_tag: header.item_tag,
        sort: header.sort,
        make_num: header.make_num,
        hour: header.hour,
        recipe_category: header.recipe_category,
        ingredients,
        secret_key_info,
        fields,
    })
}

fn get_feature_descriptions(
    mix_field_data: &mixfielddata::ExtendedFieldData,
    strings: &StringsTable,
) -> anyhow::Result<BTreeMap<u32, FeatureDescription>> {
    let mut map = BTreeMap::<u32, FeatureDescription>::default();

    let types = mix_field_data
        .0
        .values()
        .flatten()
        .flat_map(|f| &f.rings)
        .flatten()
        .flat_map(|r| &r.r#type)
        .collect::<BTreeSet<_>>();

    for type_id in types {
        let short_name = strings
            .id_lookup
            .get(&format!("STR_MIX_FEATURE_DESCRIPTION_{:03}", type_id))
            .with_context(|| format!("Get short name for id {type_id}"))?
            .clone();
        let short_description = strings
            .id_lookup
            .get(&format!("STR_MIX_FEATURE_DESCRIPTION_{:03}", type_id + 40))
            .cloned();
        let loop_info_format = strings
            .id_lookup
            .get(&format!("STR_MIX_FEATURE_DESCRIPTION_{:03}", type_id + 80))
            .cloned();
        let description = strings
            .id_lookup
            .get(&format!("STR_MIX_FEATURE_DESCRIPTION_{:03}", type_id + 120))
            .cloned();

        map.insert(
            *type_id,
            FeatureDescription {
                short_name: Some(short_name).filter(|x| !x.is_empty()),
                short_description: short_description.filter(|x| !x.is_empty()),
                loop_info_format,
                description,
            },
        );
    }

    Ok(map)
}
