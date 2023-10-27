mod item_effect;

use std::collections::BTreeMap;

use anyhow::Context;
use serde::Serialize;
use tracing::debug;
use typescript_type_def::TypeDef;

use super::strings_table::StringsTable;
use crate::ryza3::executable::Ryza3ExecutableData;
use crate::utils::PakIndex;

#[derive(Serialize, TypeDef)]
pub struct ItemEffectData {
    // TODO: add typescript support for indexmap in the `typescript-type-def` crate
    // the natural ordering in the game makes sense here so it would be nice to preserve that.
    // the automatic ordering of btreemap also doesn't work well with natural ordering of numbers,
    // eg. `_2` is followed by `_20` rather than `_3`.
    // This is fairly low prio, it's just to satiate my OCD.
    pub item_effects: BTreeMap<String, ItemEffect>,
}

#[derive(Serialize, TypeDef)]
pub struct ItemEffect {
    /// The name of the effect as it is shown in-game
    pub name: String,
    pub description: Option<String>,
    pub kind: Option<String>,
    pub base_attribute: Option<String>,

    pub attributes: Vec<EffectAttribute>,
}

#[derive(Serialize, TypeDef)]
pub struct EffectAttribute {
    pub action: String,
    pub attribute: Option<String>,
    pub min_1: Option<String>,
    pub max_1: Option<String>,
    pub min_2: Option<String>,
    pub max_2: Option<String>,
}

impl ItemEffectData {
    pub fn read(
        pak_index: &mut PakIndex,
        executable_data: &Ryza3ExecutableData,
        strings: &StringsTable,
    ) -> anyhow::Result<Self> {
        debug!("Reading item effects");
        let item_effects = item_effect::ItemEffect::read(pak_index).context("read item effects")?;

        // shortcut to avoid some extra code. if this fails, just extract the "index" from the name_id field
        assert_eq!(
            item_effects.len(),
            executable_data.item_effects.len(),
            "assuming each item effect is listed in xml"
        );

        debug!("merging item effects");
        let effects = executable_data
            .item_effects
            .iter()
            .enumerate()
            .filter_map(|(i, tag)| {
                // get the name
                let string_id = format!("STR_ITEM_EFFECT_{i:04}");

                // ignore entries that don't have a name, they are most likely unused
                // some entries also have trailing whitespace, so trim that
                let name = strings.id_lookup.get(&string_id)?.trim();
                if name.is_empty() {
                    return None;
                }

                // get the library description
                let description_string_id = format!("STR_LIBRARY_EFF_DETAIL_{i:04}");
                let description = strings.id_lookup.get(&description_string_id).cloned();

                let item_effect = &item_effects[i];
                if let Some(name_id) = &item_effect.name_id {
                    assert_eq!(
                        &string_id, name_id,
                        "order of xml effect data and exe effect data should be the same"
                    );
                }

                let max_attr_num = item_effect
                    .act_tag
                    .len()
                    .max(item_effect.att_tag.len())
                    .max(item_effect.min_1.len())
                    .max(item_effect.max_1.len())
                    .max(item_effect.min_2.len())
                    .max(item_effect.max_2.len());

                let attributes = (0..max_attr_num)
                    .flat_map(|i| {
                        let attribute = item_effect
                            .att_tag
                            .get(i)
                            .cloned()
                            .flatten()
                            .filter(|a| a != "ATT_NONE");

                        let action = item_effect
                            .act_tag
                            .get(i)
                            .cloned()
                            .flatten()
                            .filter(|a| a != "ACT_NONE");

                        let min_1 = item_effect
                            .min_1
                            .get(i)
                            .cloned()
                            .flatten()
                            .filter(|a| a != "ATT_NONE");
                        let max_1 = item_effect
                            .max_1
                            .get(i)
                            .cloned()
                            .flatten()
                            .filter(|a| a != "ATT_NONE");
                        let min_2 = item_effect
                            .min_2
                            .get(i)
                            .cloned()
                            .flatten()
                            .filter(|a| a != "ATT_NONE");
                        let max_2 = item_effect
                            .max_2
                            .get(i)
                            .cloned()
                            .flatten()
                            .filter(|a| a != "ATT_NONE");

                        // Filter out attributes where the action is `ACT_NONE`. This is mostly padding.
                        let Some(action) = action else {
                            return None;
                        };

                        Some(EffectAttribute {
                            attribute,
                            action,
                            min_1,
                            max_1,
                            min_2,
                            max_2,
                        })
                    })
                    .collect::<Vec<_>>();

                // There are many effects with no attributes. While most of them are unused, some of
                // them are (such as `ITEM_EFF_ADD_CATEGORY_39`/"Lost Property") which measn we
                // can't detect the unused ones. For now, we don't do anything.
                // The amount of effects we filter out by removing those without effects is fairly
                // low anyway.

                let item_effect = ItemEffect {
                    name: name.to_string(),
                    description,
                    kind: item_effect.kind.clone(),
                    base_attribute: Some(&item_effect.base_att_tag)
                        .filter(|t| *t != "ATT_NONE")
                        .cloned(),
                    attributes,
                };

                Some((tag.clone(), item_effect))
            })
            .collect();

        Ok(Self {
            item_effects: effects,
        })
    }
}
