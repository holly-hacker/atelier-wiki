//! Data from `\saves\feeding\feedinguniqueitemevent.xml`

use crate::{
    extract::util::{self, ElementReader},
    utils::PakIndex,
};

pub struct FeedingUniqueItemEvent {
    pub no: usize,
    pub param: Vec<String>,
    pub event: String,
    pub cond: String,
    pub item: String,
}

impl FeedingUniqueItemEvent {
    pub fn read(pak_index: &mut PakIndex) -> anyhow::Result<Vec<Self>> {
        util::read_xml(
            pak_index,
            r"\saves\feeding\feedinguniqueitemevent.xml",
            Self::read_from_doc,
        )
    }

    fn read_from_doc(document: roxmltree::Document) -> anyhow::Result<Vec<Self>> {
        let mut ret = vec![];

        let elements = document
            .root_element()
            .descendants()
            .filter(|n| n.tag_name().name() == "FeedingUniqueItemEvent");

        for element in elements {
            let reader = ElementReader(&element);

            let no = reader.read("No")?;
            let param = reader.read_flattened_sparse_list("param*")?;
            let event = reader.read("event")?;
            let cond = reader.read("cond")?;
            let item = reader.read("item")?;

            let node = Self {
                no,
                param,
                event,
                cond,
                item,
            };

            ret.push(node);
        }

        Ok(ret)
    }
}
