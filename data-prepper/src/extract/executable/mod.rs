use std::path::Path;

use anyhow::{bail, Context};
use tracing::{debug, info};

pub struct Ryza3ExecutableData {
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

        debug!("read item effects");
        let item_effects = read_item_effects(&executable_data).context("read item effects")?;
        info!("Read {} item effects tags", item_effects.len());

        // dbg!(&item_effects);

        Ok(Self { item_effects })
    }
}

fn read_item_effects(file: &[u8]) -> anyhow::Result<Vec<String>> {
    // Initially, this was implemented by grouping all matches based on a distance, but it turns out
    // there is only 1 location where item effects are stored so this wasn't needed. For now, we
    // just read all matches and return them.
    // This was tested on v1.5.0.0 of the game, future or past version may not exhibit this behavior.

    // Binary regions:
    // - v1.5.0.0: 016877F0-016A36D4 (3499 strings)

    let mut ret = vec![];

    // for perf, don't use capture groups because we don't really need them
    // let regex = regex::bytes::Regex::new("\0ITEM_EFF_[A-Z0-9_]+\0").expect("create regex");
    for pos in memchr::memmem::find_iter(file, b"\0ITEM_EFF_").map(|i| i + 1) {
        // read until the next null-byte
        let len = memchr::memchr(b'\0', &file[pos..]).unwrap_or(file.len());

        if len > "ITEM_EFF_".len() {
            let item_effect = std::str::from_utf8(&file[pos..pos + len])
                .with_context(|| format!("parse item effect at {}", pos))?
                .to_owned();

            ret.push(item_effect);
        }
    }

    Ok(ret)
}
