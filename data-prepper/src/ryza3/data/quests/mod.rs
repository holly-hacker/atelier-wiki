use anyhow::Context;
use serde::Serialize;
use typescript_type_def::TypeDef;

use crate::utils::PakIndex;

use super::strings_table::StringsTable;

mod normal;

#[derive(Serialize, TypeDef)]
pub struct QuestData {
    pub normal_quests: Vec<normal::NormalQuest>,
}

impl QuestData {
    pub fn read(pak_index: &mut PakIndex, strings: &StringsTable) -> anyhow::Result<Self> {
        let normal_quests =
            normal::NormalQuest::read(pak_index, strings).context("read normal quests")?;

        Ok(Self { normal_quests })
    }
}
