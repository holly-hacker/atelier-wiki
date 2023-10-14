use anyhow::Context;
use serde::Serialize;
use tracing::debug;
use typescript_type_def::TypeDef;

use crate::utils::PakIndex;

mod rumor;

#[derive(Serialize, TypeDef)]
pub struct Rumor {
    /// The name of the rumor as shown in the in-game list.
    pub name: String,
    pub r#type: String,
    pub group: usize,
    pub ev_tag: Option<String>,
    pub fieldmap_tag: Option<String>,
    pub monster_tag: Option<String>,
    pub item_tag: Option<String>,
    pub image_no: usize,
    pub icon_image_no: usize,
    /// The cost of the rumor.
    pub cost: usize,
    pub count: Option<usize>,
    pub deadline: usize,
    pub interval: usize,
    /// Whether this rumor can be bought again after it has been completed.
    pub redo: bool,
    pub ev_begin: Option<String>,
    pub ev_end: Option<String>,
    pub cond_quest_group: Option<String>,
    pub priority: usize,
    pub probability: usize,
    pub register: bool,
    /// The title at the top of the rumor preview.
    pub category: String,
    /// The text that is shown in the rumor preview.
    pub introduction: String,
    /// The text spoken by the rumor seller.
    pub text: String,
}

impl Rumor {
    pub fn read(pak_index: &mut PakIndex) -> anyhow::Result<Vec<Self>> {
        debug!("Reading present_ex data");
        let rumors = rumor::Rumor::read(pak_index).context("read rumor")?;

        Ok(rumors
            .into_iter()
            .map(|r| {
                debug_assert!(matches!(r.probability, 0..=100));
                Rumor {
                    name: r.name,
                    r#type: r.type_,
                    group: r.group,
                    ev_tag: r.ev_tag,
                    fieldmap_tag: r.fieldmap_tag,
                    monster_tag: r.monster_tag,
                    item_tag: r.item_tag,
                    image_no: r.image_no,
                    icon_image_no: r.icon_image_no,
                    cost: r.cost,
                    count: r.count,
                    deadline: r.deadline,
                    interval: r.interval,
                    redo: r.redo.is_some(),
                    ev_begin: r.ev_begin,
                    ev_end: r.ev_end,
                    cond_quest_group: r.cond_quest_group,
                    priority: r.priority,
                    probability: r.probability,
                    register: r.register.is_some(),
                    category: r.category,
                    introduction: r.introduction,
                    text: r.text,
                }
            })
            .collect())
    }
}
