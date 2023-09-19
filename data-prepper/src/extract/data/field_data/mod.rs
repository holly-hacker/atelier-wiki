//! Field data, such as props that can be found in the game world and items drop data for resource nodes.

mod gimmick;

use std::collections::BTreeMap;
use std::num::ParseFloatError;

use anyhow::Context;
use serde::Serialize;
use tracing::trace;
use typescript_type_def::TypeDef;

use crate::utils::PakIndex;

#[derive(Serialize, TypeDef)]
pub struct FieldData(pub BTreeMap<String, FieldDataSet>);

#[derive(Serialize, TypeDef, Default)]
pub struct FieldDataSet {
    /// Trees that can be cut down for resources.
    pub cut_down_tree: Vec<CutDownTree>,
}

#[derive(Serialize, TypeDef)]
pub struct CutDownTree {
    #[serde(flatten)]
    pub data: GimmickData,
    /// Drop information when using rod.
    pub rod: (Option<String>, Option<usize>),
    /// Drop information when using sickle.
    pub sickle: (Option<String>, Option<usize>),
    /// Drop information when using axe.
    pub axe: (Option<String>, Option<usize>),
    /// Drop information when using hammer.
    pub hammer: (Option<String>, Option<usize>),
}

#[derive(Serialize, TypeDef)]
pub struct GimmickData {
    /// The position of this gimmick.
    pub position: [f32; 3],
    /// The rotation of this gimmick.
    pub rate: usize,
    /// Whether the state of this gimmick is saved.
    pub save: bool,
}

impl FieldData {
    pub fn read(pak_index: &mut PakIndex) -> anyhow::Result<Self> {
        let gimmicks = gimmick::GimmickProperty::read(pak_index)?;

        let mut ret = Self(Default::default());
        for (map, gimmicks) in gimmicks {
            let span = tracing::span!(tracing::Level::DEBUG, "fieldmap", map = map.as_str());
            let _entered = span.enter();

            let mut set = FieldDataSet::default();

            for gimmick in gimmicks {
                let type_tag = gimmick.gimmick_type_tag.as_str();
                assert!(
                    type_tag.starts_with("FIELDMAP_GIMMICK_TYPE_"),
                    "fieldmap gimmick type tag should start with FIELDMAP_GIMMICK_TYPE_"
                );
                let tag_trimmed = &type_tag[22..];
                match tag_trimmed {
                    "ACTIVATE_GUIDE_ANIMAL" => trace!("unimplemented: {type_tag}"),
                    "CUT_DOWN_TREE" => {
                        set.cut_down_tree
                            .push(CutDownTree::from_gimmick(gimmick).context("read CutDownTree")?);
                    }
                    "DIG_HOLE" => trace!("unimplemented: {type_tag}"),
                    "ENEMY_RANDOM_SPAWNER_NEST" => trace!("unimplemented: {type_tag}"),
                    "ENEMY_RANDOM_SPAWNER" => trace!("unimplemented: {type_tag}"),
                    "FAST_TRAVEL" => trace!("unimplemented: {type_tag}"),
                    "FISHING" => trace!("unimplemented: {type_tag}"),
                    "HIDDEN_TREASURE_BOX" => trace!("unimplemented: {type_tag}"),
                    "INSTANT_ANIMAL_SPAWNER" => trace!("unimplemented: {type_tag}"),
                    "INSTANT_ENEMY_SPAWNER" => trace!("unimplemented: {type_tag}"),
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
            }

            ret.0.insert(map, set);
        }

        Ok(ret)
    }
}

impl CutDownTree {
    pub fn from_gimmick(gimmick: gimmick::GimmickProperty) -> anyhow::Result<Self> {
        if gimmick.parameters.len() != 8 {
            anyhow::bail!("CutDownTree gimmick should have 8 parameters");
        }

        Ok(Self {
            data: GimmickData::from_gimmick(&gimmick)?,
            rod: (
                gimmick.parameters[0].clone(),
                gimmick.parameters[4]
                    .as_ref()
                    .map(|p| p.parse())
                    .transpose()
                    .context("parse arg 4")?,
            ),
            sickle: (
                gimmick.parameters[1].clone(),
                gimmick.parameters[5]
                    .as_ref()
                    .map(|p| p.parse())
                    .transpose()
                    .context("parse arg 5")?,
            ),
            axe: (
                gimmick.parameters[2].clone(),
                gimmick.parameters[6]
                    .as_ref()
                    .map(|p| p.parse())
                    .transpose()
                    .context("parse arg 6")?,
            ),
            hammer: (
                gimmick.parameters[3].clone(),
                gimmick.parameters[7]
                    .as_ref()
                    .map(|p| p.parse())
                    .transpose()
                    .context("parse arg 7")?,
            ),
        })
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
