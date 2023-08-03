use std::path::PathBuf;

use anyhow::Context;
use argh::FromArgs;
use tracing::{debug, info};
use typescript_type_def::{write_definition_file, DefinitionFileOptions, TypeDef};

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

        debug!("Generating typedefs for ryza3");
        let ts_module =
            gen_typedefs::<super::extract::Ryza3Data>().context("generate typedefs for ryza3")?;
        let output_file = output_folder.join("ryza3.d.ts");
        std::fs::create_dir_all(&output_folder).context("create output folder")?;
        std::fs::write(output_file, ts_module).context("write output file")?;

        info!("Wrote all typedefs to {:?}", output_folder);

        Ok(())
    }
}

fn gen_typedefs<T>() -> anyhow::Result<String>
where
    T: TypeDef,
{
    let mut buf = Vec::new();
    write_definition_file::<_, T>(&mut buf, DefinitionFileOptions::default())
        .context("generate definition")?;
    String::from_utf8(buf).context("convert typedef to string")
}
