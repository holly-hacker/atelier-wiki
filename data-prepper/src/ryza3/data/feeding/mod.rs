mod species;
mod unique_item_event;

use anyhow::Context;
use serde::Serialize;
use typescript_type_def::TypeDef;

use super::strings_table::StringsTable;
use crate::utils::PakIndex;

/// A puni species that can result from feeding the puni.
#[derive(Serialize, TypeDef)]
pub struct PuniFeedingSpecies {
    /// The name of this species
    pub name: String,
    pub character_tag: String,
    pub image_no: usize,
    /// The energy range for this species. This is localized as "Health" in the English translation of the game.
    pub energy: (u32, u32),
    /// The color range for this species. This is localized as "Luster" in the English translation of the game.
    pub color: (u32, u32),
    /// The mood range for this species.
    pub mood: (u32, u32),
    pub rank_e: Option<(u32, u32)>,
    pub rank_d: Option<(u32, u32)>,
    pub rank_c: Option<(u32, u32)>,
    pub rank_b: Option<(u32, u32)>,
    pub rank_a: Option<(u32, u32)>,
    pub rank_s: Option<(u32, u32)>,
    pub categories: Vec<String>,
}

/// A feeding event where a unique item is awarded.
#[derive(Serialize, TypeDef)]
pub struct PuniFeedingUniqueEvent {
    /// The item tag
    pub item_tag: String,
    /// The condition required to trigger this event.
    pub condition: PuniFeedingEventCondition,
}

#[derive(Serialize, TypeDef)]
#[serde(tag = "type")]
pub enum PuniFeedingEventCondition {
    /// A specific puni species.
    PuniSpecies { species: String },
    /// A range for the energy value.
    Energy { min: u32, max: u32 },
    /// A range for the color value.
    Color { min: u32, max: u32 },
    /// A range for the mood value.
    Mood { min: u32, max: u32 },
}

#[derive(Serialize, TypeDef)]
pub struct PuniFeedingData {
    pub unique_events: Vec<PuniFeedingUniqueEvent>,
    pub species: Vec<PuniFeedingSpecies>,
}

impl PuniFeedingData {
    pub fn read(pak_index: &mut PakIndex, strings: &StringsTable) -> anyhow::Result<Self> {
        let unique_events = unique_item_event::FeedingUniqueItemEvent::read(pak_index)
            .context("read unique events")?;
        let species = species::Species::read(pak_index).context("read species")?;

        let unique_events = unique_events
            .into_iter()
            .map(|e| PuniFeedingUniqueEvent {
                item_tag: e.item,
                condition: match e.cond.as_str() {
                    "FEEDING_COND_PUNI" => PuniFeedingEventCondition::PuniSpecies {
                        species: e.param[0].clone(),
                    },
                    "FEEDING_COND_ENERGY" => PuniFeedingEventCondition::Energy {
                        min: e.param[0].parse().unwrap(),
                        max: e.param[1].parse().unwrap(),
                    },
                    "FEEDING_COND_COLOR" => PuniFeedingEventCondition::Color {
                        min: e.param[0].parse().unwrap(),
                        max: e.param[1].parse().unwrap(),
                    },
                    "FEEDING_COND_MOOD" => PuniFeedingEventCondition::Mood {
                        min: e.param[0].parse().unwrap(),
                        max: e.param[1].parse().unwrap(),
                    },
                    _ => panic!("Unknown condition type: {}", e.cond),
                },
            })
            .collect();

        #[allow(clippy::unnecessary_lazy_evaluations)] // bug, see rust-lang/rust-clippy#9422
        let species = species
            .into_iter()
            .map(|s| PuniFeedingSpecies {
                name: strings.id_lookup.get(&s.name).unwrap().clone(),
                character_tag: s.chara_tag,
                image_no: s.image_no,
                energy: (s.ene_min, s.ene_max),
                color: (s.color_min, s.color_max),
                mood: (s.mood_min, s.mood_max),
                rank_e: (s.rank_e_min != -1).then(|| (s.rank_e_min as u32, s.rank_e_max as u32)),
                rank_d: (s.rank_d_min != -1).then(|| (s.rank_d_min as u32, s.rank_d_max as u32)),
                rank_c: (s.rank_c_min != -1).then(|| (s.rank_c_min as u32, s.rank_c_max as u32)),
                rank_b: (s.rank_b_min != -1).then(|| (s.rank_b_min as u32, s.rank_b_max as u32)),
                rank_a: (s.rank_a_min != -1).then(|| (s.rank_a_min as u32, s.rank_a_max as u32)),
                rank_s: (s.rank_s_min != -1).then(|| (s.rank_s_min as u32, s.rank_s_max as u32)),
                categories: s.category_tags,
            })
            .collect();

        Ok(Self {
            unique_events,
            species,
        })
    }
}
