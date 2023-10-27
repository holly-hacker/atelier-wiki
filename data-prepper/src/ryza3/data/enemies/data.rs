use crate::utils::{self, ElementReader, PakIndex};

#[derive(Debug)]
pub struct EnemyData {
    pub name_id: String,
    pub is_big: bool,
    pub img_no: i32,
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
        utils::read_xml(pak_index, r"\saves\enemy\enemy_data.xml", |d| {
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
            let reader = ElementReader(&element);

            let name_id = reader.read("name_id")?;
            let is_big = reader.is_present("isBig");
            let img_no = reader.read("imgNo")?;
            let wait_action = reader.is_present("waitAction");
            let library_rank = reader.read_list("library_rank_*")?;
            let dlc = reader.read_list("dlc_*")?;
            let shoot_up = reader.is_present("shoot_up");
            let monster_tag = reader.read("monster_tag")?;
            let chara_tag = reader.read("chara_tag")?;
            let race_tag = reader.read("race_tag")?;
            let size = reader.read("size")?;
            let division = reader.read("division")?;

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
