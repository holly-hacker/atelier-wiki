use anyhow::Context;
use serde::Serialize;
use tracing::info;
use typescript_type_def::TypeDef;

use crate::utils::PakIndex;

use super::executable::Ryza3ExecutableData;

mod enemies;
mod item_categories;
mod item_effects;
mod items;
mod recipes;
mod strings_table;
mod util;

#[derive(Serialize, TypeDef)]
pub struct Ryza3Data {
    pub item_data: Vec<items::Item>,
    pub item_category_data: item_categories::ItemCategoryData,
    pub item_effect_data: item_effects::ItemEffectData,
    pub recipe_data: recipes::RecipeData,
    pub enemy_data: Vec<enemies::Enemy>,
}

impl Ryza3Data {
    pub fn read_all(
        pak_index: &mut PakIndex,
        executable_data: &Ryza3ExecutableData,
    ) -> anyhow::Result<Self> {
        // TODO: consider reading other languages too
        let strings_table = strings_table::StringsTable::read(pak_index).context("read strings")?;

        info!(
            "Read {} strings by id and {} strings by number",
            strings_table.id_lookup.len(),
            strings_table.no_lookup.len()
        );

        // NOTE: itemdata_no appears to be the exact same file
        let item_data = items::Item::read(pak_index, &strings_table).context("read items")?;
        info!("Read data for {} items", item_data.len());

        let item_category_data =
            item_categories::ItemCategoryData::read(executable_data, &strings_table)
                .context("read item categories")?;
        info!(
            "Read data for {} item categories",
            item_category_data.categories.len()
        );

        let item_effect_data =
            item_effects::ItemEffectData::read(pak_index, executable_data, &strings_table)
                .context("read item effects")?;
        info!(
            "Read data for {} item effects",
            item_effect_data.item_effects.len()
        );

        let recipe_data =
            recipes::RecipeData::read(pak_index, &strings_table).context("read recipes")?;
        info!("Read data for {} recipes", recipe_data.recipes.len());

        let enemy_data = enemies::read(pak_index, &strings_table).context("read enemies")?;
        info!("Read data for {} enemies", enemy_data.len());

        Ok(Self {
            item_data,
            item_category_data,
            item_effect_data,
            recipe_data,
            enemy_data,
        })
    }
}
