use crate::extract::{
    data::util::{self, ElementReader},
    pak_index::PakIndex,
};

#[derive(Debug)]
pub struct EnemyData {
    pub name_id: String,
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
}

impl EnemyData {
    pub fn read(pak_index: &mut PakIndex) -> anyhow::Result<Vec<Self>> {
        util::read_xml(pak_index, r"\saves\enemy\enemy_data.xml", |d| {
            Self::read_from_doc(d)
        })
    }

    fn read_from_doc(document: roxmltree::Document) -> anyhow::Result<Vec<Self>> {
        let mut ret = vec![];

        let elements = document
            .root_element()
            .descendants()
            .filter(|n| n.tag_name().name() == "enemy_data");

        for element in elements {
            let read = ElementReader(&element);

            let name_id = read.read_string("name_id")?;
            let is_big = read.read_present("isBig");
            let img_no = read.read_string("imgNo")?;
            let wait_action = read.read_present("waitAction");
            let library_rank = read.read_parse_list("library_rank_");
            let dlc = read.read_parse_list("dlc_");
            let shoot_up = read.read_present("shoot_up");
            let monster_tag = read.read_string("monster_tag")?;
            let chara_tag = read.read_string("chara_tag")?;
            let race_tag = read.read_string("race_tag")?;
            let size = read.read_string("size")?;
            let division = read.read_string("division")?;

            debug_assert!(dlc.len() <= 1);
            debug_assert_eq!(library_rank.len(), 4);

            let enemy_data = Self {
                name_id,
                is_big,
                img_no,
                wait_action,
                library_rank,
                dlc,
                shoot_up,
                monster_tag,
                chara_tag,
                race_tag,
                size,
                division,
            };

            ret.push(enemy_data);
        }

        Ok(ret)
    }
}
