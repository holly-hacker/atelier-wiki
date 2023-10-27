use crate::utils::{self, ElementReader, PakIndex};

/// `present_base` from `\Saves\Friend\present_base.xml`
pub struct PresentBase {
    pub friend_tag: String,
    pub attack: f32,
    pub heal: f32,
    pub support: f32,
    pub field: f32,
    pub mix: f32,
    pub machine: f32,
    pub weapon: f32,
    pub armor: f32,
    pub accessory: f32,
    pub material: f32,
}

impl PresentBase {
    pub fn read(pak_index: &mut PakIndex) -> anyhow::Result<Vec<Self>> {
        utils::read_xml(pak_index, r"\Saves\Friend\present_base.xml", |d| {
            Self::read_from_doc(d)
        })
    }

    fn read_from_doc(document: roxmltree::Document) -> anyhow::Result<Vec<Self>> {
        let mut ret = vec![];

        let elements = document
            .root_element()
            .descendants()
            .filter(|n| n.tag_name().name() == "present_base");

        for element in elements {
            let reader = ElementReader(&element);

            let friend_tag = reader.read("friend_tag")?;
            let attack = reader.read("attack")?;
            let heal = reader.read("heal")?;
            let support = reader.read("support")?;
            let field = reader.read("field")?;
            let mix = reader.read("mix")?;
            let machine = reader.read("machine")?;
            let weapon = reader.read("weapon")?;
            let armor = reader.read("armor")?;
            let accessory = reader.read("accessory")?;
            let material = reader.read("material")?;

            ret.push(Self {
                friend_tag,
                attack,
                heal,
                support,
                field,
                mix,
                machine,
                weapon,
                armor,
                accessory,
                material,
            })
        }

        Ok(ret)
    }
}
