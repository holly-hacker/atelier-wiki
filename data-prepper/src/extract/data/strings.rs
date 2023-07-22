use std::collections::HashMap;

use anyhow::Context;

pub struct StringsData {
    pub id_lookup: HashMap<String, String>,
    pub no_lookup: HashMap<usize, String>,
}

impl StringsData {
    pub fn read(document: roxmltree::Document) -> anyhow::Result<Self> {
        let mut id_lookup = HashMap::new();
        let mut no_lookup = HashMap::new();

        let elements = document
            .root_element()
            .descendants()
            .filter(|n| n.tag_name().name() == "str");
        for element in elements {
            let no: Option<usize> = element.attribute("String_No").and_then(|a| a.parse().ok());
            let id: Option<&str> = element.attribute("String_ID");
            let text: &str = element.attribute("Text").context("string must have text")?;

            if let Some(no) = no {
                no_lookup.insert(no, text.to_string());
            }
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
