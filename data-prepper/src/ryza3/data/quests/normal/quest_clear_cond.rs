use crate::utils::{self, ElementReader, PakIndex};

pub struct QuestClearCond {
    pub clear_cond_tag: String,

    pub btl_detail_text: Option<String>,
    pub target_num: Option<u32>,
    pub symbol_group_tag: Option<String>,

    pub event_detail_text: Option<String>,
    pub clear_event_tag: Option<String>,

    pub achievement_detail_text: Option<String>,
    pub clear_achievement_tag: Option<String>,
    pub disp_progress: Option<i32>,

    pub quality_min: Option<u32>,
    pub quality_max: Option<u32>,
    pub delivery_item_num: Option<u32>,
    pub delivery_detail_text: Option<String>,

    pub trace_npc_tag: Option<String>,
    pub trace_pos_x: Option<f32>,
    pub trace_pos_y: Option<f32>,
    pub trace_pos_z: Option<f32>,
    pub trace_radius: Option<f32>,
    pub trace_atelier: Option<i32>,
    pub trace_field_map_tag: Option<String>,

    pub is_valid: Option<i32>,

    pub target_name: Option<String>,

    pub item_category_tag: Option<String>,

    pub item_tag: Option<String>,
    pub pot: Vec<String>,
    pub eff: Vec<String>,
}

impl QuestClearCond {
    pub fn read(pak_index: &mut PakIndex) -> anyhow::Result<Vec<Self>> {
        utils::read_xml(pak_index, r"\saves\quest\normal\questclearcond.xml", |d| {
            Self::read_from_doc(d)
        })
    }

    fn read_from_doc(document: roxmltree::Document) -> anyhow::Result<Vec<Self>> {
        let mut ret = vec![];

        let elements = document
            .root_element()
            .descendants()
            .filter(|n| n.tag_name().name() == "QuestClearCond");

        for element in elements {
            let reader = ElementReader(&element);

            let clear_cond_tag = reader.read("ClearCondTag")?;
            let btl_detail_text = reader.read_opt("BtlDetailText")?;
            let target_num = reader.read_opt("TargetNum")?;
            let symbol_group_tag = reader.read_opt("SymbolGroupTag")?;
            let event_detail_text = reader.read_opt("EventDetailText")?;
            let clear_event_tag = reader.read_opt("ClearEventTag")?;
            let achievement_detail_text = reader.read_opt("AchivementDetailText")?;
            let clear_achievement_tag = reader.read_opt("ClearAchivementTag")?;
            let disp_progress = reader.read_opt("DispProgress")?;
            let quality_min = reader.read_opt("QualityMin")?;
            let quality_max = reader.read_opt("QualityMax")?;
            let delivery_item_num = reader.read_opt("DeliveryItemNum")?;
            let delivery_detail_text = reader.read_opt("DeliveryDetailText")?;
            let trace_npc_tag = reader.read_opt("TraceNpcTag")?;
            let trace_pos_x = reader.read_opt("TracePos_x")?;
            let trace_pos_y = reader.read_opt("TracePos_y")?;
            let trace_pos_z = reader.read_opt("TracePos_z")?;
            let trace_radius = reader.read_opt("TraceRadius")?;
            let trace_atelier = reader.read_opt("TraceAtelier")?;
            let trace_field_map_tag = reader.read_opt("TraceFieldMapTag")?;
            let is_valid = reader.read_opt("isValid")?;
            let target_name = reader.read_opt("TargetName")?;
            let item_category_tag = reader.read_opt("ItemCategoryTag")?;
            let item_tag = reader.read_opt("ItemTag")?;
            let pot = reader.read_list("Pot*")?;
            let eff = reader.read_list("Eff*")?;

            ret.push(Self {
                clear_cond_tag,
                btl_detail_text,
                target_num,
                symbol_group_tag,
                event_detail_text,
                clear_event_tag,
                achievement_detail_text,
                clear_achievement_tag,
                disp_progress,
                quality_min,
                quality_max,
                delivery_item_num,
                delivery_detail_text,
                trace_npc_tag,
                trace_pos_x,
                trace_pos_y,
                trace_pos_z,
                trace_radius,
                trace_atelier,
                trace_field_map_tag,
                is_valid,
                target_name,
                item_category_tag,
                item_tag,
                pot,
                eff,
            })
        }
        debug_assert!(!ret.is_empty());

        Ok(ret)
    }
}
