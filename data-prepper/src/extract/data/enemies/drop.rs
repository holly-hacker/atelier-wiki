use crate::extract::{
    data::util::{self, ElementReader},
    pak_index::PakIndex,
};

#[derive(Debug)]
pub struct DropData {
    pub item_tag: Vec<String>,
    pub rate: Vec<u32>,
    pub num: Vec<u32>,

    // only quality_{min/max}_adj, potential_{min/max}_adj and quality_max are encountered as f32,
    // the rest can be parsed as u32. however, to be consistent all are stored to f32.
    pub quality_min: Vec<f32>,
    pub quality_max: Vec<f32>,
    pub potential_min: Vec<f32>,
    pub potential_max: Vec<f32>,
    pub potential_num_min: Vec<f32>,
    pub potential_num_max: Vec<f32>,
    pub potential_lv_min: Vec<f32>,
    pub potential_lv_max: Vec<f32>,

    pub quality_min_adj: Vec<f32>,
    pub quality_max_adj: Vec<f32>,
    pub potential_min_adj: Vec<f32>,
    pub potential_max_adj: Vec<f32>,
    pub potential_num_min_adj: Vec<u32>,
    pub potential_num_max_adj: Vec<u32>,
    pub potential_lv_min_adj: Vec<u32>,
    pub potential_lv_max_adj: Vec<u32>,

    pub super_pot_rate: Vec<u32>,
    pub factor: Vec<String>,
    pub eff: Vec<String>,

    // this vec seems to be unrelated to the list of drops
    pub sp_item_tag: Vec<String>,
}

impl DropData {
    pub fn read(pak_index: &mut PakIndex) -> anyhow::Result<Vec<Self>> {
        util::read_xml(pak_index, r"\saves\enemy\drop_data.xml", |d| {
            Self::read_from_doc(d)
        })
    }

    fn read_from_doc(document: roxmltree::Document) -> anyhow::Result<Vec<Self>> {
        let mut ret = vec![];

        let elements = document
            .root_element()
            .descendants()
            .filter(|n| n.tag_name().name() == "drop_data");

        for element in elements {
            let reader = ElementReader(&element);

            let item_tag = reader.read_list("item_tag_*")?;
            let rate = reader.read_list("rate_*")?;
            let num = reader.read_list("num_*")?;

            let quality_min = reader.read_list("quality_min_*")?;
            let quality_max = reader.read_list("quality_max_*")?;
            let potential_min = reader.read_list("potential_min_*")?;
            let potential_max = reader.read_list("potential_max_*")?;
            let potential_num_min = reader.read_list("potential_num_min_*")?;
            let potential_num_max = reader.read_list("potential_num_max_*")?;
            let potential_lv_min = reader.read_list("potential_lv_min_*")?;
            let potential_lv_max = reader.read_list("potential_lv_max_*")?;

            let quality_min_adj = reader.read_list("quality_min_*_adj")?;
            let quality_max_adj = reader.read_list("quality_max_*_adj")?;
            let potential_min_adj = reader.read_list("potential_min_*_adj")?;
            let potential_max_adj = reader.read_list("potential_max_*_adj")?;
            let potential_num_min_adj = reader.read_list("potential_num_min_*_adj")?;
            let potential_num_max_adj = reader.read_list("potential_num_max_*_adj")?;
            let potential_lv_min_adj = reader.read_list("potential_lv_min_*_adj")?;
            let potential_lv_max_adj = reader.read_list("potential_lv_max_*_adj")?;

            let super_pot_rate = reader.read_list("super_pot_rate_*")?;
            let factor = reader.read_list("factor_*")?;
            let eff = reader.read_list("eff_*")?;

            let sp_item_tag = reader.read_list("sp_item_tag_*")?;

            // sanity check on all properties that are required on each sub-instance of a drop
            debug_assert_eq!(num.len(), item_tag.len());
            debug_assert_eq!(num.len(), rate.len());
            debug_assert_eq!(num.len(), quality_min.len());
            debug_assert_eq!(num.len(), quality_max.len());
            debug_assert_eq!(num.len(), potential_min.len());
            debug_assert_eq!(num.len(), potential_max.len());
            debug_assert_eq!(num.len(), potential_num_min.len());
            debug_assert_eq!(num.len(), potential_num_max.len());
            debug_assert_eq!(num.len(), potential_lv_min.len());
            debug_assert_eq!(num.len(), potential_lv_max.len());
            debug_assert_eq!(num.len(), quality_min_adj.len());
            debug_assert_eq!(num.len(), quality_max_adj.len());
            debug_assert_eq!(num.len(), potential_min_adj.len());
            debug_assert_eq!(num.len(), potential_max_adj.len());
            debug_assert_eq!(num.len(), potential_num_min_adj.len());
            debug_assert_eq!(num.len(), potential_num_max_adj.len());
            debug_assert_eq!(num.len(), potential_lv_min_adj.len());
            debug_assert_eq!(num.len(), potential_lv_max_adj.len());

            debug_assert_eq!(sp_item_tag.len(), 1);

            ret.push(Self {
                item_tag,
                rate,
                num,

                quality_min,
                quality_max,
                potential_min,
                potential_max,
                potential_num_min,
                potential_num_max,
                potential_lv_min,
                potential_lv_max,

                quality_min_adj,
                quality_max_adj,
                potential_min_adj,
                potential_max_adj,
                potential_num_min_adj,
                potential_num_max_adj,
                potential_lv_min_adj,
                potential_lv_max_adj,

                super_pot_rate,
                factor,
                eff,

                sp_item_tag,
            });
        }

        Ok(ret)
    }
}
