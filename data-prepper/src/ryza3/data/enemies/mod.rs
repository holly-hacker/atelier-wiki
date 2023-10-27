//! Files of interest:
//! - [x] enemy\enemy_data.xml: General info of an enemy.
//! - [x] enemy\enemy_status.xml: An "instance" of a monster, which has a specific level.
//! - [x] enemy\drop_data.xml: Dropped items by an enemy status.
//! - [x] enemy\librarymonster.xml: Enemy info in in-game library, contains description.
//! - [ ] encount\encount_data.xml: Info on enemy encounter groups?
//! - [ ] fieldmap\charaenemy\charaenemy_sg_data.xml: Info about enemy encounters for the world map?

use anyhow::Context;
use serde::Serialize;
use tracing::debug;
use typescript_type_def::TypeDef;

use super::strings_table::StringsTable;
use crate::utils::PakIndex;

mod data;
mod drop;
mod library_monster;
mod status;

#[derive(Serialize, TypeDef)]
pub struct Enemy {
    pub name: String,
    pub library_note: Option<String>,
    pub is_big: bool,
    pub img_no: i32,
    pub wait_action: bool,
    pub library_rank_health: u32,
    pub library_rank_attack: u32,
    pub library_rank_speed: u32,
    pub library_rank_defense: u32,
    pub dlc: Option<String>,
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

    /// `sp_item_tag` from drop data
    pub sp_item_tag: String,
    pub drops: Vec<EnemyDrop>,
}

#[derive(Serialize, TypeDef)]
pub struct EnemyDrop {
    pub item_tag: String,
    pub rate: u32,
    pub num: u32,
    pub quality_min: f32,
    pub quality_max: f32,
    pub potential_min: f32,
    pub potential_max: f32,
    pub potential_num_min: f32,
    pub potential_num_max: f32,
    pub potential_lv_min: f32,
    pub potential_lv_max: f32,
    pub quality_min_adj: f32,
    pub quality_max_adj: f32,
    pub potential_min_adj: f32,
    pub potential_max_adj: f32,
    pub potential_num_min_adj: u32,
    pub potential_num_max_adj: u32,
    pub potential_lv_min_adj: u32,
    pub potential_lv_max_adj: u32,
    pub super_pot_rate: u32,
    pub factor: String,
    pub eff: Option<String>,
}

pub fn read(pak_index: &mut PakIndex, strings: &StringsTable) -> anyhow::Result<Vec<Enemy>> {
    debug!("Reading enemy data");
    let data = data::EnemyData::read(pak_index).context("read enemy_data")?;

    debug!("Reading enemy status");
    let status = status::EnemyStatus::read(pak_index).context("read enemy_status")?;

    debug!("Reading enemy drops");
    let drops = drop::DropData::read(pak_index).context("read drop data")?;

    debug!("Reading library monsters");
    let library_monsters =
        library_monster::LibraryMonster::read(pak_index).context("read library monsters")?;

    debug!("Merging enemy info");
    let ret = data
        .into_iter()
        .map(|d| {
            Enemy {
                name: strings.id_lookup[&d.name_id].clone(),
                library_note: library_monsters
                    .iter()
                    .find(|l| l.monster_tag == d.monster_tag)
                    .map(|l| {
                        l.note_id
                            .iter()
                            .filter_map(|id| strings.id_lookup.get(id).cloned())
                            .collect::<Vec<_>>()
                            .join("\n")
                    }),
                is_big: d.is_big,
                img_no: d.img_no,
                wait_action: d.wait_action,
                library_rank_health: d.library_rank[0],
                library_rank_attack: d.library_rank[1],
                library_rank_speed: d.library_rank[2],
                library_rank_defense: d.library_rank[3],
                dlc: d.dlc.get(0).cloned(),
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
                    .enumerate()
                    .filter(|(_, s)| s.monster_tag == d.monster_tag)
                    .map(|(i, s)| EnemyStatus {
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

                        sp_item_tag: drops[i].sp_item_tag[0].clone(),
                        drops: (0..drops[i].num.len())
                            .map(|j| EnemyDrop {
                                item_tag: drops[i].item_tag[j].clone(),
                                rate: drops[i].rate[j],
                                num: drops[i].num[j],
                                quality_min: drops[i].quality_min[j],
                                quality_max: drops[i].quality_max[j],
                                potential_min: drops[i].potential_min[j],
                                potential_max: drops[i].potential_max[j],
                                potential_num_min: drops[i].potential_num_min[j],
                                potential_num_max: drops[i].potential_num_max[j],
                                potential_lv_min: drops[i].potential_lv_min[j],
                                potential_lv_max: drops[i].potential_lv_max[j],
                                quality_min_adj: drops[i].quality_min_adj[j],
                                quality_max_adj: drops[i].quality_max_adj[j],
                                potential_min_adj: drops[i].potential_min_adj[j],
                                potential_max_adj: drops[i].potential_max_adj[j],
                                potential_num_min_adj: drops[i].potential_num_min_adj[j],
                                potential_num_max_adj: drops[i].potential_num_max_adj[j],
                                potential_lv_min_adj: drops[i].potential_lv_min_adj[j],
                                potential_lv_max_adj: drops[i].potential_lv_max_adj[j],
                                super_pot_rate: drops[i].super_pot_rate[j],
                                factor: drops[i].factor[j].clone(),
                                eff: drops[i].eff.get(j).cloned().flatten(),
                            })
                            .collect(),
                    })
                    .collect(),
                monster_tag: d.monster_tag,
            }
        })
        .collect();

    Ok(ret)
}
