mod pak_index;

use std::path::{Path, PathBuf};

use anyhow::{bail, Context};
use argh::FromArgs;
use gust_pak::common::GameVersion;
use tracing::{debug, info};

use crate::extract::pak_index::PakIndex;

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
    let mut map = PakIndex::read(&pak_dir, game_version).context("read data dir")?;
    info!("Loaded pak file index with {} entries", map.len());

    // for now, just extract some file
    let file_read = map
        .get_file(r"\saves\item\itemdata.xml")
        .context("read file ")?;

    let Some(mut file_read) = file_read else {
        bail!("File {} not found", r"\saves\item\itemdata.xml");
    };

    debug!("Creating output directory");
    std::fs::create_dir_all(&output_directory).context("create output directory")?;

    debug!("Writing file");
    let output_file_path = output_directory.join("itemdata.xml");
    let mut output_file = std::fs::File::create(&output_file_path)
        .with_context(|| format!("create output file {:?}", output_file_path))?;
    std::io::copy(&mut file_read, &mut output_file).context("write output file")?;

    Ok(())
}

fn extract_game_version(path: &Path) -> Option<GameVersion> {
    // currently, only detect Atelier Ryza 3. we can add more later
    if path.join("Atelier_Ryza_3.exe").exists() {
        Some(GameVersion::A24)
    } else {
        None
    }
}
