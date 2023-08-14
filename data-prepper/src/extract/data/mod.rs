use anyhow::Context;
use serde::Serialize;
use tracing::info;
use typescript_type_def::TypeDef;

use crate::utils::PakIndex;

mod enemies;
mod items;
mod recipes;
mod strings;
mod util;

#[derive(Serialize, TypeDef)]
pub struct Ryza3Data {
    pub item_data: Vec<items::Item>,
    pub recipe_data: recipes::RecipeData,
    pub enemy_data: Vec<enemies::Enemy>,
}

impl Ryza3Data {
    pub fn read_all(pak_index: &mut PakIndex) -> anyhow::Result<Self> {
        // TODO: consider reading other languages too
        let strings = strings::StringsData::read(pak_index).context("read strings")?;

        info!(
            "Read {} strings by id and {} strings by number",
            strings.id_lookup.len(),
            strings.no_lookup.len()
        );

        // NOTE: itemdata_no appears to be the exact same file
        let item_data = items::Item::read(pak_index, &strings).context("read items")?;
        info!("Read data for {} items", item_data.len());

        let recipe_data = recipes::RecipeData::read(pak_index, &strings).context("read recipes")?;
        info!("Read data for {} recipes", recipe_data.recipes.len());

        let enemy_data = enemies::read(pak_index, &strings).context("read enemies")?;
        info!("Read data for {} enemies", enemy_data.len());

        Ok(Self {
            item_data,
            recipe_data,
            enemy_data,
        })
    }
}
