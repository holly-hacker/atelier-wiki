use crate::utils::{self, ElementReader, PakIndex};

/// `present_ex` from `\Saves\Friend\present_ex.xml`
pub struct PresentEx {
    pub friend_tag: String,
    pub item_tag: String,
    pub pts: f32,
}

impl PresentEx {
    pub fn read(pak_index: &mut PakIndex) -> anyhow::Result<Vec<Self>> {
        utils::read_xml(pak_index, r"\Saves\Friend\present_ex.xml", |d| {
            Self::read_from_doc(d)
        })
    }

    fn read_from_doc(document: roxmltree::Document) -> anyhow::Result<Vec<Self>> {
        let mut ret = vec![];

        let elements = document
            .root_element()
            .descendants()
            .filter(|n| n.tag_name().name() == "present_ex");

        for element in elements {
            let reader = ElementReader(&element);

            let friend_tag = reader.read("friend_tag")?;
            let item_tag = reader.read("item_tag")?;
            let pts = reader.read("pts")?;

            ret.push(Self {
                friend_tag,
                item_tag,
                pts,
            })
        }

        Ok(ret)
    }
}
