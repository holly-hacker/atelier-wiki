use crate::extract::{
    data::util::{self, ElementReader},
    pak_index::PakIndex,
};

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
            let reader = ElementReader(&element);

            let exp = reader.read("exp")?;
            let money = reader.read("money")?;
            let exp_rosca = reader.read("exp_rosca")?;
            let money_rosca = reader.read("money_rosca")?;
            let gold_coin = reader.read("gold_coin")?;
            let gold_coin_rate = reader.read("gold_coin_rate")?;
            let drop_tag = reader.read("drop_tag")?;
            let skill_tag = reader.read("skill_tag")?;
            let extra_skill_tag = reader.read("extra_skill_tag")?;
            let lv = reader.read("lv")?;
            let stun = reader.read("stun")?;
            let key_make = reader.read("key_make")?;
            let atk_num = reader.read("atk_num")?;
            let burst_up = reader.read("burst_up")?;
            let burst_max = reader.read("burst_max")?;
            let hp = reader.read("hp")?;
            let atk = reader.read("atk")?;
            let def = reader.read("def")?;
            let spd = reader.read("spd")?;
            let bad_resist = reader.read_list("bad_resist_*")?;
            let resist_non = reader.read("resist_non")?;
            let monster_tag = reader.read("monster_tag")?;
            let key_create_tag = reader.read("key_create_tag")?;
            let att = reader.read_list("att_*")?;

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
