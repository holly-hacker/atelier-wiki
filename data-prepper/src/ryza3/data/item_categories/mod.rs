use std::collections::BTreeMap;

use anyhow::Context;
use serde::Serialize;
use typescript_type_def::TypeDef;

use crate::ryza3::executable::Ryza3ExecutableData;

use super::strings_table::StringsTable;

#[derive(Serialize, TypeDef)]
pub struct ItemCategoryData {
    pub categories: BTreeMap<String, String>,
}

impl ItemCategoryData {
    pub fn read(
        executable_data: &Ryza3ExecutableData,
        strings: &StringsTable,
    ) -> anyhow::Result<Self> {
        let categories = executable_data
            .item_categories
            .iter()
            .enumerate()
            .map(|(i, category)| {
                let name = strings
                    .id_lookup
                    .get(&format!("STR_ITEM_CATEGORY_{i:03}"))
                    .cloned()
                    .with_context(|| format!("cannot find string for category {category}"))?;

                Ok((category.clone(), name))
            })
            .collect::<anyhow::Result<_>>()
            .context("read categories from string table")?;

        Ok(Self { categories })
    }
}
