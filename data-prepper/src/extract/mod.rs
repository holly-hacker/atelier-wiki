mod data;

use std::path::PathBuf;

use anyhow::{bail, Context};
use argh::FromArgs;
pub use data::Ryza3Data;
use tracing::{debug, info};

use crate::utils::{extract_game_version, PakIndex};

/// Extract and prepare the game data from the game install directory
#[derive(FromArgs)]
#[argh(subcommand, name = "extract")]
pub struct ExtractArgs {
    /// the game install directory
    #[argh(option, short = 'i')]
    game_directory: PathBuf,

    /// the output directory
    #[argh(option, short = 'o')]
    output_directory: Option<PathBuf>,
}

pub fn extract(args: ExtractArgs) -> anyhow::Result<()> {
    let output_directory = args
        .output_directory
        .unwrap_or_else(|| PathBuf::from("game_data"));
    debug!(?output_directory);

    debug!("Detecting game version");
    let game_version = extract_game_version(&args.game_directory);
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
    let pak_dir = args.game_directory.join("Data");
    let mut pak_index = PakIndex::read(&pak_dir, game_version).context("read data dir")?;
    info!("Loaded pak file index with {} entries", pak_index.len());

    // TODO: only if game version is A24
    let data = data::Ryza3Data::read_all(&mut pak_index).context("read data files")?;
    let formatted_data = serde_json::to_string_pretty(&data).context("format data")?;

    debug!("Creating output directory");
    std::fs::create_dir_all(&output_directory).context("create output directory")?;

    debug!("Writing file");
    let output_file_path = output_directory.join("ryza3.json");
    let mut output_file = std::fs::File::create(&output_file_path)
        .with_context(|| format!("create output file {:?}", output_file_path))?;
    std::io::copy(&mut formatted_data.as_bytes(), &mut output_file).context("write output file")?;
    info!("Wrote data to {:?}", output_file_path);

    Ok(())
}
