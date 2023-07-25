use std::{io::Read, str::FromStr};

use anyhow::Context;

use crate::extract::pak_index::PakIndex;

pub fn read_xml<T, F>(pak_index: &mut PakIndex, path: &str, parse_fn: F) -> anyhow::Result<T>
where
    F: FnOnce(roxmltree::Document) -> anyhow::Result<T>,
{
    let mut file = pak_index
        .get_file(path)
        .context("open xml file")?
        .context("xml file not found")?;

    // NOTE: I could read the actual size from the pak file
    let mut xml_string = String::new();
    file.read_to_string(&mut xml_string)
        .context("read strings file to string")?;
    let document = roxmltree::Document::parse(&xml_string).context("parse strings xml file")?;

    let result = parse_fn(document)?;

    Ok(result)
}

pub struct ElementReader<'x, 'a, 'b>(pub &'x roxmltree::Node<'a, 'b>);

impl<'x, 'a, 'b> ElementReader<'x, 'a, 'b> {
    pub fn read_parse<T>(&self, name: &str) -> anyhow::Result<T>
    where
        T: FromStr,
        <T as FromStr>::Err: std::error::Error + Send + Sync + 'static,
    {
        self.0
            .attribute(name)
            .with_context(|| format!("field '{name}' is required"))?
            .parse()
            .with_context(|| format!("parse '{name}'"))
    }

    pub fn read_parse_opt<T>(&self, name: &str) -> anyhow::Result<Option<T>>
    where
        T: FromStr,
        <T as FromStr>::Err: std::error::Error + Send + Sync + 'static,
    {
        self.0
            .attribute(name)
            .map(|v| v.parse().with_context(|| format!("parse '{name}'")))
            .transpose()
    }

    pub fn read_parse_list<T>(&self, name_start: &str) -> Vec<T>
    where
        T: FromStr,
        <T as FromStr>::Err: std::error::Error + Send + Sync + 'static,
    {
        // TODO: ensure order is correct
        // TODO: generally improve this, eg. allow suffixes as is used by enemy drops
        self.0
            .attributes()
            .filter(|a| a.name().starts_with(name_start))
            .flat_map(|a| {
                a.value()
                    .parse()
                    .with_context(|| format!("parse '{name_start}*'"))
            })
            .collect::<Vec<_>>()
    }

    pub fn read_string(&self, name: &str) -> anyhow::Result<String> {
        Ok(self
            .0
            .attribute(name)
            .with_context(|| format!("field '{name}' is required"))?
            .to_string())
    }

    pub fn read_string_opt(&self, name: &str) -> Option<String> {
        self.0.attribute(name).map(|s| s.to_string())
    }

    pub fn read_string_list(&self, name_start: &str) -> Vec<String> {
        self.0
            .attributes()
            .filter(|a| a.name().starts_with(name_start))
            .map(|a| a.value().to_string())
            .collect::<Vec<_>>()
    }

    pub fn read_present(&self, name: &str) -> bool {
        self.0.attribute(name).is_some()
    }
}
