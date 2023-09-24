use crate::extract::data::strings_table::StringsTable;
use crate::utils::PakIndex;
use serde::Serialize;
use typescript_type_def::TypeDef;

mod fm_info;
mod fm_info2;

#[derive(Serialize, TypeDef)]
pub struct FieldMapData {
    pub field_maps: Vec<FieldMap>,
}

#[derive(Serialize, TypeDef)]
pub struct FieldMap {
    pub field_map_name: Option<String>,
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

    pub area_tag: String,
    pub region_tag: String,
    pub qua_min: Option<usize>,
    pub qua_max: Option<usize>,
    pub num_min: Option<usize>,
    pub num_max: Option<usize>,
    pub grade_min: Option<usize>,
    pub grade_max: Option<usize>,
}

impl FieldMapData {
    pub fn read(pak_index: &mut PakIndex, strings: &StringsTable) -> anyhow::Result<Self> {
        let fm_info = fm_info::FieldMapInfo::read(pak_index)?;
        let fm_info2 = fm_info2::FieldMapInfo2::read(pak_index)?;

        assert_eq!(fm_info.len(), fm_info2.len());

        let field_maps = fm_info
            .into_iter()
            .zip(fm_info2.into_iter())
            .map(|(info, info2)| FieldMap {
                field_map_name: info
                    .field_map_name_id
                    .and_then(|id| strings.id_lookup.get(&id))
                    .cloned(),
                data_file_name: info.data_file_name,
                load_region: info.load_region,
                range_min_x: info.range_min_x,
                range_min_z: info.range_min_z,
                range_max_x: info.range_max_x,
                range_max_z: info.range_max_z,
                navi_range_min_x: info.navi_range_min_x,
                navi_range_min_z: info.navi_range_min_z,
                navi_range_max_x: info.navi_range_max_x,
                navi_range_max_z: info.navi_range_max_z,

                area_tag: info2.area_tag,
                region_tag: info2.region_tag,
                qua_min: info2.qua_min,
                qua_max: info2.qua_max,
                num_min: info2.num_min,
                num_max: info2.num_max,
                grade_min: info2.grade_min,
                grade_max: info2.grade_max,
            })
            .collect();

        Ok(Self { field_maps })
    }
}
