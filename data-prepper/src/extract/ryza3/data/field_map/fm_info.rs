use crate::extract::ryza3::data::util;
use crate::extract::ryza3::data::util::ElementReader;
use crate::utils::PakIndex;
use anyhow::Context;

pub struct FieldMapInfo {
    pub field_map_name_id: Option<String>,
    pub data_file_name: String,
    pub load_region: Option<String>,
    pub range_min_x: usize,
    pub range_min_z: usize,
    pub range_max_x: usize,
    pub range_max_z: usize,
    pub navi_range_min_x: Option<usize>,
    pub navi_range_min_z: Option<usize>,
    pub navi_range_max_x: Option<usize>,
    pub navi_range_max_z: Option<usize>,
}

impl FieldMapInfo {
    pub fn read(pak_index: &mut PakIndex) -> anyhow::Result<Vec<Self>> {
        util::read_xml(pak_index, r"\saves\fieldmap\info\fm_info.xml", |d| {
            Self::read_from_doc(d)
        })
    }
    fn read_from_doc(document: roxmltree::Document) -> anyhow::Result<Vec<Self>> {
        let mut ret = vec![];

        let elements = document
            .root_element()
            .descendants()
            .filter(|n| n.tag_name().name() == "fm_info");

        for (i, element) in elements.enumerate() {
            let node = Self::read_node(element).with_context(|| format!("read node {i}"))?;
            ret.push(node);
        }

        Ok(ret)
    }

    fn read_node(element: roxmltree::Node<'_, '_>) -> anyhow::Result<Self> {
        let reader = ElementReader(&element);

        let name_id = reader.read_opt("FieldMapNameStrID")?;
        let data_file = reader.read("DataFileName")?;
        let region = reader.read_opt("LoadRegion")?;
        let range_min_x = reader.read("RangeMinX")?;
        let range_min_z = reader.read("RangeMinZ")?;
        let range_max_x = reader.read("RangeMaxX")?;
        let range_max_z = reader.read("RangeMaxZ")?;
        let navi_range_min_x = reader.read_opt("NaviRangeMinX")?;
        let navi_range_min_z = reader.read_opt("NaviRangeMinZ")?;
        let navi_range_max_x = reader.read_opt("NaviRangeMaxX")?;
        let navi_range_max_z = reader.read_opt("NaviRangeMaxZ")?;

        Ok(Self {
            field_map_name_id: name_id,
            data_file_name: data_file,
            load_region: region,
            range_min_x,
            range_min_z,
            range_max_x,
            range_max_z,
            navi_range_min_x,
            navi_range_min_z,
            navi_range_max_x,
            navi_range_max_z,
        })
    }
}
