mod extract;
mod extract_images;
mod typedefs;
mod utils;

use anyhow::Context;
use argh::FromArgs;
use tracing::{debug, info};

/// Data Prepper
#[derive(FromArgs)]
struct CliArgs {
    /// enable debug logging
    #[argh(switch, short = 'v')]
    verbose: bool,

    /// enable trace logging
    #[argh(switch, short = 't')]
    trace: bool,

    #[argh(subcommand)]
    subcommand: Subcommand,
}

#[derive(FromArgs)]
#[argh(subcommand)]
enum Subcommand {
    Extract(extract::Args),
    ExtractImages(extract_images::Args),
    TypeDefs(typedefs::Args),
}

fn main() {
    let args: CliArgs = argh::from_env();

    let log_level = if args.trace {
        tracing::Level::TRACE
    } else if args.verbose || cfg!(debug_assertions) {
        tracing::Level::DEBUG
    } else {
        tracing::Level::INFO
    };

    tracing_subscriber::fmt().with_max_level(log_level).init();
    debug!("Initialized tracing");

    let time_before_command_handling = std::time::Instant::now();
    let result = match args.subcommand {
        Subcommand::Extract(args) => args.handle().context("Run extract command"),
        Subcommand::ExtractImages(args) => args.handle().context("Run extract-images command"),
        Subcommand::TypeDefs(args) => args.handle().context("Extract typescript defs"),
    };
    let time_elapsed = time_before_command_handling.elapsed();

    if let Err(error) = result {
        tracing::error!("An error occured: {:?}", error);
    }

    info!("Time elapsed: {:?}", time_elapsed);
}
