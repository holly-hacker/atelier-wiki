//! Files of interest:
//! - [x] item\itemdata.xml: General item data
//! - [x] library\libraryitem.xml: Item info in in-game library, contains description.
//! - [ ] feeding\*.xml: Item drops for puni feeding
//! - [ ] shop\*.xml: Shop items
//! - [ ] item\item_use_data.xml: Item effects in battle

mod item_data;
mod library_item;

use anyhow::Context;
use serde::Serialize;
use tracing::debug;
use typescript_type_def::TypeDef;

use super::strings_table::StringsTable;
use crate::utils::PakIndex;

#[derive(Serialize, TypeDef)]
pub struct Item {
    /// The item tag. This is the closest we get to a string id but it does not exist for all items.
    pub tag: Option<String>,

    pub library_note: Option<String>,

    pub name: Option<String>,
    pub temp_name: Option<String>,
    pub temp_end_event: Option<String>,
    pub sort: i32,
    pub img_no: i32,
    pub price: u32,
    pub lv: u32,

    pub element: Option<u32>,
    pub element_value: Option<u32>,

    pub elem_fire: bool,
    pub elem_ice: bool,
    pub elem_thunder: bool,
    pub elem_air: bool,

    pub pc: Vec<i32>,

    pub hp: Option<i32>,
    pub atk: Option<i32>,
    pub def: Option<i32>,
    pub spd: Option<i32>,

    pub w_hp: Option<f32>,
    pub w_mp: Option<f32>,
    pub w_atk: Option<f32>,
    pub w_def: Option<f32>,
    pub w_spd: Option<f32>,

    /// The DLC required for this item.
    pub dlc: Option<String>,

    pub use_tag: String,
    pub kind_tag: String,

    pub bme: Option<String>,
    pub bmee: Option<String>,

    pub cat: Vec<String>,
}

impl Item {
    pub fn read(pak_index: &mut PakIndex, strings: &StringsTable) -> anyhow::Result<Vec<Self>> {
        debug!("Reading item data");
        let item_data = item_data::ItemData::read(pak_index).context("read item data")?;

        debug!("Reading library items");
        let library_items =
            library_item::LibraryItem::read(pak_index).context("read library items")?;

        let items = item_data
            .into_iter()
            .enumerate()
            .map(|(item_index, d)| {
                Ok(Self {
                    tag: library_items.get(item_index).map(|l| l.item_tag.clone()),
                    library_note: library_items.get(item_index).map(|l| {
                        l.note_id
                            .iter()
                            .filter_map(|id| strings.id_lookup.get(id).cloned())
                            .collect::<Vec<_>>()
                            .join("\n")
                    }),

                    name: d
                        .name_id
                        .and_then(|id| strings.id_lookup.get(&id).cloned())
                        .filter(|s| !s.is_empty()),
                    temp_name: d
                        .temp_name_id
                        .and_then(|id| strings.id_lookup.get(&id).cloned()),
                    temp_end_event: d.temp_end_event,
                    sort: d.sort,
                    img_no: d.img_no,
                    price: d.price,
                    lv: d.lv,
                    element: d.element,
                    element_value: d.element_value,
                    elem_fire: d.elem_fire,
                    elem_ice: d.elem_ice,
                    elem_thunder: d.elem_thunder,
                    elem_air: d.elem_air,
                    pc: d.pc,
                    hp: d.hp,
                    atk: d.atk,
                    def: d.def,
                    spd: d.spd,
                    w_hp: d.w_hp,
                    w_mp: d.w_mp,
                    w_atk: d.w_atk,
                    w_def: d.w_def,
                    w_spd: d.w_spd,
                    dlc: d.dlc.get(0).cloned(),
                    use_tag: d.use_tag,
                    kind_tag: d.kind_tag,
                    bme: d.bme,
                    bmee: d.bmee,
                    cat: d.cat,
                })
            })
            .collect::<anyhow::Result<Vec<Self>>>()
            .context("create items")?;

        Ok(items)
    }
}
