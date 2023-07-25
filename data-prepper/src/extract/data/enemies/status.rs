use anyhow::Context;

use crate::extract::{data::util, pak_index::PakIndex};

#[derive(Debug, Clone)]
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
    pub monster_tag: String,
    pub key_create_tag: String,
    pub att: Vec<String>,
}

impl EnemyStatus {
    pub fn read(pak_index: &mut PakIndex) -> anyhow::Result<Vec<Self>> {
        util::read_xml(pak_index, r"\saves\enemy\enemy_status.xml", |d| {
            Self::read_from_doc(d)
        })
    }

    fn read_from_doc(document: roxmltree::Document) -> anyhow::Result<Vec<Self>> {
        let mut ret = vec![];

        let elements = document
            .root_element()
            .descendants()
            .filter(|n| n.tag_name().name() == "enemy_status");

        for element in elements {
            let exp = element
                .attribute("exp")
                .context("field 'exp' is required on enemy status")?
                .parse()
                .context("parse 'exp'")?;
            let money = element
                .attribute("money")
                .context("field 'money' is required on enemy status")?
                .parse()
                .context("parse 'money'")?;
            let exp_rosca = element
                .attribute("exp_rosca")
                .context("field 'exp_rosca' is required on enemy status")?
                .parse()
                .context("parse 'exp_rosca'")?;
            let money_rosca = element
                .attribute("money_rosca")
                .context("field 'money_rosca' is required on enemy status")?
                .parse()
                .context("parse 'money_rosca'")?;
            let gold_coin = element
                .attribute("gold_coin")
                .context("field 'gold_coin' is required on enemy status")?
                .parse()
                .context("parse 'gold_coin'")?;
            let gold_coin_rate = element
                .attribute("gold_coin_rate")
                .context("field 'gold_coin_rate' is required on enemy status")?
                .parse()
                .context("parse 'gold_coin_rate'")?;
            let drop_tag = element
                .attribute("drop_tag")
                .context("field 'drop_tag' is required on enemy status")?
                .to_string();
            let skill_tag = element
                .attribute("skill_tag")
                .context("field 'skill_tag' is required on enemy status")?
                .to_string();
            let extra_skill_tag = element
                .attribute("extra_skill_tag")
                .context("field 'extra_skill_tag' is required on enemy status")?
                .to_string();
            let lv = element
                .attribute("lv")
                .context("field 'lv' is required on enemy status")?
                .parse()
                .context("parse 'lv'")?;
            let stun = element
                .attribute("stun")
                .context("field 'stun' is required on enemy status")?
                .parse()
                .context("parse 'stun'")?;
            let key_make = element
                .attribute("key_make")
                .context("field 'key_make' is required on enemy status")?
                .parse()
                .context("parse 'key_make'")?;
            let atk_num = element
                .attribute("atk_num")
                .context("field 'atk_num' is required on enemy status")?
                .parse()
                .context("parse 'atk_num'")?;
            let burst_up = element
                .attribute("burst_up")
                .context("field 'burst_up' is required on enemy status")?
                .parse()
                .context("parse 'burst_up'")?;
            let burst_max = element
                .attribute("burst_max")
                .context("field 'burst_max' is required on enemy status")?
                .parse()
                .context("parse 'burst_max'")?;
            let hp = element
                .attribute("hp")
                .context("field 'hp' is required on enemy status")?
                .parse()
                .context("parse 'hp'")?;
            let atk = element
                .attribute("atk")
                .context("field 'atk' is required on enemy status")?
                .parse()
                .context("parse 'atk'")?;
            let def = element
                .attribute("def")
                .context("field 'def' is required on enemy status")?
                .parse()
                .context("parse 'def'")?;
            let spd = element
                .attribute("spd")
                .context("field 'spd' is required on enemy status")?
                .parse()
                .context("parse 'spd'")?;
            let bad_resist = element
                .attributes()
                .filter(|a| a.name().starts_with("bad_resist_"))
                .flat_map(|a| a.value().parse().context("parse 'bad_resist_*'"))
                .collect::<Vec<_>>();
            let resist_non = element
                .attribute("resist_non")
                .context("field 'resist_non' is required on enemy status")?
                .parse()
                .context("parse 'resist_non'")?;
            let monster_tag = element
                .attribute("monster_tag")
                .context("field 'monster_tag' is required on enemy status")?
                .to_string();
            let key_create_tag = element
                .attribute("key_create_tag")
                .context("field 'key_create_tag' is required on enemy status")?
                .to_string();
            let att = element
                .attributes()
                .filter(|a| a.name().starts_with("att_"))
                .map(|a| a.value().to_string())
                .collect::<Vec<_>>();

            debug_assert_eq!(bad_resist.len(), 10);
            debug_assert_eq!(att.len(), 8);

            ret.push(Self {
                exp,
                money,
                exp_rosca,
                money_rosca,
                gold_coin,
                gold_coin_rate,
                drop_tag,
                skill_tag,
                extra_skill_tag,
                lv,
                stun,
                key_make,
                atk_num,
                burst_up,
                burst_max,
                hp,
                atk,
                def,
                spd,
                bad_resist,
                resist_non,
                monster_tag,
                key_create_tag,
                att,
            })
        }

        Ok(ret)
    }
}
