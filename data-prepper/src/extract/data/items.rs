use serde::Serialize;
use tracing::trace;
use typescript_type_def::TypeDef;

use crate::extract::{data::util::ElementReader, pak_index::PakIndex};

use super::{strings::StringsData, util};

#[derive(Serialize, TypeDef)]
pub struct ItemData {
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

    pub dlc: Vec<String>, // note: in practice this contains only 1 item

    pub use_tag: String,
    pub kind_tag: String,

    pub bme: Option<String>,
    pub bmee: Option<String>,

    pub cat: Vec<String>,
}

impl ItemData {
    pub fn read(pak_index: &mut PakIndex, strings: &StringsData) -> anyhow::Result<Vec<Self>> {
        util::read_xml(pak_index, r"\saves\item\itemdata.xml", |d| {
            Self::read_from_doc(d, strings)
        })
    }

    pub fn read_from_doc(
        document: roxmltree::Document,
        strings: &StringsData,
    ) -> anyhow::Result<Vec<Self>> {
        let mut ret = vec![];

        // NOTE: encoding in header seems to be SHIFT-JIS, may need to account for that?
        let elements = document
            .root_element()
            .descendants()
            .filter(|n| n.tag_name().name() == "itemData");

        for element in elements {
            let reader = ElementReader(&element);

            // start with required properties
            let sort = reader.read("sort")?;
            trace!(?sort, "Reading item");
            let img_no = reader.read("imgNo")?;
            let price = reader.read("price")?;
            let lv = reader.read("lv")?;
            let use_tag = reader.read("useTag")?;
            let kind_tag = reader.read("kindTag")?;

            // resolvable strings
            // we're not going to fail if we can't resolve them, some items (eg. STR_ITEM_NAME_744) don't have a string
            let name = reader
                .read_opt::<String>("nameID")?
                .and_then(|s| strings.id_lookup.get(&s).cloned());
            let temp_name = reader
                .read_opt::<String>("tempNameID")?
                .and_then(|s| strings.id_lookup.get(&s).cloned());

            // optional string properties
            let temp_end_event = reader.read_opt("tempEndEvent")?;
            let bme = reader.read_opt("bme")?;
            let bmee = reader.read_opt("bmee")?;

            // optional numbers properties
            // notably, elemFire, elemIce, elemThunder and elemAir may incorrectly contain the value "TURE"
            let element_ = reader.read_opt("element")?;
            let element_value = reader.read_opt("elementValue")?;
            let elem_fire = reader.read_present("elemFire");
            let elem_ice = reader.read_present("elemIce");
            let elem_thunder = reader.read_present("elemThunder");
            let elem_air = reader.read_present("elemAir");

            let hp = reader.read_opt("hp")?;
            let atk = reader.read_opt("atk")?;
            let def = reader.read_opt("def")?;
            let spd = reader.read_opt("spd")?;
            let w_hp = reader.read_opt("w_hp")?;
            let w_mp = reader.read_opt("w_mp")?;
            let w_atk = reader.read_opt("w_atk")?;
            let w_def = reader.read_opt("w_def")?;
            let w_spd = reader.read_opt("w_spd")?;

            // optional list properties
            let pc = reader.read_list("pc_*");
            let dlc = reader.read_list("dlc_*");
            let cat = reader.read_list("cat_*");

            debug_assert!(dlc.len() <= 1);

            let data_item = Self {
                name,
                temp_name,
                temp_end_event,
                sort,
                img_no,
                price,
                lv,

                element: element_,
                element_value,

                elem_fire,
                elem_ice,
                elem_thunder,
                elem_air,

                pc,

                hp,
                atk,
                def,
                spd,

                w_hp,
                w_mp,
                w_atk,
                w_def,
                w_spd,

                dlc,
                use_tag,
                kind_tag,

                bme,
                bmee,

                cat,
            };

            ret.push(data_item);
        }

        Ok(ret)
    }
}
