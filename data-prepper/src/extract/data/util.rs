use std::{io::Read, str::FromStr};

use anyhow::Context;

use crate::utils::{match_pattern, PakIndex};

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

pub struct ElementReader<'node, 'attr, 'xml_str>(pub &'node roxmltree::Node<'attr, 'xml_str>);

// TODO: consider a mode where the read properties are tracked, allowing to check for missing properties
impl<'node, 'attr, 'xml_str> ElementReader<'node, 'attr, 'xml_str> {
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

    /// Read a continuous list of values.
    ///
    /// This function returns an error if the list is not continuous, ie. if there are holes in the list, or if there
    /// are duplicated values.
    pub fn read_list<T>(&self, name_pattern: &'static str) -> anyhow::Result<Vec<T>>
    where
        T: FromStr,
        <T as FromStr>::Err: std::error::Error + Send + Sync + 'static,
    {
        // extract all attributes that match the pattern
        let mut values_with_indices = self
            .0
            .attributes()
            .flat_map(|a| match_pattern(name_pattern, a.name()).map(|idx| (idx, a.value())))
            .collect::<Vec<_>>();

        values_with_indices.sort_by(|(i1, _), (i2, _)| i1.cmp(i2));

        // we have a vec<(index, value)> tuples, now extract it to vec<value> and ensure the indices are correct
        let parsed_list = values_with_indices
            .into_iter()
            .enumerate()
            .map(|(i_real, (i_attr, value))| {
                if i_real != i_attr {
                    anyhow::bail!("index mismatch: expected {i_real}, got {i_attr} for pattern {name_pattern}")
                }

                value.parse().with_context(|| {
                    format!("parse value `{value}` for '{name_pattern}' index {i_real}")
                })
            })
            .collect::<anyhow::Result<Vec<_>>>()?;

        Ok(parsed_list)
    }

    /// Read a sparse list, ie. a list that may contain hole.
    ///
    /// This function returns an error if there are duplicated values.
    pub fn read_sparse_list<T>(&self, name_pattern: &'static str) -> anyhow::Result<Vec<Option<T>>>
    where
        T: FromStr,
        <T as FromStr>::Err: std::error::Error + Send + Sync + 'static,
    {
        // extract all attributes that match the pattern
        let mut values_with_indices = self
            .0
            .attributes()
            .flat_map(|a| match_pattern(name_pattern, a.name()).map(|idx| (idx, a.value())))
            .collect::<Vec<_>>();

        values_with_indices.sort_by(|(i1, _), (i2, _)| i1.cmp(i2));

        let len = values_with_indices.iter().map(|(i, _)| i).max();

        let Some(&len) = len else {
            return Ok(Vec::new());
        };

        // we have a vec<(index, value)> tuples, now extract it to vec<option<value>> for each index until the max
        let parsed_list = (0..=len)
            .map(|i| {
                let mut opt_iter = values_with_indices
                    .iter()
                    .filter(move |(i_attr, _)| *i_attr == i);

                // NOTE: I initially implemented this using itertool's at_most_one, but this gave very weird lifetime
                // errors.
                let one = opt_iter.next();
                let two = opt_iter.next();

                if two.is_some() {
                    anyhow::bail!("duplicate index {i} for pattern {name_pattern}");
                }

                one.map(|(i, value)| {
                    value.parse().with_context(|| {
                        format!("parse value `{value}` for '{name_pattern}' index {i}")
                    })
                })
                .transpose()
            })
            .collect::<anyhow::Result<Vec<Option<_>>>>()?;

        Ok(parsed_list)
    }

    /// Read a sparse list, ie. a list that may contain hole, and flatten it. This essentially "hides" the holes in the
    /// function.
    ///
    /// This function returns an error if there are duplicated values.
    ///
    /// This function is an optimized specialization of
    /// `reader.read_sparse_list(my_pattern)?.into_iter().flatten().collect();`.
    pub fn read_flattened_sparse_list<T>(
        &self,
        name_pattern: &'static str,
    ) -> anyhow::Result<Vec<T>>
    where
        T: FromStr,
        <T as FromStr>::Err: std::error::Error + Send + Sync + 'static,
    {
        // extract all attributes that match the pattern
        let mut values_with_indices = self
            .0
            .attributes()
            .flat_map(|a| match_pattern(name_pattern, a.name()).map(|idx| (idx, a.value())))
            .collect::<Vec<_>>();

        values_with_indices.sort_by(|(i1, _), (i2, _)| i1.cmp(i2));

        let len = values_with_indices.iter().map(|(i, _)| i).max();

        let Some(&len) = len else {
            return Ok(Vec::new());
        };

        // we have a vec<(index, value)> tuples, now extract it to vec<option<value>> for each index until the max
        let parsed_list = (0..=len)
            .filter_map(|i| {
                let mut opt_iter = values_with_indices
                    .iter()
                    .filter(move |(i_attr, _)| *i_attr == i);

                // NOTE: I initially implemented this using itertool's at_most_one, but this gave very weird lifetime
                // errors.
                let one = opt_iter.next();
                let two = opt_iter.next();

                if two.is_some() {
                    return Some(Err(anyhow::anyhow!(
                        "duplicate index {i} for pattern {name_pattern}"
                    )));
                }

                one.map(|(i, value)| {
                    value.parse().with_context(|| {
                        format!("parse value `{value}` for '{name_pattern}' index {i}")
                    })
                })
            })
            .collect::<anyhow::Result<Vec<_>>>()?;

        Ok(parsed_list)
    }

    pub fn is_present(&self, name: &str) -> bool {
        self.0.attribute(name).is_some()
    }
}
