use std::collections::BTreeMap;

use anyhow::Context;
use serde::Serialize;
use tracing::debug;
use typescript_type_def::TypeDef;

use crate::utils::PakIndex;

mod point;
mod present_base;
mod present_ex;

#[derive(Serialize, TypeDef)]
pub struct PresentInfo {
    /// Present info for each friend
    pub friend_present_info: BTreeMap<String, FriendPresentInfo>,
}

/// Present info for a specific friend
#[derive(Serialize, TypeDef, Default)]
pub struct FriendPresentInfo {
    /// Gift items and their points
    pub item_points: BTreeMap<String, f32>,
    /// Base points for each item type
    pub base_points: PresentBasePoints,

    /// The default friendship points for this friend
    pub default_points: usize,
    /// The default friendship point limit for this friend
    pub default_limit: usize,

    /// Unlockable friendship point limits with their required events
    pub unlockable_limits: Vec<(usize, String)>,
}

#[derive(Serialize, TypeDef, Default)]
pub struct PresentBasePoints {
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

impl PresentInfo {
    pub fn read(pak_index: &mut PakIndex) -> anyhow::Result<Self> {
        debug!("Reading present_ex data");
        let present_ex = present_ex::PresentEx::read(pak_index).context("read present_ex")?;
        debug!("Reading present_base data");
        let present_base =
            present_base::PresentBase::read(pak_index).context("read present_base")?;
        debug!("Reading point data");
        let points = point::Point::read(pak_index).context("read point")?;

        let mut friend_present_info = BTreeMap::<_, FriendPresentInfo>::new();

        present_ex.into_iter().for_each(|item| {
            let friend = friend_present_info.entry(item.friend_tag).or_default();
            friend.item_points.insert(item.item_tag, item.pts);
        });

        present_base.into_iter().for_each(|base| {
            let friend = friend_present_info.get_mut(&base.friend_tag).expect(
                "each present_base has a corresponding present_ex with the same friend_tag",
            );
            friend.base_points = PresentBasePoints {
                attack: base.attack,
                heal: base.heal,
                support: base.support,
                field: base.field,
                mix: base.mix,
                machine: base.machine,
                weapon: base.weapon,
                armor: base.armor,
                accessory: base.accessory,
                material: base.material,
            };
        });

        points.into_iter().for_each(|point| {
            let friend = friend_present_info
                .get_mut(&point.friend_tag)
                .expect("each point has a corresponding present_ex with the same friend_tag");

            friend.default_points = point.default_pts;
            friend.default_limit = point.limit_default;

            debug_assert!(
                (point.limit_pts.is_empty() && point.pass_ev.is_empty())
                    || (point.limit_pts.len() == point.pass_ev.len() + 1)
            );
            friend.unlockable_limits = point.limit_pts.into_iter().zip(point.pass_ev).collect();
        });

        Ok(Self {
            friend_present_info,
        })
    }
}
