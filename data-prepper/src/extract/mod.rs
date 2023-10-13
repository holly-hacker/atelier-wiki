mod ryza3;
mod util;

use std::{
    fs::File,
    path::{Path, PathBuf},
};

use anyhow::{bail, Context};
use argh::FromArgs;
use gust_pak::common::GameVersion;
pub use ryza3::data::Ryza3Data;
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

    /// the game version. detected automatically if not specified
    #[argh(option, short = 'g')]
    game_version: Option<GameVersion>,
}

impl Args {
    pub fn handle(self) -> anyhow::Result<()> {
        let output_directory = self
            .output_directory
            .unwrap_or_else(|| PathBuf::from("game_data"));
        debug!(?output_directory);

        debug!("Detecting game version");
        let game_version = self
            .game_version
            .or_else(|| extract_game_version(&self.game_directory));
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

        // extract data from game files
        match game_version {
            GameVersion::A24 => ryza3::extract(&self.game_directory, pak_index, &output_directory),
            _ => bail!("Unsupported game version {:?}", game_version),
        }
    }
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
