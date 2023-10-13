use crate::{
    extract::util::{self, ElementReader},
    utils::PakIndex,
};

pub struct LibraryItem {
    pub item_tag: String,
    pub note_id: Vec<String>,
    pub ep: Vec<Option<i32>>,
    pub permit: bool,
}

impl LibraryItem {
    pub fn read(pak_index: &mut PakIndex) -> anyhow::Result<Vec<Self>> {
        util::read_xml(pak_index, r"\saves\library\libraryitem.xml", |d| {
            Self::read_from_doc(d)
        })
    }

    fn read_from_doc(document: roxmltree::Document) -> anyhow::Result<Vec<Self>> {
        let mut ret = vec![];

        let elements = document
            .root_element()
            .descendants()
            .filter(|n| n.tag_name().name() == "LibraryItem");

        for element in elements {
            let reader = ElementReader(&element);

            let item = Self {
                item_tag: reader.read("itemTag")?,
                note_id: reader.read_list("note_id_*")?,
                ep: reader.read_sparse_list("ep*")?,
                permit: reader.is_present("permit"),
            };

            ret.push(item);
        }

        Ok(ret)
    }
}
