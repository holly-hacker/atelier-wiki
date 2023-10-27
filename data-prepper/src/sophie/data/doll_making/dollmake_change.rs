use crate::utils::{self, ElementReader, PakIndex};

/// `dollmake_change` from `r\Saves\dollmake\dollmake_change.xml`
pub struct DollmakeChange {
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

    pub doll_mhp: usize,
    pub doll_mmp: usize,
    pub doll_mlp: usize,
    pub doll_atk: usize,
    pub doll_def: usize,
    pub doll_spd: usize,
    pub doll_dmg_min: usize,
    pub doll_dmg_max: usize,
}

impl DollmakeChange {
    pub fn read(pak_index: &mut PakIndex) -> anyhow::Result<Vec<Self>> {
        utils::read_xml(pak_index, r"\Saves\dollmake\dollmake_change.xml", |d| {
            Self::read_from_doc(d)
        })
    }

    fn read_from_doc(document: roxmltree::Document) -> anyhow::Result<Vec<Self>> {
        let mut ret = vec![];

        let elements = document
            .root_element()
            .descendants()
            .filter(|n| n.tag_name().name() == "dollmake_change");

        for element in elements {
            let reader = ElementReader(&element);

            let no = reader.read("No")?;
            let name = reader.read("name")?;
            let doll_tag = reader.read("doll_tag")?;
            let chara_base_tag = reader.read("chara_base_tag")?;
            let dlc_tag = reader.read("dlc_tag")?;
            let doll_event_tag = reader.read("doll_event_tag")?;
            let cute_min = reader.read("CUTE_MIN")?;
            let cute_max = reader.read("CUTE_MAX")?;
            let wise_min = reader.read("WISE_MIN")?;
            let wise_max = reader.read("WISE_MAX")?;
            let brave_min = reader.read("BRAVE_MIN")?;
            let brave_max = reader.read("BRAVE_MAX")?;
            let fool_min = reader.read("FOOL_MIN")?;
            let fool_max = reader.read("FOOL_MAX")?;
            let doll_mhp = reader.read("DOLL_MHP")?;
            let doll_mmp = reader.read("DOLL_MMP")?;
            let doll_mlp = reader.read("DOLL_MLP")?;
            let doll_atk = reader.read("DOLL_ATK")?;
            let doll_def = reader.read("DOLL_DEF")?;
            let doll_spd = reader.read("DOLL_SPD")?;
            let doll_dmg_min = reader.read("DOLL_DMG_MIN")?;
            let doll_dmg_max = reader.read("DOLL_DMG_MAX")?;

            ret.push(Self {
                no,
                name,
                doll_tag,
                chara_base_tag,
                dlc_tag,
                doll_event_tag,
                cute_min,
                cute_max,
                wise_min,
                wise_max,
                brave_min,
                brave_max,
                fool_min,
                fool_max,
                doll_mhp,
                doll_mmp,
                doll_mlp,
                doll_atk,
                doll_def,
                doll_spd,
                doll_dmg_min,
                doll_dmg_max,
            })
        }

        Ok(ret)
    }
}
