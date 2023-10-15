use anyhow::Context;
use serde::Serialize;
use tracing::debug;
use typescript_type_def::TypeDef;

use crate::utils::PakIndex;

mod dollmake_change;

#[derive(Serialize, TypeDef)]
pub struct Doll {
    pub no: usize,
    pub name: String,
    pub doll_tag: String,
    pub chara_base_tag: String,
    pub dlc_tag: String,
    pub doll_event_tag: String,

    pub cute_min: i32,
    pub cute_max: i32,
    pub wise_min: i32,
    pub wise_max: i32,
    pub brave_min: i32,
    pub brave_max: i32,
    pub fool_min: i32,
    pub fool_max: i32,

    pub doll_hp: usize,
    pub doll_mp: usize,
    pub doll_lp: usize,
    pub doll_atk: usize,
    pub doll_def: usize,
    pub doll_spd: usize,
    pub doll_dmg_min: usize,
    pub doll_dmg_max: usize,
}

impl Doll {
    pub fn read(pak_index: &mut PakIndex) -> anyhow::Result<Vec<Self>> {
        debug!("Reading dollmake_change data");
        let changes =
            dollmake_change::DollmakeChange::read(pak_index).context("read dollmake_change")?;

        let dolls = changes
            .into_iter()
            .map(|change| Doll {
                no: change.no,
                name: change.name,
                doll_tag: change.doll_tag,
                chara_base_tag: change.chara_base_tag,
                dlc_tag: change.dlc_tag,
                doll_event_tag: change.doll_event_tag,
                cute_min: change.cute_min,
                cute_max: change.cute_max,
                wise_min: change.wise_min,
                wise_max: change.wise_max,
                brave_min: change.brave_min,
                brave_max: change.brave_max,
                fool_min: change.fool_min,
                fool_max: change.fool_max,
                doll_hp: change.doll_mhp,
                doll_mp: change.doll_mmp,
                doll_lp: change.doll_mlp,
                doll_atk: change.doll_atk,
                doll_def: change.doll_def,
                doll_spd: change.doll_spd,
                doll_dmg_min: change.doll_dmg_min,
                doll_dmg_max: change.doll_dmg_max,
            })
            .collect();

        Ok(dolls)
    }
}
