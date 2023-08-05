mod data;

use std::{
    fs::File,
    path::{Path, PathBuf},
};

use anyhow::{bail, Context};
use argh::FromArgs;
pub use data::Ryza3Data;
use gust_pak::common::GameVersion;
use serde::Serialize;
use tracing::{debug, info};

use crate::utils::{extract_game_version, PakIndex};

/// Extract and prepare the game data from the game install directory
#[derive(FromArgs)]
#[argh(subcommand, name = "extract")]
pub struct Args {
    /// the game install directory
    #[argh(option, short = 'i')]
    game_directory: PathBuf,

    /// the output directory
    #[argh(option, short = 'o')]
    output_directory: Option<PathBuf>,
}

impl Args {
    pub fn handle(self) -> anyhow::Result<()> {
        let output_directory = self
            .output_directory
            .unwrap_or_else(|| PathBuf::from("game_data"));
        debug!(?output_directory);

        debug!("Detecting game version");
        let game_version = extract_game_version(&self.game_directory);
        let Some(game_version) = game_version else {
            bail!("Could not detect game version in the given install directory");
        };
        info!(
            "Detected game {:?} ({})",
            game_version,
            game_version.get_short_name()
        );

        // loading index of game files
        debug!("Reading pak file index");
        let pak_dir = self.game_directory.join("Data");
        let pak_index = PakIndex::read(&pak_dir, game_version).context("read data dir")?;
        info!("Loaded pak file index with {} entries", pak_index.len());

        match game_version {
            GameVersion::A24 => extract_ryza3(pak_index, &output_directory),
            _ => bail!("Unsupported game version {:?}", game_version),
        }
    }
}

fn extract_ryza3(mut pak_index: PakIndex, output_directory: &Path) -> anyhow::Result<()> {
    let output_directory = output_directory.join("ryza3");
    let data = data::Ryza3Data::read_all(&mut pak_index).context("read data files")?;

    debug!("Creating output directory");
    std::fs::create_dir_all(&output_directory).context("create output directory")?;

    debug!("Writing files");

    info!("Writing item data");
    write_data_to_file(&output_directory.join("items.json"), &data.item_data)
        .context("write item data")?;

    info!("Writing enemy data");
    write_data_to_file(&output_directory.join("enemies.json"), &data.enemy_data)
        .context("write enemy data")?;

    info!("Wrote ryza3 data to {:?}", output_directory);

    Ok(())
}

pub fn write_data_to_file<T>(path: &Path, data: &T) -> anyhow::Result<()>
where
    T: Serialize,
{
    let mut output_file =
        File::create(path).with_context(|| format!("create output file {:?}", path))?;

    let formatted_data = serde_json::to_string_pretty(data).context("format data")?;

    std::io::copy(&mut formatted_data.as_bytes(), &mut output_file).context("write output file")?;

    Ok(())
}
