use std::collections::BTreeMap;

use anyhow::Context;
use tracing::debug;

use crate::{
    extract::ryza3::data::util::{self, ElementReader},
    utils::PakIndex,
};

#[derive(Debug)]
pub struct GimmickProperty {
    pub fix_id: usize,
    pub gimmick_type_tag: String,
    pub form_tag: String,
    pub rotation_y: f64,
    pub icon_rotation_y: f64,
    pub rate: usize,
    pub keep_weather_flag: bool,
    pub save_flag: bool,
    pub need_place_x64: Option<bool>,
    pub need_place_ps4: Option<bool>,
    pub need_place_nx: Option<bool>,
    pub form_param: String,
    pub position: String,
    pub model: Option<String>,

    pub parameters: Vec<Option<String>>,
    pub unique_params: Vec<BTreeMap<String, String>>, // TODO: use hashmaps in file
}

impl GimmickProperty {
    pub fn read(pak_index: &mut PakIndex) -> anyhow::Result<BTreeMap<String, Vec<Self>>> {
        const PREFIX: &str = r"\saves\fieldmap\gimmick\gimmick_fieldmap_";

        let entries = pak_index
            .iter_entries()
            .filter(|e| e.get_file_name().starts_with(PREFIX))
            .map(|e| e.into_owned())
            .collect::<Vec<_>>();
        debug!("Found {} gimmick files", entries.len());

        let ret = entries
            .into_iter()
            .map(|e| {
                let e = e.as_ref();
                let file_name = e.get_file_name();
                let stripped = file_name.trim_start_matches(PREFIX);
                debug_assert!(stripped.ends_with(".xml"));

                let parsed = util::read_xml(pak_index, file_name, Self::read_from_doc)
                    .with_context(|| format!("parse gimmick doc '{file_name}'"))?;

                Ok((stripped.to_owned(), parsed))
            })
            .collect::<anyhow::Result<BTreeMap<String, Vec<Self>>>>()?;

        Ok(ret)
    }

    fn read_from_doc(document: roxmltree::Document) -> anyhow::Result<Vec<Self>> {
        let mut ret = vec![];

        let elements = document
            .root_element()
            .descendants()
            .filter(|n| n.tag_name().name() == "Property");

        for element in elements {
            let reader = ElementReader(&element);

            let fix_id = reader.read("fixID")?;
            let gimmick_type_tag = reader.read("gimmickTypeTag")?;
            let form_tag = reader.read("formTag")?;
            let rotation_y = reader.read("rotationY")?;
            let icon_rotation_y = reader.read("iconRatationY")?; // lit. "Ratation"
            let rate = reader.read("rate")?;
            let keep_weather_flag = reader.read("keepWeatherFlag")?;
            let save_flag = reader.read("saveFlag")?;
            let need_place_x64 = reader.read_opt("NeedPlaceX64")?;
            let need_place_ps4 = reader.read_opt("NeedPlacePS4")?;
            let need_place_nx = reader.read_opt("NeedPlaceNX")?;
            let form_param = reader.read("formParam")?;
            let position = reader.read("position")?;
            let model = reader.read_opt("model")?;

            let parameters = element
                .children()
                .filter(|n| n.tag_name().name() == "Parameter")
                .flat_map(|n| n.children().filter(|n| n.tag_name().name() == "Value"))
                .map(|n| n.text().map(|s| s.to_owned()))
                .collect::<Vec<_>>();

            let unique_params = element
                .children()
                .filter(|n| n.tag_name().name() == "UniqueParam")
                .flat_map(|n| n.children().filter(|n| n.tag_name().name() == "Value"))
                .map(|n| {
                    n.attributes()
                        .map(|a| (a.name().to_owned(), a.value().to_owned()))
                        .collect::<BTreeMap<_, _>>()
                })
                .collect::<Vec<_>>();

            let property = Self {
                fix_id,
                gimmick_type_tag,
                form_tag,
                rotation_y,
                icon_rotation_y,
                rate,
                keep_weather_flag,
                save_flag,
                need_place_x64,
                need_place_ps4,
                need_place_nx,
                form_param,
                position,
                model,
                parameters,
                unique_params,
            };

            ret.push(property);
        }

        Ok(ret)
    }
}
