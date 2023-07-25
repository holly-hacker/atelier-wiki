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
            let read = ElementReader(&element);

            let exp = read.read_parse("exp")?;
            let money = read.read_parse("money")?;
            let exp_rosca = read.read_parse("exp_rosca")?;
            let money_rosca = read.read_parse("money_rosca")?;
            let gold_coin = read.read_parse("gold_coin")?;
            let gold_coin_rate = read.read_parse("gold_coin_rate")?;
            let drop_tag = read.read_string("drop_tag")?;
            let skill_tag = read.read_string("skill_tag")?;
            let extra_skill_tag = read.read_string("extra_skill_tag")?;
            let lv = read.read_parse("lv")?;
            let stun = read.read_parse("stun")?;
            let key_make = read.read_parse("key_make")?;
            let atk_num = read.read_parse("atk_num")?;
            let burst_up = read.read_parse("burst_up")?;
            let burst_max = read.read_parse("burst_max")?;
            let hp = read.read_parse("hp")?;
            let atk = read.read_parse("atk")?;
            let def = read.read_parse("def")?;
            let spd = read.read_parse("spd")?;
            let bad_resist = read.read_parse_list("bad_resist_");
            let resist_non = read.read_parse("resist_non")?;
            let monster_tag = read.read_string("monster_tag")?;
            let key_create_tag = read.read_string("key_create_tag")?;
            let att = read.read_string_list("att_");

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
