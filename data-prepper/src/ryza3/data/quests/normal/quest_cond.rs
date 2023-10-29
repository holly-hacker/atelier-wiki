use crate::utils::{self, ElementReader, PakIndex};

pub struct QuestCond {
    pub quest_cond_tag: String,
    pub acceptable_cond_event: Vec<String>,
}

impl QuestCond {
    pub fn read(pak_index: &mut PakIndex) -> anyhow::Result<Vec<Self>> {
        utils::read_xml(pak_index, r"\saves\quest\normal\questcond.xml", |d| {
            Self::read_from_doc(d)
        })
    }

    fn read_from_doc(document: roxmltree::Document) -> anyhow::Result<Vec<Self>> {
        let mut ret = vec![];

        let elements = document
            .root_element()
            .descendants()
            .filter(|n| n.tag_name().name() == "QuestCond");

        for element in elements {
            let reader = ElementReader(&element);

            let quest_cond_tag = reader.read("QuestCondTag")?;
            let acceptable_cond_event = reader.read_list("AcceptableCondEvent*")?;

            ret.push(Self {
                quest_cond_tag,
                acceptable_cond_event,
            })
        }
        debug_assert!(!ret.is_empty());

        Ok(ret)
    }
}
