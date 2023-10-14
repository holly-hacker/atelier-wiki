use crate::{
    extract::util::{self, ElementReader},
    utils::PakIndex,
};

/// `point` from `\Saves\Friend\point.xml`
pub struct Point {
    pub friend_tag: String,
    pub chara_tag: String,
    pub default_pts: usize,
    pub limit_default: usize,
    pub limit_pts: Vec<usize>,
    pub pass_ev: Vec<String>,
}

impl Point {
    pub fn read(pak_index: &mut PakIndex) -> anyhow::Result<Vec<Self>> {
        util::read_xml(pak_index, r"\Saves\Friend\point.xml", |d| {
            Self::read_from_doc(d)
        })
    }

    fn read_from_doc(document: roxmltree::Document) -> anyhow::Result<Vec<Self>> {
        let mut ret = vec![];

        let elements = document
            .root_element()
            .descendants()
            .filter(|n| n.tag_name().name() == "point");

        for element in elements {
            let reader = ElementReader(&element);

            let friend_tag = reader.read("friend_tag")?;
            let chara_tag = reader.read("chara_tag")?;
            let default_pts = reader.read("default_pts")?;
            let limit_default = reader.read("limit_default")?;
            let limit_pts = reader.read_flattened_sparse_list("limit_pts*")?;
            let pass_ev = reader.read_flattened_sparse_list("pass_ev*")?;

            ret.push(Self {
                friend_tag,
                chara_tag,
                default_pts,
                limit_default,
                limit_pts,
                pass_ev,
            })
        }

        Ok(ret)
    }
}
