//! Data from `\saves\feeding\feedingspecies.xml`

use crate::{
    extract::util::{self, ElementReader},
    utils::PakIndex,
};

pub struct Species {
    pub no: usize,
    pub chara_tag: String,
    pub name: String,
    pub image_no: usize,
    pub ene_min: u32,
    pub ene_max: u32,
    pub color_min: u32,
    pub color_max: u32,
    pub mood_min: u32,
    pub mood_max: u32,

    pub rank_e_min: i32,
    pub rank_e_max: i32,
    pub rank_d_min: i32,
    pub rank_d_max: i32,
    pub rank_c_min: i32,
    pub rank_c_max: i32,
    pub rank_b_min: i32,
    pub rank_b_max: i32,
    pub rank_a_min: i32,
    pub rank_a_max: i32,
    pub rank_s_min: i32,
    pub rank_s_max: i32,

    pub category_tags: Vec<String>,
}

impl Species {
    pub fn read(pak_index: &mut PakIndex) -> anyhow::Result<Vec<Self>> {
        util::read_xml(
            pak_index,
            r"\saves\feeding\feedingspecies.xml",
            Self::read_from_doc,
        )
    }

    fn read_from_doc(document: roxmltree::Document) -> anyhow::Result<Vec<Self>> {
        let mut ret = vec![];

        let elements = document
            .root_element()
            .descendants()
            .filter(|n| n.tag_name().name() == "FeedingSpecies");

        for element in elements {
            let reader = ElementReader(&element);

            let no = reader.read("No")?;
            let chara_tag = reader.read("charaTag")?;
            let name = reader.read("name")?;
            let image_no = reader.read("imageNo")?;

            let ene_min = reader.read("eneMin")?;
            let ene_max = reader.read("eneMax")?;
            let color_min = reader.read("colorMin")?;
            let color_max = reader.read("colorMax")?;
            let mood_min = reader.read("moodMin")?;
            let mood_max = reader.read("moodMax")?;

            let rank_e_min = reader.read("rankEMin")?;
            let rank_e_max = reader.read("rankEMax")?;
            let rank_d_min = reader.read("rankDMin")?;
            let rank_d_max = reader.read("rankDMax")?;
            let rank_c_min = reader.read("rankCMin")?;
            let rank_c_max = reader.read("rankCMax")?;
            let rank_b_min = reader.read("rankBMin")?;
            let rank_b_max = reader.read("rankBMax")?;
            let rank_a_min = reader.read("rankAMin")?;
            let rank_a_max = reader.read("rankAMax")?;
            let rank_s_min = reader.read("rankSMin")?;
            let rank_s_max = reader.read("rankSMax")?;

            let category_tag = reader.read_list("category_tag_*")?;

            let node = Self {
                no,
                chara_tag,
                name,
                image_no,
                ene_min,
                ene_max,
                color_min,
                color_max,
                mood_min,
                mood_max,
                rank_e_min,
                rank_e_max,
                rank_d_min,
                rank_d_max,
                rank_c_min,
                rank_c_max,
                rank_b_min,
                rank_b_max,
                rank_a_min,
                rank_a_max,
                rank_s_min,
                rank_s_max,
                category_tags: category_tag,
            };

            ret.push(node);
        }

        Ok(ret)
    }
}
