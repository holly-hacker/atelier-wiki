use crate::utils::{self, ElementReader, PakIndex};

#[derive(Debug)]
pub struct QuestPrize {
    pub prize_tag: String,

    pub prize_num: Vec<Option<u32>>,
    pub item_tag: Vec<Option<String>>,
    pub pot_0: Vec<Option<String>>,
    pub pot_1: Vec<Option<String>>,
    pub pot_2: Vec<Option<String>>,
    pub eff_0: Vec<Option<String>>,
    pub eff_1: Vec<Option<String>>,
    pub eff_2: Vec<Option<String>>,
    pub eff_3: Vec<Option<String>>,
    pub pot_grade_min: Vec<Option<u32>>,
    pub pot_grade_max: Vec<Option<u32>>,
    pub pot_lv_min: Vec<Option<u32>>,
    pub pot_lv_max: Vec<Option<u32>>,
    pub pot_lv_min_0: Vec<Option<u32>>,
    pub pot_lv_max_0: Vec<Option<u32>>,
    pub pot_lv_min_1: Vec<Option<u32>>,
    pub pot_lv_max_1: Vec<Option<u32>>,
    pub pot_num_min: Vec<Option<u32>>,
    pub pot_num_max: Vec<Option<u32>>,
    pub quality_min: Vec<Option<u32>>,
    pub quality_max: Vec<Option<u32>>,

    pub prize_money: Vec<Option<u32>>,
    pub prize_gold_coin: Vec<Option<u32>>,
    pub prize_sp: Vec<Option<u32>>,
    pub prize_memories_tag: Vec<Option<String>>,

    pub unknown_flag: Vec<Option<i32>>,
}

impl QuestPrize {
    pub fn read(pak_index: &mut PakIndex) -> anyhow::Result<Vec<Self>> {
        utils::read_xml(pak_index, r"\saves\quest\normal\questprize.xml", |d| {
            Self::read_from_doc(d)
        })
    }

    fn read_from_doc(document: roxmltree::Document) -> anyhow::Result<Vec<Self>> {
        let mut ret = vec![];

        let elements = document
            .root_element()
            .descendants()
            .filter(|n| n.tag_name().name() == "QuestPrize");

        for element in elements {
            let reader = ElementReader(&element);

            let prize_tag = reader.read("PrizeTag")?;
            let item_tag = reader.read_sparse_list("ItemTag_*")?;
            let pot_0 = reader.read_sparse_list("Pot00_*")?;
            let pot_1 = reader.read_sparse_list("Pot01_*")?;
            let pot_2 = reader.read_sparse_list("Pot02_*")?;
            let eff_0 = reader.read_sparse_list("Eff00_*")?;
            let eff_1 = reader.read_sparse_list("Eff01_*")?;
            let eff_2 = reader.read_sparse_list("Eff02_*")?;
            let eff_3 = reader.read_sparse_list("Eff03_*")?;
            let pot_grade_min = reader.read_sparse_list("PotGradeMin_*")?;
            let pot_grade_max = reader.read_sparse_list("PotGradeMax_*")?;
            let pot_lv_min = reader.read_sparse_list("PotLvMin_*")?;
            let pot_lv_max = reader.read_sparse_list("PotLvMax_*")?;
            let pot_lv_min_0 = reader.read_sparse_list("PotLvMin00_*")?;
            let pot_lv_max_0 = reader.read_sparse_list("PotLvMax00_*")?;
            let pot_lv_min_1 = reader.read_sparse_list("PotLvMin01_*")?;
            let pot_lv_max_1 = reader.read_sparse_list("PotLvMax01_*")?;
            let pot_num_min = reader.read_sparse_list("PotNumMin_*")?;
            let pot_num_max = reader.read_sparse_list("PotNumMax_*")?;
            let prize_gold_coin = reader.read_sparse_list("PrizeGoldCoin_*")?;
            let prize_memories_tag = reader.read_sparse_list("PrizeMemoriesTag_*")?;
            let prize_money = reader.read_sparse_list("PrizeMoney_*")?;
            let prize_num = reader.read_sparse_list("PrizeNum_*")?;
            let prize_sp = reader.read_sparse_list("PrizeSP_*")?;
            let quality_min = reader.read_sparse_list("Quality_Min*")?;
            let quality_max = reader.read_sparse_list("Quality_Max*")?;
            let unknown_flag = reader.read_sparse_list("UnknownFlag_*")?;

            ret.push(Self {
                prize_tag,
                item_tag,
                pot_0,
                pot_1,
                pot_2,
                eff_0,
                eff_1,
                eff_2,
                eff_3,
                pot_grade_min,
                pot_grade_max,
                pot_lv_min,
                pot_lv_max,
                pot_lv_min_0,
                pot_lv_max_0,
                pot_lv_min_1,
                pot_lv_max_1,
                pot_num_min,
                pot_num_max,
                prize_gold_coin,
                prize_memories_tag,
                prize_money,
                prize_num,
                prize_sp,
                quality_min,
                quality_max,
                unknown_flag,
            })
        }
        debug_assert!(!ret.is_empty());

        Ok(ret)
    }
}
