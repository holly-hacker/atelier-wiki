use crate::extract::data::util;
use crate::extract::data::util::ElementReader;
use crate::utils::PakIndex;
use anyhow::Context;

pub struct FieldMapInfo2 {
    pub area_tag: String,
    pub region_tag: String,
    pub qua_min: Option<usize>,
    pub qua_max: Option<usize>,
    pub num_min: Option<usize>,
    pub num_max: Option<usize>,
    pub grade_min: Option<usize>,
    pub grade_max: Option<usize>,
}

impl FieldMapInfo2 {
    pub fn read(pak_index: &mut PakIndex) -> anyhow::Result<Vec<Self>> {
        // NOTE: reading as shift_jis
        util::read_xml_shift_jis(pak_index, r"\saves\fieldmap\info\fm_info2.xml", |d| {
            Self::read_from_doc(d)
        })
    }
    fn read_from_doc(document: roxmltree::Document) -> anyhow::Result<Vec<Self>> {
        let mut ret = vec![];

        let elements = document
            .root_element()
            .descendants()
            .filter(|n| n.tag_name().name() == "fm_info2");

        for (i, element) in elements.enumerate() {
            let node = Self::read_node(element).with_context(|| format!("read node {i}"))?;
            ret.push(node);
        }

        Ok(ret)
    }

    fn read_node(element: roxmltree::Node<'_, '_>) -> anyhow::Result<Self> {
        let reader = ElementReader(&element);

        let area_tag = reader.read("AreaTag")?;
        let region_tag = reader.read("RegionTag")?;
        let qua_min = reader.read_opt("quaMin")?;
        let qua_max = reader.read_opt("quaMax")?;
        let num_min = reader.read_opt("numMin")?;
        let num_max = reader.read_opt("numMax")?;
        let grade_min = reader.read_opt("gradeMin")?;
        let grade_max = reader.read_opt("gradeMax")?;

        Ok(Self {
            area_tag,
            region_tag,
            qua_min,
            qua_max,
            num_min,
            num_max,
            grade_min,
            grade_max,
        })
    }
}
