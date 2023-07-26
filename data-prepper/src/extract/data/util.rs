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

// TODO: consider a mode where the read properties are tracked, allowing to check for missing properties
impl<'x, 'a, 'b> ElementReader<'x, 'a, 'b> {
    pub fn read<T>(&self, name: &str) -> anyhow::Result<T>
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

    pub fn read_opt<T>(&self, name: &str) -> anyhow::Result<Option<T>>
    where
        T: FromStr,
        <T as FromStr>::Err: std::error::Error + Send + Sync + 'static,
    {
        self.0
            .attribute(name)
            .map(|v| v.parse().with_context(|| format!("parse '{name}'")))
            .transpose()
    }

    pub fn read_list<T>(&self, name_pattern: &'static str) -> Vec<T>
    where
        T: FromStr,
        <T as FromStr>::Err: std::error::Error + Send + Sync + 'static,
    {
        // TODO: ensure order is correct
        self.0
            .attributes()
            .filter(|a| Self::match_pattern(name_pattern, a.name()).is_some())
            .flat_map(|a| {
                a.value()
                    .parse()
                    .with_context(|| format!("parse '{name_pattern}'"))
            })
            .collect::<Vec<_>>()
    }

    fn match_pattern<'h>(needle: &'static str, haystack: &'h str) -> Option<&'h str> {
        let Some(index) = needle.find('*') else {
            return if needle == haystack {
                Some(haystack)
            } else {
                None
            }
        };

        let left = &needle[..index];
        let right = &needle[index + 1..];

        if haystack.starts_with(left) && haystack.ends_with(right) {
            Some(&haystack[left.len()..haystack.len() - right.len()])
        } else {
            None
        }
    }

    pub fn read_present(&self, name: &str) -> bool {
        self.0.attribute(name).is_some()
    }
}
