use std::collections::HashMap;

use anyhow::{bail, Context};

use crate::{
    extract::data::util::{self, ElementReader},
    utils::PakIndex,
};

/// A hash map of `tag` to `ExtendedFieldData`.
pub struct ExtendedFieldData(pub HashMap<String, Vec<Field>>);

pub struct Field {
    pub rings: Vec<Option<Ring>>,
}

pub struct Ring {
    /// The element.
    ///
    /// 0: Fire
    /// 1: Ice
    /// 2: Bolt
    /// 3: Wind
    pub elem: Option<u32>,
    /// The effect type of the ring.
    ///
    /// See `str_mix_feature_description` for more info.
    pub r#type: Option<u32>,
    /// The explicit material to use. Mutually exclusive with `restrict`.
    pub ex_material: Option<String>,
    /// Whether this is a core loop.
    pub is_essential: bool,
    /// The pre-defined item or category to use, as defined in `itemrecipedata.xml`. Mutually
    /// exclusive with `ex_material`.
    pub restrict: Option<u32>,
    /// 0 if `None`
    pub x: Option<i32>,
    /// 0 if `None`
    pub y: Option<i32>,

    pub child: Option<RingChild>,
    /// The connection to the previous ring it is connected to, if any.
    pub connect: Option<RingConnect>,
    pub param: Option<RingParam>,
}

pub struct RingChild {
    /// The indices of the connected rings that depend on this one. Clockwise order, starting from
    /// the top and rotating by 60 degrees.
    pub indices: String,
}

pub struct RingConnect {
    /// The index of the ring it is connected to.
    pub idx: Option<u32>,
    /// The element value required to unlock this ring.
    pub val: Option<u32>,
    /// The element required to unlock this ring.
    pub elem: Option<u32>,
    /// The quality required to unlock this ring, if any.
    pub qual: Option<u32>,
    /// The direction it is unlocked from.
    pub dir: Option<u32>,
}

pub struct RingParam {
    /// The value of the effect
    pub v: Vec<Option<String>>,
    /// The element values required for each tier.
    pub e: Vec<Option<u32>>,
    /// Whether this effect is hidden?
    pub n: Vec<bool>,
}

impl ExtendedFieldData {
    pub fn read(pak_index: &mut PakIndex) -> anyhow::Result<Self> {
        util::read_xml(pak_index, r"\saves\mix\mixfielddata.xml", |d| {
            Self::read_from_doc(d)
        })
    }

    fn read_from_doc(document: roxmltree::Document) -> anyhow::Result<Self> {
        let mut ret = HashMap::new();

        let field_data_elements = document
            .root_element()
            .descendants()
            .filter(|n| n.tag_name().name() == "ExtendFieldData");

        for field_data_element in field_data_elements {
            let tag: String = ElementReader(&field_data_element).read("tag")?;

            let field_elements = field_data_element
                .descendants()
                .filter(|n| n.tag_name().name() == "Field");

            let mut fields = vec![];
            for field_element in field_elements {
                let field = Field::read_from_reader(&field_element)
                    .with_context(|| format!("read Field tag {}", tag))?;

                fields.push(field);
            }

            let prev_value = ret.insert(tag, fields);
            if let Some(_prev_value) = prev_value {
                let tag = ElementReader(&field_data_element).read::<String>("tag")?;
                bail!("duplicate tag {}", tag);
            }
        }

        Ok(Self(ret))
    }
}

impl Field {
    pub fn read_from_reader(node: &roxmltree::Node<'_, '_>) -> anyhow::Result<Self> {
        let ring_elements = node
            .children()
            .filter(|n| n.tag_name().name() == "Ring")
            .collect::<Vec<_>>();

        let mut rings = vec![];
        for (ring_idx, ring_element) in ring_elements.iter().enumerate() {
            let ring = if ring_element.attributes().len() != 0 {
                Some(
                    Ring::read_from_reader(ring_element)
                        .with_context(|| format!("read Ring index {ring_idx}"))?,
                )
            } else {
                // there are some <Ring/> elements without attributes, assume they are empty
                None
            };
            rings.push(ring);
        }

        Ok(Self { rings })
    }
}

impl Ring {
    fn read_from_reader(node: &roxmltree::Node<'_, '_>) -> anyhow::Result<Self> {
        let reader = ElementReader(node);
        let elem = reader.read_opt("elem")?;
        let r#type = reader.read_opt("type")?;
        let ex_material = reader.read_opt("ex_material")?;
        let is_essential = reader.is_present("is_essential");
        let restrict = reader.read_opt("restrict")?;
        let x = reader.read_opt("x")?;
        let y = reader.read_opt("y")?;

        let child = node
            .children()
            .find(|n| n.tag_name().name() == "Child")
            .map(RingChild::read_from_reader)
            .transpose()
            .context("read Ring.Child")?;
        let connect = node
            .children()
            .find(|n| n.tag_name().name() == "Connect")
            .map(RingConnect::read_from_reader)
            .transpose()
            .context("read Ring.Connect")?;
        let param = node
            .children()
            .find(|n| n.tag_name().name() == "Param")
            .map(RingParam::read_from_reader)
            .transpose()
            .context("read Ring.Param")?;

        Ok(Self {
            elem,
            r#type,
            ex_material,
            is_essential,
            restrict,
            x,
            y,
            child,
            connect,
            param,
        })
    }
}

impl RingChild {
    fn read_from_reader(node: roxmltree::Node<'_, '_>) -> anyhow::Result<Self> {
        let reader = ElementReader(&node);

        let indices = reader.read("indexes")?;

        Ok(Self { indices })
    }
}

impl RingConnect {
    fn read_from_reader(node: roxmltree::Node<'_, '_>) -> anyhow::Result<Self> {
        let reader = ElementReader(&node);

        let idx = reader.read_opt("idx")?;
        let val = reader.read_opt("val")?;
        let elem = reader.read_opt("elem")?;
        let qual = reader.read_opt("qual")?;
        let dir = reader.read_opt("dir")?;

        Ok(Self {
            idx,
            val,
            elem,
            qual,
            dir,
        })
    }
}

impl RingParam {
    fn read_from_reader(node: roxmltree::Node<'_, '_>) -> anyhow::Result<Self> {
        let reader = ElementReader(&node);

        let v = reader.read_sparse_list("v*")?;
        let e = reader.read_sparse_list("e*")?;
        let n = reader
            .read_sparse_list::<String>("n*")?
            .into_iter()
            .map(|opt_s| opt_s.is_some())
            .collect();

        Ok(Self { v, e, n })
    }
}
