use crate::utils::{self, ElementReader, PakIndex};

pub struct NormalQuest {
    pub quest_tag: Option<String>,
    pub quest_title: Option<String>,
    pub quest_cond_tag: Option<String>,
    pub start_exec_ev: Option<String>,
    pub quest_flavor: Option<String>,
    pub clear_cond_tag: Vec<String>,
    pub quest_flavor_after: Option<String>,
    pub prize_tag: Option<String>,
    pub image_no: Option<u32>,
    pub is_valid: Option<i32>,
}

impl NormalQuest {
    pub fn read(pak_index: &mut PakIndex) -> anyhow::Result<Vec<Self>> {
        utils::read_xml(pak_index, r"\saves\quest\normal\normalquest.xml", |d| {
            Self::read_from_doc(d)
        })
    }

    fn read_from_doc(document: roxmltree::Document) -> anyhow::Result<Vec<Self>> {
        let mut ret = vec![];

        let elements = document
            .root_element()
            .descendants()
            .filter(|n| n.tag_name().name() == "NormalQuest");

        for element in elements {
            let reader = ElementReader(&element);

            let quest_tag = reader.read_opt("QuestTag")?;
            let quest_title = reader.read_opt("QuestTitle")?;
            let quest_cond_tag = reader.read_opt("QuestCondTag")?;
            let start_exec_ev = reader.read_opt("StartExecEv")?;
            let quest_flavor = reader.read_opt("QuestFlavor")?;
            let clear_cond_tag = reader.read_flattened_sparse_list("ClearCondTag*")?;
            let quest_flavor_after = reader.read_opt("QuestFlavorAfter")?;
            let prize_tag = reader.read_opt("PrizeTag")?;
            let image_no = reader.read_opt("ImageNo")?;
            let is_valid = reader.read_opt("isValid")?;

            ret.push(Self {
                quest_tag,
                quest_title,
                quest_cond_tag,
                start_exec_ev,
                quest_flavor,
                clear_cond_tag,
                quest_flavor_after,
                prize_tag,
                image_no,
                is_valid,
            })
        }
        debug_assert!(!ret.is_empty());

        Ok(ret)
    }
}
