use crate::{
    extract::util::{self, ElementReader},
    utils::PakIndex,
};

/// `rumor` from `\Saves\rumor\rumor.xml`
pub struct Rumor {
    pub name: String,
    pub type_: String,
    pub group: usize,
    pub ev_tag: Option<String>,
    pub fieldmap_tag: Option<String>,
    pub monster_tag: Option<String>,
    pub item_tag: Option<String>,
    pub image_no: usize,
    pub icon_image_no: usize,
    pub cost: usize,
    pub count: Option<usize>,
    pub deadline: usize,
    pub interval: usize,
    pub redo: Option<i32>,
    pub ev_begin: Option<String>,
    pub ev_end: Option<String>,
    pub cond_quest_group: Option<String>,
    pub priority: usize,
    pub probability: usize,
    pub register: Option<i32>,
    pub category: String,
    pub introduction: String,
    pub text: String,
}

impl Rumor {
    pub fn read(pak_index: &mut PakIndex) -> anyhow::Result<Vec<Self>> {
        util::read_xml(pak_index, r"\Saves\rumor\rumor.xml", |d| {
            Self::read_from_doc(d)
        })
    }

    fn read_from_doc(document: roxmltree::Document) -> anyhow::Result<Vec<Self>> {
        let mut ret = vec![];

        let elements = document
            .root_element()
            .descendants()
            .filter(|n| n.tag_name().name() == "rumor");

        for element in elements {
            let reader = ElementReader(&element);

            let name = reader.read("name")?;
            let type_ = reader.read("type")?;
            let group = reader.read("group")?;
            let ev_tag = reader.read_opt("ev_tag")?;
            let fieldmap_tag = reader.read_opt("fieldmap_tag")?;
            let monster_tag = reader.read_opt("monster_tag")?;
            let item_tag = reader.read_opt("item_tag")?;
            let image_no = reader.read("image_no")?;
            let icon_image_no = reader.read("icon_image_no")?;
            let cost = reader.read("cost")?;
            let count = reader.read_opt("count")?;
            let deadline = reader.read("deadline")?;
            let interval = reader.read("interval")?;
            let redo = reader.read_opt("redo")?;
            let ev_begin = reader.read_opt("ev_begin")?;
            let ev_end = reader.read_opt("ev_end")?;
            let cond_quest_group = reader.read_opt("cond_quest_group")?;
            let priority = reader.read("priority")?;
            let probability = reader.read("probability")?;
            let register = reader.read_opt("register")?;
            let category = reader.read("category")?;
            let introduction = reader.read("introduction")?;
            let text = reader.read("text")?;

            ret.push(Self {
                name,
                type_,
                group,
                ev_tag,
                fieldmap_tag,
                monster_tag,
                item_tag,
                image_no,
                icon_image_no,
                cost,
                count,
                deadline,
                interval,
                redo,
                ev_begin,
                ev_end,
                cond_quest_group,
                priority,
                probability,
                register,
                category,
                introduction,
                text,
            })
        }

        Ok(ret)
    }
}
