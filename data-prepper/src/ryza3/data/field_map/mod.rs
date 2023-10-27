use crate::ryza3::data::strings_table::StringsTable;
use crate::utils::PakIndex;
use serde::Serialize;
use std::collections::BTreeMap;
use typescript_type_def::TypeDef;

mod fm_info;
mod fm_info2;
mod region_maps;

#[derive(Serialize, TypeDef)]
pub struct FieldMapData {
    pub field_maps: Vec<FieldMap>,
    pub region_maps: BTreeMap<usize, RegionMap>,
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

#[derive(Serialize, TypeDef)]
pub struct RegionMap {
    pub image_name: String,
    pub rot: [f32; 3],
    pub pos: [f32; 3],
    pub scale: [f32; 3],
}

impl FieldMapData {
    pub fn read(pak_index: &mut PakIndex, strings: &StringsTable) -> anyhow::Result<Self> {
        let fm_info = fm_info::FieldMapInfo::read(pak_index)?;
        let fm_info2 = fm_info2::FieldMapInfo2::read(pak_index)?;
        let region_maps = region_maps::RegionMap::read(pak_index)?;

        assert_eq!(fm_info.len(), fm_info2.len());

        let field_maps = fm_info
            .into_iter()
            .zip(fm_info2)
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

        let region_maps = region_maps
            .into_iter()
            .map(|r| {
                assert!(r.name.starts_with("bg_image_"));
                assert!(r.style.starts_with("gen_a24_minimap_all_"));

                let index = r.style["gen_a24_minimap_all_".len()..].parse().unwrap();

                let map = RegionMap {
                    image_name: r.name["bg_image_".len()..].to_string(),
                    rot: {
                        let mut split = r.rot.split(", ");
                        [
                            split.next().unwrap().parse().unwrap(),
                            split.next().unwrap().parse().unwrap(),
                            split.next().unwrap().parse().unwrap(),
                        ]
                    },
                    pos: {
                        let mut split = r.pos.split(", ");
                        [
                            split.next().unwrap().parse().unwrap(),
                            split.next().unwrap().parse().unwrap(),
                            split.next().unwrap().parse().unwrap(),
                        ]
                    },
                    scale: {
                        let mut split = r.scale.split(", ");
                        [
                            split.next().unwrap().parse().unwrap(),
                            split.next().unwrap().parse().unwrap(),
                            split.next().unwrap().parse().unwrap(),
                        ]
                    },
                };

                (index, map)
            })
            .collect();

        Ok(Self {
            field_maps,
            region_maps,
        })
    }
}
