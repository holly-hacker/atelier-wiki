mod extract;

use anyhow::Context;
use argh::FromArgs;
use tracing::{debug, info};

/// Data Prepper
#[derive(FromArgs)]
struct CliArgs {
    #[argh(subcommand)]
    subcommand: Subcommand,
}

#[derive(FromArgs)]
#[argh(subcommand)]
enum Subcommand {
    Extract(extract::ExtractArgs),
}

fn main() {
    let args: CliArgs = argh::from_env();

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();
    debug!("Initialized tracing");

    let time_before_command_handling = std::time::Instant::now();
    let result = match args.subcommand {
        Subcommand::Extract(extract_args) => {
            extract::extract(extract_args).context("Run extract command")
        }
    };
    let time_elapsed = time_before_command_handling.elapsed();

    if let Err(error) = result {
        tracing::error!("An error occured: {:?}", error);
    }

    info!("Time elapsed: {:?}", time_elapsed);
}
