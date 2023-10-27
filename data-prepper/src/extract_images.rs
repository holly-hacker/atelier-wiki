use std::path::PathBuf;
use std::str::FromStr;

use anyhow::{bail, Context};
use argh::FromArgs;
use tracing::{debug, info};

use crate::ryza3;
use crate::utils::{extract_game_version, game_slug, PakIndex};

/// Extract and prepare the game data from the game install directory
#[derive(FromArgs)]
#[argh(subcommand, name = "extract-images")]
pub struct Args {
    /// the game install directory
    #[argh(option, short = 'i')]
    pub game_directory: PathBuf,

    /// the output directory for the generated files
    #[argh(option, short = 'o')]
    pub output_directory: Option<PathBuf>,

    /// don't write images to disk (this does not prevent in-memory decoding)
    #[argh(switch, short = 'd')]
    pub dont_write_images: bool,

    /// level of oxipng compression to use. if not present, use standard png encoder for png
    #[argh(option, short = 'c')]
    pub compression: Option<u8>,

    /// the category of images to extract. if not present, extract all images
    #[argh(option)]
    pub category: Option<Category>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Category {
    Monsters,
    Items,
    Maps,
}

impl FromStr for Category {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "monsters" => Ok(Self::Monsters),
            "items" => Ok(Self::Items),
            "maps" => Ok(Self::Maps),
            _ => Err(format!("Unknown category {}", s)),
        }
    }
}

impl Args {
    pub fn handle(self) -> anyhow::Result<()> {
        debug!("Detecting game version");
        let game_version = extract_game_version(&self.game_directory);
        let Some(game_version) = game_version else {
            bail!("Could not detect game version in the given install directory");
        };
        let slug = game_slug(game_version);
        info!(
            "Detected game {game_version:?} ({}), using slug {slug}",
            game_version.get_short_name()
        );

        let output_dir = self
            .output_directory
            .clone()
            .unwrap_or_else(|| PathBuf::from("game_data"));
        debug!(?output_dir);

        // loading index of game files
        debug!("Reading pak file index");
        let mut pak_index =
            PakIndex::read(&self.game_directory, game_version).context("read data dir")?;
        info!("Loaded pak file index with {} entries", pak_index.len());

        let output_directory = output_dir.join(slug);

        debug!("Creating output directory");
        std::fs::create_dir_all(&output_directory).context("create output directory")?;

        // TODO: switch on game type
        match game_version {
            gust_pak::common::GameVersion::A17 => todo!("extract sophie images"),
            gust_pak::common::GameVersion::A24 => {
                ryza3::extract_images(&self, &mut pak_index, &output_directory, self.category)?
            }
            _ => bail!("Unsupported game version {:?}", game_version),
        }

        info!("Extracted images");

        Ok(())
    }
}
