use std::path::{Path, PathBuf};

use anyhow::Context;
use argh::FromArgs;
use gust_pak::common::GameVersion;
use tracing::{debug, info};
use typescript_type_def::{write_definition_file, DefinitionFileOptions, TypeDef};

use crate::utils::game_slug;

/// Generate typescript definitions
#[derive(FromArgs)]
#[argh(subcommand, name = "type-defs")]
pub struct Args {
    /// the output folder
    #[argh(option, short = 'o')]
    output_path: Option<PathBuf>,
}

impl Args {
    pub fn handle(self) -> anyhow::Result<()> {
        let output_folder = self
            .output_path
            .unwrap_or_else(|| PathBuf::from("typedefs"));
        debug!(?output_folder);

        debug!("Generating typedefs");
        gen_typedefs::<super::extract::sophie::data::SophieData>(
            &output_folder,
            &format!("{}.d.ts", game_slug(GameVersion::A17)),
        )
        .context("generate typedefs for sophie")?;
        gen_typedefs::<super::extract::ryza3::data::Ryza3Data>(
            &output_folder,
            &format!("{}.d.ts", game_slug(GameVersion::A24)),
        )
        .context("generate typedefs for ryza3")?;
        gen_typedefs::<super::extract_images::UniformTextureAtlasInfo>(
            &output_folder,
            "texture_atlas.d.ts",
        )
        .context("generate texture atlas typedefs")?;
        gen_typedefs::<super::extract_images::MapInfoList>(&output_folder, "map_data.d.ts")
            .context("generate map data typedefs")?;

        info!("Wrote all typedefs to {:?}", output_folder);

        Ok(())
    }
}

fn gen_typedefs<T>(output_folder: &Path, file_name: &str) -> anyhow::Result<()>
where
    T: TypeDef,
{
    let mut buf = Vec::new();
    write_definition_file::<_, T>(&mut buf, DefinitionFileOptions::default())
        .context("generate definition")?;
    let ts_module = String::from_utf8(buf).context("convert typedef to string")?;

    let output_file = output_folder.join(file_name);
    std::fs::create_dir_all(output_folder).context("create output folder")?;
    std::fs::write(output_file, ts_module).context("write output file")?;

    Ok(())
}
