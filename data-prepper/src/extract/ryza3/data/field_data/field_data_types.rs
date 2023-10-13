use super::*;
use serde::Serialize;
use typescript_type_def::TypeDef;

#[derive(Serialize, TypeDef)]
pub struct CutDownTree {
    #[serde(flatten)]
    pub data: GimmickData,
    /// Drop information when using rod.
    pub rod: (Option<String>, Option<usize>),
    /// Drop information when using sickle.
    pub sickle: (Option<String>, Option<usize>),
    /// Drop information when using axe.
    pub axe: (Option<String>, Option<usize>),
    /// Drop information when using hammer.
    pub hammer: (Option<String>, Option<usize>),
}

impl CutDownTree {
    pub fn from_gimmick(gimmick: gimmick::GimmickProperty) -> anyhow::Result<Self> {
        if gimmick.parameters.len() != 8 {
            anyhow::bail!("CutDownTree gimmick should have 8 parameters");
        }

        Ok(Self {
            data: GimmickData::from_gimmick(&gimmick)?,
            rod: (
                gimmick.parameters[0].clone(),
                gimmick.parameters[4]
                    .as_ref()
                    .map(|p| p.parse())
                    .transpose()
                    .context("parse arg 4")?,
            ),
            sickle: (
                gimmick.parameters[1].clone(),
                gimmick.parameters[5]
                    .as_ref()
                    .map(|p| p.parse())
                    .transpose()
                    .context("parse arg 5")?,
            ),
            axe: (
                gimmick.parameters[2].clone(),
                gimmick.parameters[6]
                    .as_ref()
                    .map(|p| p.parse())
                    .transpose()
                    .context("parse arg 6")?,
            ),
            hammer: (
                gimmick.parameters[3].clone(),
                gimmick.parameters[7]
                    .as_ref()
                    .map(|p| p.parse())
                    .transpose()
                    .context("parse arg 7")?,
            ),
        })
    }
}

#[derive(Serialize, TypeDef)]
pub struct EnemyRandomSpawner {
    #[serde(flatten)]
    pub data: GimmickData,

    pub min: usize,
    pub max: usize,
    pub symbol_group_1: Option<String>,
    pub symbol_group_2: Option<String>,
    pub symbol_group_3: Option<String>,
    pub symbol_group_4: Option<String>,
    pub symbol_group_5: Option<String>,

    pub monster_count: Option<usize>,
    pub monster: Option<String>,
}

impl EnemyRandomSpawner {
    pub fn from_gimmick(gimmick: gimmick::GimmickProperty) -> anyhow::Result<Self> {
        if gimmick.parameters.len() != 10 {
            anyhow::bail!("CutDownTree gimmick should have 10 parameters");
        }

        debug_assert!(
            gimmick.parameters[9].is_none(),
            "arg 9 should be None, was {:?}",
            gimmick.parameters[9]
        );

        Ok(Self {
            data: GimmickData::from_gimmick(&gimmick)?,
            min: gimmick.parameters[0]
                .as_ref()
                .context("arg 0 must be present")?
                .parse()
                .context("parse arg 0")?,
            max: gimmick.parameters[1]
                .as_ref()
                .context("arg 1 must be present")?
                .parse()
                .context("parse arg 1")?,
            symbol_group_1: gimmick.parameters[2].clone(),
            symbol_group_2: gimmick.parameters[3].clone(),
            symbol_group_3: gimmick.parameters[4].clone(),
            symbol_group_4: gimmick.parameters[5].clone(),
            symbol_group_5: gimmick.parameters[6].clone(),

            monster_count: gimmick.parameters[7]
                .as_ref()
                .map(|x| x.parse())
                .transpose()
                .with_context(|| format!("parse index 7: {:?}", gimmick.parameters[7]))?,
            monster: gimmick.parameters[8].clone(),
        })
    }
}

#[derive(Serialize, TypeDef)]
pub struct InstantEnemySpawner {
    #[serde(flatten)]
    pub data: GimmickData,

    pub symbol_group: String,
    pub min_count: Option<usize>,
}

impl InstantEnemySpawner {
    pub fn from_gimmick(gimmick: gimmick::GimmickProperty) -> anyhow::Result<Self> {
        if gimmick.parameters.len() != 3 {
            anyhow::bail!("InstantEnemySpawner gimmick should have 3 parameters");
        }

        debug_assert!(
            gimmick.parameters[2].is_none(),
            "arg 2 should be None, was {:?}",
            gimmick.parameters[2]
        );

        Ok(Self {
            data: GimmickData::from_gimmick(&gimmick)?,

            symbol_group: gimmick.parameters[0]
                .clone()
                .with_context(|| "arg 0 must be present")?,
            min_count: gimmick.parameters[1]
                .as_ref()
                .map(|x| x.parse())
                .transpose()
                .with_context(|| format!("parse index 1: {:?}", gimmick.parameters[1]))?,
        })
    }
}
