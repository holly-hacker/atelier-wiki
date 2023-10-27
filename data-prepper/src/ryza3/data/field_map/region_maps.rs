use crate::utils::{self, ElementReader, PakIndex};
use anyhow::Context;

pub struct RegionMap {
    pub name: String,
    pub style: String,
    pub rot: String,
    pub pos: String,
    pub scale: String,
}

impl RegionMap {
    pub fn read(pak_index: &mut PakIndex) -> anyhow::Result<Vec<Self>> {
        utils::read_xml(
            pak_index,
            r"\saves\ui_cmn\a24_map\uil_a24_overall_map.xml",
            Self::read_from_doc,
        )
    }
    fn read_from_doc(document: roxmltree::Document) -> anyhow::Result<Vec<Self>> {
        let mut ret = vec![];

        let elements = document
            .root_element()
            .descendants()
            .find(|n| n.tag_name().name() == "node" && n.attribute("name") == Some("bg_images"))
            .context("find bg_images node")?
            .descendants()
            .filter(|n| n.tag_name().name() == "image");

        for (i, element) in elements.enumerate() {
            let node = Self::read_node(element).with_context(|| format!("read node {i}"))?;
            ret.push(node);
        }

        Ok(ret)
    }

    fn read_node(element: roxmltree::Node<'_, '_>) -> anyhow::Result<Self> {
        let reader = ElementReader(&element);

        let name = reader.read("name")?;
        let style = reader.read("style")?;
        let rot = reader.read("rot")?;
        let pos = reader.read("pos")?;
        let scale = reader.read("scale")?;

        Ok(Self {
            name,
            style,
            rot,
            pos,
            scale,
        })
    }
}
