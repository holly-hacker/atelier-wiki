mod normal_quest;
mod quest_clear_cond;
mod quest_cond;
mod quest_prize;

use anyhow::Context;
use serde::Serialize;
use tracing::warn;
use typescript_type_def::TypeDef;

use crate::{ryza3::data::strings_table::StringsTable, utils::PakIndex};

#[derive(Serialize, TypeDef)]
pub struct NormalQuest {
    pub tag: Option<String>,
    pub image_no: u32,

    /// The title of the quest.
    pub title: Option<String>,
    /// The flavor text/description of the quest.
    pub flavor_text: Option<String>,
    /// The flavor text/description of the quest after it has been cleared.
    pub flavor_text_after: Option<String>,

    pub start_exec_ev: Option<String>,

    /// Event that must have been completed before this quest can be started.
    pub condition_event_tags: Vec<String>,
    /// Conditions that must be cleared before this quest can be completed.
    pub clear_cond_tag: Vec<NormalQuestClearCondition>,
    /// The prizes that are awarded for completing this quest.
    pub prizes: Vec<NormalQuestPrize>,
}

#[derive(Serialize, TypeDef)]
pub struct NormalQuestClearCondition {
    /// A potential achievement that should be completed.
    pub achievement_condition: Option<NormalQuestClearAchievement>,
    /// A potential event that should be completed.
    pub event_condition: Option<NormalQuestClearEvent>,
    /// A potential item that should be delivered.
    pub delivery_condition: Option<NormalQuestClearDelivery>,
    /// A potential enemy that should be defeated.
    pub battle_condition: Option<NormalQuestClearBattle>,

    /// A marked position on the map that should be visited.
    pub trace_pos: Option<(f32, f32, f32)>,
    /// The radius around the marked position on the map.
    pub trace_radius: Option<f32>,
    /// The field tag of the map that should be visited.
    pub trace_field_map_tag: Option<String>,
    /// A marked NPC that should be interacted with.
    pub trace_npc_tag: Option<String>,
    /// Whether the atelier should be visited.
    pub trace_atelier: bool,
}
#[derive(Serialize, TypeDef)]
pub struct NormalQuestClearAchievement {
    pub detail_text: String,
    pub clear_achievement_tag: String,
    pub display_progress: bool,
}
#[derive(Serialize, TypeDef)]
pub struct NormalQuestClearEvent {
    pub detail_text: String,
    pub clear_event_tag: String,
}
#[derive(Serialize, TypeDef)]
pub struct NormalQuestClearDelivery {
    pub detail_text: Option<String>,
    /// The tag of the item that should be delivered.
    pub item_tag: String,
    /// The amount of items that should be delivered.
    pub delivery_item_num: u32,
    pub quality_min: Option<u32>,
    pub quality_max: Option<u32>,
    /// The traits that the item should have.
    pub pot: Vec<String>,
    /// The effects that the item should have.
    pub eff: Vec<String>,
}
#[derive(Serialize, TypeDef)]
pub struct NormalQuestClearBattle {
    pub detail_text: Option<String>,
    /// The monster that should be defeated.
    pub target_name: String,
    /// The amount of monsters that should be defeated.
    pub target_num: u32,
    /// The specific symbol group that the monster must come from.
    pub symbol_group_tag: Option<String>,
}

#[derive(Serialize, TypeDef)]
pub struct NormalQuestPrize {
    pub is_unknown: bool,
    pub prize: NormalQuestPrizeType,
}

#[derive(Serialize, TypeDef)]
#[serde(tag = "type")]
pub enum NormalQuestPrizeType {
    Item {
        num: u32,

        item_tag: String,
        quality_min: u32,
        quality_max: u32,

        pots: Vec<String>,
        effects: Vec<String>,

        pot_num_min: u32,
        pot_num_max: u32,
        pot_grade_min: u32,
        pot_grade_max: u32,
        pot_lvl_min: u32,
        pot_lvl_max: u32,
    },
    Money {
        amount: u32,
    },
    GoldCoin {
        amount: u32,
    },
    SP {
        amount: u32,
    },
    Memories {
        tag: String,
    },
}

impl NormalQuest {
    pub fn read(pak_index: &mut PakIndex, strings: &StringsTable) -> anyhow::Result<Vec<Self>> {
        let normal_quests =
            normal_quest::NormalQuest::read(pak_index).context("read normal_quest")?;
        let quest_clear_conds =
            quest_clear_cond::QuestClearCond::read(pak_index).context("read quest_clear_cond")?;
        let quest_conds = quest_cond::QuestCond::read(pak_index).context("read quest_cond")?;
        let quest_prizes = quest_prize::QuestPrize::read(pak_index).context("read quest_prize")?;

        let quests = normal_quests
            .into_iter()
            .filter(|q| q.is_valid == Some(-1))
            .map(|q| {
                Ok(Self {
                    tag: q.quest_tag,
                    image_no: q.image_no.context("get image_no")?,
                    title: q
                        .quest_title
                        .and_then(|t| strings.id_lookup.get(&t).cloned()),
                    flavor_text: q
                        .quest_flavor
                        .and_then(|t| strings.id_lookup.get(&t).cloned()),
                    flavor_text_after: q
                        .quest_flavor_after
                        .and_then(|t| strings.id_lookup.get(&t).cloned()),
                    start_exec_ev: q.start_exec_ev,
                    condition_event_tags: q
                        .quest_cond_tag
                        .and_then(|cond_tag| {
                            quest_conds
                                .iter()
                                .find(|cond| cond.quest_cond_tag == cond_tag)
                                .map(|c| c.acceptable_cond_event.clone())
                        })
                        .unwrap_or_default(),
                    clear_cond_tag: q
                        .clear_cond_tag
                        .into_iter()
                        .map(|cond_tag| {
                            quest_clear_conds
                                .iter()
                                .find(|cond| cond.clear_cond_tag == cond_tag)
                                .unwrap()
                        })
                        .map(|clear_cond| map_clear_cond(clear_cond, strings))
                        .collect(),
                    prizes: q
                        .prize_tag
                        .and_then(|prize_tag| {
                            quest_prizes
                                .iter()
                                .find(|prize| prize.prize_tag == prize_tag)
                                .map(map_prize)
                        })
                        .unwrap_or_default(),
                })
            })
            .collect::<anyhow::Result<_>>()?;

        Ok(quests)
    }
}

fn map_clear_cond(
    clear_cond: &quest_clear_cond::QuestClearCond,
    strings: &StringsTable,
) -> NormalQuestClearCondition {
    if clear_cond.is_valid != Some(-1) {
        warn!(
            "Clear condition {} is referenced by a quest but does not have an isValid tag",
            clear_cond.clear_cond_tag
        );
    }

    let trace_pos = match (
        clear_cond.trace_pos_x,
        clear_cond.trace_pos_y,
        clear_cond.trace_pos_z,
    ) {
        (Some(x), Some(y), Some(z)) => Some((x, y, z)),
        (None, None, None) => None,
        _ => panic!("Invalid trace_pos"),
    };
    let trace_radius = clear_cond.trace_radius;
    let trace_npc_tag = clear_cond.trace_npc_tag.clone();
    let trace_field_map_tag = clear_cond.trace_field_map_tag.clone();
    let trace_atelier = clear_cond.trace_atelier == Some(-1);

    let achievement_condition =
        clear_cond
            .clear_achievement_tag
            .as_ref()
            .map(|clear_achievement_tag| NormalQuestClearAchievement {
                detail_text: strings
                    .id_lookup
                    .get(clear_cond.achievement_detail_text.as_ref().unwrap())
                    .unwrap()
                    .clone(),
                clear_achievement_tag: clear_achievement_tag.clone(),
                display_progress: clear_cond.disp_progress == Some(-1),
            });

    let event_condition =
        clear_cond
            .clear_event_tag
            .as_ref()
            .map(|clear_event_tag| NormalQuestClearEvent {
                detail_text: strings
                    .id_lookup
                    .get(clear_cond.event_detail_text.as_ref().unwrap())
                    .unwrap()
                    .clone(),
                clear_event_tag: clear_event_tag.clone(),
            });

    let delivery_condition =
        clear_cond
            .item_tag
            .as_ref()
            .map(|item_tag| NormalQuestClearDelivery {
                quality_min: clear_cond.quality_min,
                quality_max: clear_cond.quality_max,
                delivery_item_num: clear_cond.delivery_item_num.unwrap(),
                item_tag: item_tag.clone(),
                detail_text: clear_cond
                    .delivery_detail_text
                    .as_ref()
                    .map(|id| strings.id_lookup.get(id).unwrap().clone()),
                pot: clear_cond.pot.clone(),
                eff: clear_cond.eff.clone(),
            });

    let battle_condition =
        clear_cond
            .target_name
            .as_ref()
            .map(|target_name| NormalQuestClearBattle {
                detail_text: clear_cond
                    .btl_detail_text
                    .as_ref()
                    .map(|id| strings.id_lookup.get(id).unwrap().clone()),
                target_name: target_name.clone(),
                target_num: clear_cond.target_num.unwrap(),
                symbol_group_tag: clear_cond.symbol_group_tag.clone(),
            });

    NormalQuestClearCondition {
        trace_pos,
        trace_radius,
        trace_npc_tag,
        trace_field_map_tag,
        trace_atelier,
        achievement_condition,
        event_condition,
        delivery_condition,
        battle_condition,
    }
}

fn map_prize(quest_prize: &quest_prize::QuestPrize) -> Vec<NormalQuestPrize> {
    const MAX_PRIZES: usize = 2;

    (0..MAX_PRIZES)
        .filter_map(|i| {
            let is_unknown = quest_prize.unknown_flag.get(i).cloned() == Some(Some(-1));

            let prize_type = if let Some(Some(money)) = quest_prize.prize_money.get(i) {
                Some(NormalQuestPrizeType::Money { amount: *money })
            } else if let Some(Some(gold_coin)) = quest_prize.prize_gold_coin.get(i) {
                Some(NormalQuestPrizeType::GoldCoin { amount: *gold_coin })
            } else if let Some(Some(sp)) = quest_prize.prize_sp.get(i) {
                Some(NormalQuestPrizeType::SP { amount: *sp })
            } else if let Some(Some(memories_tag)) = quest_prize.prize_memories_tag.get(i) {
                Some(NormalQuestPrizeType::Memories {
                    tag: memories_tag.clone(),
                })
            } else if let Some(Some(item_tag)) = quest_prize.item_tag.get(i) {
                Some(NormalQuestPrizeType::Item {
                    num: quest_prize.prize_num.get(i).unwrap().unwrap(),
                    item_tag: item_tag.clone(),
                    quality_min: quest_prize.quality_min.get(i).cloned().flatten().unwrap(),
                    quality_max: quest_prize.quality_max.get(i).cloned().flatten().unwrap(),
                    pots: vec![
                        quest_prize.pot_0.get(i).cloned().flatten(),
                        quest_prize.pot_1.get(i).cloned().flatten(),
                        quest_prize.pot_2.get(i).cloned().flatten(),
                    ]
                    .into_iter()
                    .flatten()
                    .collect(),
                    effects: vec![
                        quest_prize.eff_0.get(i).cloned().flatten(),
                        quest_prize.eff_1.get(i).cloned().flatten(),
                        quest_prize.eff_2.get(i).cloned().flatten(),
                        quest_prize.eff_3.get(i).cloned().flatten(),
                    ]
                    .into_iter()
                    .flatten()
                    .collect(),
                    pot_num_min: quest_prize.pot_num_min.get(i).cloned().flatten().unwrap(),
                    pot_num_max: quest_prize.pot_num_max.get(i).cloned().flatten().unwrap(),
                    pot_grade_min: quest_prize.pot_grade_min.get(i).cloned().flatten().unwrap(),
                    pot_grade_max: quest_prize.pot_grade_max.get(i).cloned().flatten().unwrap(),
                    pot_lvl_min: quest_prize.pot_lv_min.get(i).cloned().flatten().unwrap(),
                    pot_lvl_max: quest_prize.pot_lv_max.get(i).cloned().flatten().unwrap(),
                })
            } else {
                None
            };

            prize_type.map(|prize| NormalQuestPrize { is_unknown, prize })
        })
        .collect()
}
