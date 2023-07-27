use std::collections::HashMap;

use anyhow::Context;

use crate::extract::pak_index::PakIndex;

use super::util::{self, ElementReader};

pub struct StringsData {
    pub id_lookup: HashMap<String, String>,
    pub no_lookup: HashMap<usize, String>,
}

impl StringsData {
    pub fn read(pak_index: &mut PakIndex) -> anyhow::Result<Self> {
        util::read_xml(
            pak_index,
            r"\saves\text_en\strcombineall.xml",
            Self::read_from_doc,
        )
    }
    fn read_from_doc(document: roxmltree::Document) -> anyhow::Result<Self> {
        let mut id_lookup = HashMap::new();
        let mut no_lookup = HashMap::new();

        let elements = document
            .root_element()
            .descendants()
            .filter(|n| n.tag_name().name() == "str");
        for element in elements {
            let reader = ElementReader(&element);

            let no: usize = reader.read("String_No")?;
            let id: Option<&str> = element.attribute("String_ID");
            let text: &str = element.attribute("Text").context("string must have text")?;

            no_lookup.insert(no, text.to_string());

            if let Some(id) = id {
                id_lookup.insert(id.to_string(), text.to_string());
            }
        }

        Ok(Self {
            id_lookup,
            no_lookup,
        })
    }
}
