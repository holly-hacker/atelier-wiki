//! Field data, such as props that can be found in the game world and items drop data for resource nodes.

mod field_data_types;
mod gimmick;

use std::collections::BTreeMap;
use std::num::ParseFloatError;

use anyhow::Context;
use serde::Serialize;
use tracing::trace;
use typescript_type_def::TypeDef;

use crate::extract::ryza3::data::field_data::gimmick::GimmickProperty;
use crate::utils::PakIndex;
pub use field_data_types::*;

#[derive(Serialize, TypeDef)]
pub struct FieldData(pub BTreeMap<String, FieldDataSet>);

#[derive(Serialize, TypeDef, Default)]
pub struct FieldDataSet {
    /// Trees that can be cut down for resources.
    pub cut_down_tree: Vec<CutDownTree>,

    /// Random spawn points for enemies.
    pub enemy_random_spawner: Vec<EnemyRandomSpawner>,

    /// Instant spawn points for enemies. Presumably used for boss monsters.
    pub instant_enemy_spawner: Vec<InstantEnemySpawner>,
}

#[derive(Serialize, TypeDef)]
pub struct GimmickData {
    /// The position of this gimmick.
    pub position: [f32; 3],
    /// The chance of this gimmick being placed, between 0 and 100 inclusive.
    pub rate: usize,
    /// Whether the state of this gimmick is saved.
    pub save: bool,
}

impl FieldData {
    pub fn read(pak_index: &mut PakIndex) -> anyhow::Result<Self> {
        let gimmicks = GimmickProperty::read(pak_index)?;

        let mut ret = Self(Default::default());
        for (map, gimmicks) in gimmicks {
            let span = tracing::span!(tracing::Level::DEBUG, "fieldmap", map = map.as_str());
            let _entered = span.enter();

            let mut set = FieldDataSet::default();

            for (gimmick_idx, gimmick) in gimmicks.into_iter().enumerate() {
                set.read_gimmick(gimmick)
                    .with_context(|| format!("read gimmick {gimmick_idx} for map {map}"))?;
            }

            ret.0.insert(map, set);
        }

        Ok(ret)
    }
}

impl FieldDataSet {
    fn read_gimmick(&mut self, gimmick: GimmickProperty) -> anyhow::Result<()> {
        let type_tag = gimmick.gimmick_type_tag.as_str();
        assert!(
            type_tag.starts_with("FIELDMAP_GIMMICK_TYPE_"),
            "fieldmap gimmick type tag should start with FIELDMAP_GIMMICK_TYPE_"
        );
        let tag_trimmed = &type_tag[22..];
        match tag_trimmed {
            "ACTIVATE_GUIDE_ANIMAL" => trace!("unimplemented: {type_tag}"),
            "CUT_DOWN_TREE" => {
                self.cut_down_tree
                    .push(CutDownTree::from_gimmick(gimmick).context("read CutDownTree")?);
            }
            "DIG_HOLE" => trace!("unimplemented: {type_tag}"),
            "ENEMY_RANDOM_SPAWNER_NEST" => trace!("unimplemented: {type_tag}"),
            "ENEMY_RANDOM_SPAWNER" => {
                self.enemy_random_spawner.push(
                    EnemyRandomSpawner::from_gimmick(gimmick).context("read EnemyRandomSpawner")?,
                );
            }
            "FAST_TRAVEL" => trace!("unimplemented: {type_tag}"),
            "FISHING" => trace!("unimplemented: {type_tag}"),
            "HIDDEN_TREASURE_BOX" => trace!("unimplemented: {type_tag}"),
            "INSTANT_ANIMAL_SPAWNER" => trace!("unimplemented: {type_tag}"),
            "INSTANT_ENEMY_SPAWNER" => self.instant_enemy_spawner.push(
                InstantEnemySpawner::from_gimmick(gimmick).context("read InstantEnemySpawner")?,
            ),
            "INSTANT_FISH_SCHOOL_SPAWNER_COLLECT" => trace!("unimplemented: {type_tag}"),
            "INSTANT_FISH_SCHOOL_SPAWNER" => trace!("unimplemented: {type_tag}"),
            "ITEM_BOX_WITH_OBJ_ON_THE_SEA" => trace!("unimplemented: {type_tag}"),
            "ITEM_BOX_WITH_OBJ" => trace!("unimplemented: {type_tag}"),
            "ITEM_D_MULTIPLE_INDIRECT" => trace!("unimplemented: {type_tag}"),
            "ITEM_D_MULTIPLE_RANGE" => trace!("unimplemented: {type_tag}"),
            "ITEM_D_MULTIPLE_WAND_ACTION" => trace!("unimplemented: {type_tag}"),
            "ITEM_D_MULTIPLE" => trace!("unimplemented: {type_tag}"),
            _ => (),
        }

        Ok(())
    }
}

impl GimmickData {
    pub fn from_gimmick(gimmick: &gimmick::GimmickProperty) -> anyhow::Result<Self> {
        Ok(Self {
            position: gimmick
                .position
                .split(',')
                .map(|x| x.parse())
                .collect::<Result<Vec<_>, ParseFloatError>>()
                .context("parse gimmick position")?
                .try_into()
                .map_err(|e| anyhow::anyhow!("parse gimmick position: {e:?}"))?,
            rate: gimmick.rate,
            save: gimmick.save_flag,
        })
    }
}
