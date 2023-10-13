use std::path::Path;

use anyhow::{bail, Context};
use tracing::{debug, info};

pub struct Ryza3ExecutableData {
    pub item_categories: Vec<String>,
    pub item_effects: Vec<String>,
}

impl Ryza3ExecutableData {
    pub fn read_all(game_directory: &Path) -> anyhow::Result<Self> {
        let executable_path = game_directory.join("Atelier_Ryza_3.exe");

        debug!("verify file size");
        {
            let executable_file = std::fs::File::open(&executable_path)
                .with_context(|| format!("open executable {:?}", executable_path))?;

            let metadata = executable_file
                .metadata()
                .context("read executable metadata")?;
            if metadata.len() > 500 * 1024 * 1024 {
                bail!("executable is larger than expected (expected less than 500MB but found {}MB), is this the correct file?", metadata.len()/1024/1024);
            }
        }

        debug!("read file");
        let executable_data = std::fs::read(executable_path).context("read executable")?;

        debug!("read item categories");
        // Binary regions:
        // - v1.5.0.0: 01687370-016877E0 (47 tags)
        let item_categories =
            find_tag(&executable_data, b"ITEM_CATEGORY_").context("read item categories")?;
        info!("Read {} item categories", item_categories.len());

        debug!("read item effects");
        // Binary regions:
        // - v1.5.0.0: 016877F0-016A36D4 (3499 tags)
        let item_effects = find_tag(&executable_data, b"ITEM_EFF_").context("read item effects")?;
        info!("Read {} item effects tags", item_effects.len());

        // dbg!(&item_effects);

        Ok(Self {
            item_categories,
            item_effects,
        })
    }
}

fn find_tag(file: &[u8], tag_prefix: &[u8]) -> anyhow::Result<Vec<String>> {
    // Initially, this was implemented by grouping all matches based on a distance, but it turns out
    // there is only 1 location where tags are stored so this wasn't needed. For now, we just read
    // all matches and return them.
    // This was tested on v1.5.0.0 of the game, future or past version may not exhibit this behavior.

    let tag_with_null_byte = vec![0u8]
        .into_iter()
        .chain(tag_prefix.iter().copied())
        .collect::<Vec<_>>();

    let mut ret = vec![];

    let mut min_pos = usize::MAX;
    let mut max_pos = usize::MIN;
    for pos in memchr::memmem::find_iter(file, &tag_with_null_byte).map(|i| i + 1) {
        // read until the next null-byte
        let len = memchr::memchr(b'\0', &file[pos..]).unwrap_or(file.len());

        if len > tag_prefix.len() {
            min_pos = min_pos.min(pos);
            max_pos = max_pos.max(pos);

            let tag = std::str::from_utf8(&file[pos..pos + len])
                .with_context(|| format!("parse tag at {}", pos))?
                .to_owned();

            ret.push(tag);
        }
    }

    if ret.is_empty() {
        bail!("no tags found in binary");
    }

    // ensure all the positions we read are within the same region
    let max_pos_spread: usize = ret.len() * 64;
    let pos_spread = max_pos - min_pos;
    if pos_spread > max_pos_spread {
        bail!("tags are too far apart (expected less than 0x{max_pos_spread:X} but found 0x{pos_spread:X})");
    }

    Ok(ret)
}
