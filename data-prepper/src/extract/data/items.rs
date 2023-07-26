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
            let read = ElementReader(&element);

            // start with required properties
            let sort = read.read_parse("sort")?;
            trace!(?sort, "Reading item");
            let img_no = read.read_parse("imgNo")?;
            let price = read.read_parse("price")?;
            let lv = read.read_parse("lv")?;
            let use_tag = read.read_string("useTag")?;
            let kind_tag = read.read_string("kindTag")?;

            // resolvable strings
            // we're not going to fail if we can't resolve them, some items (eg. STR_ITEM_NAME_744) don't have a string
            let name = read
                .read_string_opt("nameID")
                .and_then(|s| strings.id_lookup.get(&s).cloned());
            let temp_name = read
                .read_string_opt("tempNameID")
                .and_then(|s| strings.id_lookup.get(&s).cloned());

            // optional string properties
            let temp_end_event = read.read_string_opt("tempEndEvent");
            let bme = read.read_string_opt("bme");
            let bmee = read.read_string_opt("bmee");

            // optional numbers properties
            // notably, elemFire, elemIce, elemThunder and elemAir may incorrectly contain the value "TURE"
            let element_ = read.read_parse_opt("element")?;
            let element_value = read.read_parse_opt("elementValue")?;
            let elem_fire = read.read_present("elemFire");
            let elem_ice = read.read_present("elemIce");
            let elem_thunder = read.read_present("elemThunder");
            let elem_air = read.read_present("elemAir");

            let hp = read.read_parse_opt("hp")?;
            let atk = read.read_parse_opt("atk")?;
            let def = read.read_parse_opt("def")?;
            let spd = read.read_parse_opt("spd")?;
            let w_hp = read.read_parse_opt("w_hp")?;
            let w_mp = read.read_parse_opt("w_mp")?;
            let w_atk = read.read_parse_opt("w_atk")?;
            let w_def = read.read_parse_opt("w_def")?;
            let w_spd = read.read_parse_opt("w_spd")?;

            // optional list properties
            let pc = read.read_parse_list("pc_*");
            let dlc = read.read_parse_list("dlc_*");
            let cat = read.read_parse_list("cat_*");

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
