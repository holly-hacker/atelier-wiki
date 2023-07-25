use serde::Serialize;
use tracing::debug;
use typescript_type_def::TypeDef;

use crate::extract::pak_index::PakIndex;

use super::strings::StringsData;

mod data;
mod status;

#[derive(Serialize, TypeDef)]
pub struct Enemy {
    pub name: String,
    pub is_big: bool,
    pub img_no: String,
    pub wait_action: bool,
    pub library_rank: Vec<u32>,
    pub dlc: Vec<String>,
    pub shoot_up: bool,
    pub monster_tag: String,
    pub chara_tag: String,
    pub race_tag: String,
    pub size: String,
    pub division: String,
    pub statusses: Vec<EnemyStatus>,
}

#[derive(Serialize, TypeDef)]
pub struct EnemyStatus {
    pub exp: u32,
    pub money: u32,
    pub exp_rosca: u32,
    pub money_rosca: u32,
    pub gold_coin: u32,
    pub gold_coin_rate: u32,
    pub drop_tag: String,
    pub skill_tag: String,
    pub extra_skill_tag: String,
    pub lv: u32,
    pub stun: u32,
    pub key_make: u32,
    pub atk_num: u32,
    pub burst_up: u32,
    pub burst_max: u32,
    pub hp: i32,
    pub atk: i32,
    pub def: i32,
    pub spd: i32,
    pub bad_resist: Vec<u32>,
    pub resist_non: u32,
    pub key_create_tag: String,
    pub att: Vec<String>,
}

pub fn read(pak_index: &mut PakIndex, strings: &StringsData) -> anyhow::Result<Vec<Enemy>> {
    debug!("Reading enemy data");
    let data = data::EnemyData::read(pak_index)?;

    debug!("Reading enemy status");
    let status = status::EnemyStatus::read(pak_index)?;

    debug!("Merging enemy info");
    let ret = data
        .into_iter()
        .map(|d| Enemy {
            name: strings.id_lookup[&d.name_id].clone(),
            is_big: d.is_big,
            img_no: d.img_no,
            wait_action: d.wait_action,
            library_rank: d.library_rank,
            dlc: d.dlc,
            shoot_up: d.shoot_up,
            chara_tag: d.chara_tag,
            race_tag: d.race_tag,
            size: d.size,
            division: d.division,

            // Ideally, we'd use `status.drain_filter` but this is not stable yet
            // this would allow us to avoid copying and allows us to later check if we got everything
            // see: rust-lang/rust#43244
            statusses: status
                .iter()
                .filter(|s| s.monster_tag == d.monster_tag)
                .map(|s| EnemyStatus {
                    exp: s.exp,
                    money: s.money,
                    exp_rosca: s.exp_rosca,
                    money_rosca: s.money_rosca,
                    gold_coin: s.gold_coin,
                    gold_coin_rate: s.gold_coin_rate,
                    drop_tag: s.drop_tag.clone(),
                    skill_tag: s.skill_tag.clone(),
                    extra_skill_tag: s.extra_skill_tag.clone(),
                    lv: s.lv,
                    stun: s.stun,
                    key_make: s.key_make,
                    atk_num: s.atk_num,
                    burst_up: s.burst_up,
                    burst_max: s.burst_max,
                    hp: s.hp,
                    atk: s.atk,
                    def: s.def,
                    spd: s.spd,
                    bad_resist: s.bad_resist.clone(),
                    resist_non: s.resist_non,
                    key_create_tag: s.key_create_tag.clone(),
                    att: s.att.clone(),
                })
                .collect(),
            monster_tag: d.monster_tag,
        })
        .collect();

    Ok(ret)
}
