use anyhow::Context;

use crate::extract::{data::util, pak_index::PakIndex};

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
            let name_id = element
                .attribute("name_id")
                .context("field 'name_id' is required on each enemy data")?
                .to_string();
            let is_big = element.attribute("isBig").is_some();
            let img_no = element
                .attribute("imgNo")
                .context("field 'imgNo' is required on each enemy data")?
                .parse()
                .context("parse 'imgNo'")?;
            let wait_action = element.attribute("waitAction").is_some();
            let library_rank = element
                .attributes()
                .filter(|a| a.name().starts_with("library_rank_"))
                .flat_map(|a| a.value().parse().context("parse 'library_rank_*'"))
                .collect::<Vec<_>>();
            let dlc = element
                .attributes()
                .filter(|a| a.name().starts_with("dlc_"))
                .map(|a| a.value().to_string())
                .collect::<Vec<_>>();
            let shoot_up = element.attribute("shoot_up").is_some();
            let monster_tag = element
                .attribute("monster_tag")
                .context("field 'monster_tag' is required on each enemy data")?
                .to_string();
            let chara_tag = element
                .attribute("chara_tag")
                .context("field 'chara_tag' is required on each enemy data")?
                .to_string();
            let race_tag = element
                .attribute("race_tag")
                .context("field 'race_tag' is required on each enemy data")?
                .to_string();
            let size = element
                .attribute("size")
                .context("field 'size' is required on each enemy data")?
                .to_string();
            let division = element
                .attribute("division")
                .context("field 'division' is required on each enemy data")?
                .to_string();

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
