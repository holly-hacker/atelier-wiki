use anyhow::Context;
use roxmltree::Node;

use crate::{
    extract::data::util::{self, ElementReader},
    utils::PakIndex,
};

pub struct ItemEffect {
    pub name_id: Option<String>,
    pub kind: Option<String>,
    pub base_att_tag: String,

    pub att_tag: Vec<Option<String>>,
    pub act_tag: Vec<Option<String>>,

    pub min_1: Vec<Option<String>>,
    pub max_1: Vec<Option<String>>,

    pub min_2: Vec<Option<String>>,
    pub max_2: Vec<Option<String>>,
}

impl ItemEffect {
    pub fn read(pak_index: &mut PakIndex) -> anyhow::Result<Vec<Self>> {
        util::read_xml(pak_index, r"\saves\item\item_effect.xml", |d| {
            Self::read_from_doc(d)
        })
    }

    fn read_from_doc(document: roxmltree::Document) -> anyhow::Result<Vec<Self>> {
        let mut ret = vec![];

        // NOTE: encoding in header seems to be SHIFT-JIS, may need to account for that?
        let elements = document
            .root_element()
            .descendants()
            .filter(|n| n.tag_name().name() == "item_effect");

        for (i, element) in elements.enumerate() {
            let node = Self::read_node(element).with_context(|| format!("read node {i}"))?;
            ret.push(node);
        }

        Ok(ret)
    }

    fn read_node(element: Node<'_, '_>) -> anyhow::Result<Self> {
        let reader = ElementReader(&element);

        // start with required properties
        let name_id = reader.read_opt("nameID")?;

        let kind = reader.read_opt("kind")?;
        let base_att_tag = reader.read("baseAttTag")?;

        // we read as sparse but in reality it's not. there just seems to be "padding" in some
        // effects where higher values are default values (eg. STR_ITEM_EFFECT_0397)
        let att_tag = reader.read_sparse_list("attTag_*")?;
        let act_tag = reader.read_sparse_list("actTag_*")?;

        let min_1 = reader.read_sparse_list("min_1_*")?;
        let max_1 = reader.read_sparse_list("max_1_*")?;
        let min_2 = reader.read_sparse_list("min_2_*")?;
        let max_2 = reader.read_sparse_list("max_2_*")?;

        // we could place some asserts here but the data is so inconsistent that it's not worth it

        Ok(Self {
            name_id,
            kind,
            base_att_tag,
            att_tag,
            act_tag,
            min_1,
            max_1,
            min_2,
            max_2,
        })
    }
}
