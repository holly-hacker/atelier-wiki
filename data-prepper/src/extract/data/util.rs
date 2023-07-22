use std::io::Read;

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
