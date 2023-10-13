//! Parsing of item\itemrecipedata.xml

use anyhow::{bail, Context};

use crate::{
    extract::util::{self, ElementReader},
    utils::PakIndex,
};

pub enum ItemRecipeData {
    Header(RecipeHeader),
    Ingredient(IngredientData),
}

pub struct RecipeHeader {
    pub item_tag: String,
    pub sort: Option<i32>,
    pub make_num: u32,
    pub hour: u32,
    pub need_num: u32,
    pub recipe_category: String,

    pub first_ingredient: IngredientData,
}

pub struct IngredientData {
    pub mat_tag: String,
    pub is_category: bool,
    pub add_eff: Vec<String>,
    pub mass_effect: Option<String>,
    pub rl_cond_sk_elem: Option<String>,
    pub rl_cond_sk_motif: Option<String>,
    pub rl_cond_sk_rarity: Option<String>,
    pub rl_reward: Option<String>,
}

impl RecipeHeader {
    fn read_from_reader(reader: &ElementReader) -> anyhow::Result<Self> {
        debug_assert!(reader.is_present("HasData"));

        let ret = Self {
            item_tag: reader.read("ItemTag")?,
            sort: reader.read_opt("sort")?,
            make_num: reader.read("MakeNum")?,
            hour: reader.read("Hour")?,
            need_num: reader.read("NeedNum")?,

            recipe_category: reader.read("RecipeCategory")?,
            first_ingredient: IngredientData::read_from_reader(reader)
                .context("read ingredient inside header")?,
        };

        Ok(ret)
    }
}

impl IngredientData {
    fn read_from_reader(reader: &ElementReader) -> anyhow::Result<Self> {
        let ret = Self {
            mat_tag: reader.read("MatTag")?,
            is_category: reader.read("IsCategory")?,
            add_eff: reader.read_list("AddEff*")?,
            mass_effect: reader.read_opt("MassEffect")?,
            rl_cond_sk_elem: reader.read_opt("rl_cond_sk_elem")?,
            rl_cond_sk_motif: reader.read_opt("rl_cond_sk_motif")?,
            rl_cond_sk_rarity: reader.read_opt("rl_cond_sk_rarity")?,
            rl_reward: reader.read_opt("rl_reward")?,
        };

        let requirement_count = [
            ret.rl_cond_sk_elem.is_some(),
            ret.rl_cond_sk_motif.is_some(),
            ret.rl_cond_sk_rarity.is_some(),
        ]
        .into_iter()
        .filter(|x| *x)
        .count();

        if requirement_count > 1 {
            bail!("at most one of rl_cond_sk_elem, rl_cond_sk_motif, rl_cond_sk_rarity may be present");
        }

        Ok(ret)
    }
}

impl ItemRecipeData {
    pub fn read(pak_index: &mut PakIndex) -> anyhow::Result<Vec<Self>> {
        util::read_xml(pak_index, r"\saves\item\itemrecipedata.xml", |d| {
            Self::read_from_doc(d)
        })
    }

    fn read_from_doc(document: roxmltree::Document) -> anyhow::Result<Vec<Self>> {
        let mut ret = vec![];

        let elements = document
            .root_element()
            .descendants()
            .filter(|n| n.tag_name().name() == "itemRecipeData");

        for (idx, element) in elements.enumerate() {
            let reader = ElementReader(&element);

            if reader.is_present("HasData") {
                debug_assert!(reader.is_present("ItemTag"));

                // item is a header
                let header = RecipeHeader::read_from_reader(&reader)
                    .with_context(|| format!("read header on index {idx}"))?;
                ret.push(ItemRecipeData::Header(header));
            } else if reader.is_present("MatTag") {
                // item is an ingredient
                let ingredient = IngredientData::read_from_reader(&reader)
                    .with_context(|| format!("read ingredients on index {idx}"))?;
                ret.push(ItemRecipeData::Ingredient(ingredient));
            } else {
                // item is empty, these seem to exist for reserved item spots
            }
        }

        Ok(ret)
    }
}
