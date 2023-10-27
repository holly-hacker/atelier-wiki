use crate::utils::{self, ElementReader, PakIndex};

/// `itemData` from `\Saves\item\itemData.xml`
pub struct ItemData {
    pub name: String,
    pub tag: String,
    pub image_no: usize,
    pub cost: i32,
    pub use_type: String,
    pub base: String,
    pub level: usize,
    pub shape_type: String,
    pub base_size: usize,
    pub quality_name: String,
    pub size_name: String,
    pub color: String,
    pub category: Vec<String>,
    pub reasonable: Vec<usize>,

    pub strengthening: Option<usize>,
    pub hp: Option<usize>,
    pub mp: Option<usize>,
    pub lp: Option<usize>,
    pub atk: Option<usize>,
    pub def: Option<usize>,
    pub spd: Option<usize>,
    pub damage_min: Option<usize>,
    pub damage_max: Option<usize>,

    pub doll_tendency_cute: Option<i32>,
    pub doll_tendency_wise: Option<i32>,
    pub doll_tendency_brave: Option<i32>,
    pub doll_tendency_fool: Option<i32>,

    /// Player character, relates to which player cannot use this item.
    pub pc: Vec<Option<i32>>,
}

impl ItemData {
    pub fn read(pak_index: &mut PakIndex) -> anyhow::Result<Vec<Self>> {
        utils::read_xml(pak_index, r"\Saves\item\itemData.xml", |d| {
            Self::read_from_doc(d)
        })
    }

    fn read_from_doc(document: roxmltree::Document) -> anyhow::Result<Vec<Self>> {
        let mut ret = vec![];

        let elements = document
            .root_element()
            .descendants()
            .filter(|n| n.tag_name().name() == "itemData");

        for element in elements {
            let reader = ElementReader(&element);

            let name = reader.read("ItemName")?;
            let tag = reader.read("itemTag")?;
            let image_no = reader.read("ImageNo")?;
            let cost = reader.read("cost")?;
            let use_type = reader.read("UseType")?;
            let base = reader.read("base")?;
            let level = reader.read("level")?;
            let shape_type = reader.read("shapeType")?;
            let base_size = reader.read("baseSize")?;
            let quality_name = reader.read("qualityName")?;
            let size_name = reader.read("sizeName")?;
            let color = reader.read("color")?;
            let category = reader.read_list("category_*")?;
            let reasonable = reader.read_list("reasonable_*")?;
            let strengthening = reader.read_opt("strengthening")?;
            let hp = reader.read_opt("HP")?;
            let mp = reader.read_opt("MP")?;
            let lp = reader.read_opt("LP")?;
            let atk = reader.read_opt("ATK")?;
            let def = reader.read_opt("DEF")?;
            let spd = reader.read_opt("SPD")?;
            let damage_min = reader.read_opt("DamageMin")?;
            let damage_max = reader.read_opt("DamageMax")?;
            let doll_tendency_cute = reader.read_opt("DOLL_TENDENCY_CUTE")?;
            let doll_tendency_wise = reader.read_opt("DOLL_TENDENCY_WISE")?;
            let doll_tendency_brave = reader.read_opt("DOLL_TENDENCY_BRAVE")?;
            let doll_tendency_fool = reader.read_opt("DOLL_TENDENCY_FOOL")?;
            let pc = reader.read_sparse_list("PC*")?;

            ret.push(Self {
                name,
                tag,
                image_no,
                cost,
                use_type,
                base,
                level,
                shape_type,
                base_size,
                quality_name,
                size_name,
                color,
                category,
                reasonable,
                strengthening,
                hp,
                mp,
                lp,
                atk,
                def,
                spd,
                damage_min,
                damage_max,
                doll_tendency_cute,
                doll_tendency_wise,
                doll_tendency_brave,
                doll_tendency_fool,
                pc,
            })
        }

        Ok(ret)
    }
}
