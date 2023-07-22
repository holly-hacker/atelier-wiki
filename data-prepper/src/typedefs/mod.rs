use std::path::PathBuf;

use anyhow::Context;
use argh::FromArgs;
use tracing::{debug, info};
use typescript_type_def::{write_definition_file, DefinitionFileOptions};

/// Generate typescript definitions
#[derive(FromArgs)]
#[argh(subcommand, name = "type-defs")]
pub struct TypeDefsArgs {
    /// the output file
    #[argh(option, short = 'o')]
    output_file: Option<PathBuf>,
}

pub fn generate_typedefs(args: TypeDefsArgs) -> anyhow::Result<()> {
    let output_file = args
        .output_file
        .unwrap_or_else(|| PathBuf::from("typedefs.d.ts"));
    debug!(?output_file);

    debug!("Generating typedefs");
    let ts_module = {
        let mut buf = Vec::new();
        let options = DefinitionFileOptions::default();
        write_definition_file::<_, super::extract::Data>(&mut buf, options)
            .context("generate definition")?;
        String::from_utf8(buf).context("convert typedef to string")?
    };

    debug!("Writing file");
    std::fs::write(&output_file, ts_module).context("write output file")?;

    info!("Wrote typedefs to {:?}", output_file);

    Ok(())
}
