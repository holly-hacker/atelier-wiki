use anyhow::Context;
use serde::Serialize;
use tracing::trace;

use super::strings::StringsData;

#[derive(Serialize)]
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

    pub elem_fire: Option<i32>,
    pub elem_ice: Option<i32>,
    pub elem_thunder: Option<i32>,
    pub elem_air: Option<i32>,

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
    pub fn read(document: roxmltree::Document, strings: &StringsData) -> anyhow::Result<Vec<Self>> {
        let mut ret = vec![];

        // NOTE: encoding in header seems to be SHIFT-JIS, may need to account for that?
        let elements = document
            .root_element()
            .descendants()
            .filter(|n| n.tag_name().name() == "itemData");

        for element in elements {
            // start with required properties
            let sort = element
                .attribute("sort")
                .context("field 'sort' is required on each item")?
                .parse()
                .context("parse 'sort'")?;
            trace!(?sort, "Reading item");
            let img_no = element
                .attribute("imgNo")
                .context("field 'imgNo' is required on each item")?
                .parse()
                .context("parse 'imgNo'")?;
            let price = element
                .attribute("price")
                .context("field 'price' is required on each item")?
                .parse()
                .context("parse 'price'")?;
            let lv = element
                .attribute("lv")
                .context("field 'lv' is required on each item")?
                .parse()
                .context("parse 'lv'")?;
            let use_tag = element
                .attribute("useTag")
                .context("field 'useTag' is required on each item")?
                .to_string();
            let kind_tag = element
                .attribute("kindTag")
                .context("field 'kindTag' is required on each item")?
                .to_string();

            // resolvable strings
            // we're not going to fail if we can't resolve them, some items (eg. STR_ITEM_NAME_744) don't have a string
            let name = element
                .attribute("nameID")
                .and_then(|id| strings.id_lookup.get(id).cloned());
            let temp_name = element
                .attribute("tempNameID")
                .and_then(|id| strings.id_lookup.get(id).cloned());

            // optional string properties
            let temp_end_event = element.attribute("tempEndEvent").map(|s| s.to_string());
            let bme = element.attribute("bme").map(|s| s.to_string());
            let bmee = element.attribute("bmee").map(|s| s.to_string());

            // optional numbers properties
            // notably, elemFire, elemIce, elemThunder and elemAir may incorrectly contain the value "TURE"
            let element_ = element
                .attribute("element")
                .map(|s| s.parse().context("parse 'element'"))
                .transpose()?;
            let element_value = element
                .attribute("elementValue")
                .map(|s| s.parse().context("parse 'elementValue'"))
                .transpose()?;
            let elem_fire = element
                .attribute("elemFire")
                .map(|s| {
                    if s == "TURE" {
                        Ok(1)
                    } else {
                        s.parse().context("parse 'elemFire'")
                    }
                })
                .transpose()?;
            let elem_ice = element
                .attribute("elemIce")
                .map(|s| {
                    if s == "TURE" {
                        Ok(1)
                    } else {
                        s.parse().context("parse 'elemIce'")
                    }
                })
                .transpose()?;
            let elem_thunder = element
                .attribute("elemThunder")
                .map(|s| {
                    if s == "TURE" {
                        Ok(1)
                    } else {
                        s.parse().context("parse 'elemThunder'")
                    }
                })
                .transpose()?;
            let elem_air = element
                .attribute("elemAir")
                .map(|s| {
                    if s == "TURE" {
                        Ok(1)
                    } else {
                        s.parse().context("parse 'elemAir'")
                    }
                })
                .transpose()?;
            let hp = element.attribute("hp").map(|s| s.parse()).transpose()?;
            let atk = element.attribute("atk").map(|s| s.parse()).transpose()?;
            let def = element.attribute("def").map(|s| s.parse()).transpose()?;
            let spd = element.attribute("spd").map(|s| s.parse()).transpose()?;
            let w_hp = element.attribute("w_hp").map(|s| s.parse()).transpose()?;
            let w_mp = element.attribute("w_mp").map(|s| s.parse()).transpose()?;
            let w_atk = element.attribute("w_atk").map(|s| s.parse()).transpose()?;
            let w_def = element.attribute("w_def").map(|s| s.parse()).transpose()?;
            let w_spd = element.attribute("w_spd").map(|s| s.parse()).transpose()?;

            // optional list properties
            // TODO: ensure order is correct
            let pc = element
                .attributes()
                .filter(|a| a.name().starts_with("pc_"))
                .flat_map(|a| a.value().parse().context("parse 'pc_*'"))
                .collect::<Vec<_>>();
            let dlc = element
                .attributes()
                .filter(|a| a.name().starts_with("dlc_"))
                .map(|a| a.value().to_string())
                .collect::<Vec<_>>();
            let cat = element
                .attributes()
                .filter(|a| a.name().starts_with("cat_"))
                .map(|a| a.value().to_string())
                .collect::<Vec<_>>();

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
