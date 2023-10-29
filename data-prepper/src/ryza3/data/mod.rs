use anyhow::Context;
use serde::Serialize;
use tracing::info;
use typescript_type_def::TypeDef;

use super::executable::Ryza3ExecutableData;
use crate::utils::PakIndex;

mod enemies;
mod feeding;
mod field_data;
mod field_map;
mod item_categories;
mod item_effects;
mod items;
mod quests;
mod recipes;
mod strings_table;

#[derive(Serialize, TypeDef)]
pub struct Ryza3Data {
    pub item_data: Vec<items::Item>,
    pub item_category_data: item_categories::ItemCategoryData,
    pub item_effect_data: item_effects::ItemEffectData,
    pub recipe_data: recipes::RecipeData,
    pub field_map: field_map::FieldMapData,
    pub field_data: field_data::FieldData,
    pub enemy_data: Vec<enemies::Enemy>,
    pub puni_feeding_data: feeding::PuniFeedingData,
    pub quests: quests::QuestData,
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

        let field_map =
            field_map::FieldMapData::read(pak_index, &strings_table).context("read field map")?;
        // info!("Read data for {} field data", field_map.0.len());
        info!("Read data for field map");

        let field_data = field_data::FieldData::read(pak_index).context("read field data")?;
        info!("Read data for {} field data", field_data.0.len());

        let enemy_data = enemies::read(pak_index, &strings_table).context("read enemies")?;
        info!("Read data for {} enemies", enemy_data.len());

        let puni_feeding_data = feeding::PuniFeedingData::read(pak_index, &strings_table)
            .context("read puni feeding info")?;
        info!("Read puni feeding data");

        let quests = quests::QuestData::read(pak_index, &strings_table).context("read quests")?;
        info!("Read quest data");

        Ok(Self {
            item_data,
            item_category_data,
            item_effect_data,
            recipe_data,
            field_map,
            field_data,
            enemy_data,
            puni_feeding_data,
            quests,
        })
    }
}
