mod data;

use std::path::{Path, PathBuf};

use anyhow::{bail, Context};
use argh::FromArgs;
pub use data::Ryza3Data;
use gust_pak::common::GameVersion;
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
    let data = data::Ryza3Data::read_all(&mut pak_index).context("read data files")?;
    let formatted_data = serde_json::to_string_pretty(&data).context("format data")?;

    debug!("Creating output directory");
    std::fs::create_dir_all(output_directory).context("create output directory")?;

    debug!("Writing files");
    let output_file_path = output_directory.join("ryza3.json");
    let mut output_file = std::fs::File::create(&output_file_path)
        .with_context(|| format!("create output file {:?}", output_file_path))?;
    std::io::copy(&mut formatted_data.as_bytes(), &mut output_file).context("write output file")?;
    info!("Wrote data to {:?}", output_file_path);

    Ok(())
}
