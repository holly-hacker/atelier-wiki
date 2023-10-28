mod extract;
pub mod extract_images;
pub mod ryza3;
mod shared;
pub mod sophie;
mod typedefs;
mod utils;

use anyhow::Context;
use argh::FromArgs;
use tracing::{debug, info, trace};

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

    init_logging(&args);
    debug!("Initialized tracing");
    trace!("Trace logging enabled");

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

fn init_logging(args: &CliArgs) {
    use tracing_subscriber::filter::{FilterFn, LevelFilter};
    use tracing_subscriber::prelude::*;
    use tracing_subscriber::{fmt, registry};

    let log_level = if args.trace {
        tracing::Level::TRACE
    } else if args.verbose || cfg!(debug_assertions) {
        tracing::Level::DEBUG
    } else {
        tracing::Level::INFO
    };
    let level_filter = LevelFilter::from_level(log_level);

    let target_filter = FilterFn::new(|md| {
        md.target().starts_with("data_prepper")
            || md.target().starts_with("gust_g1t")
            || md.target().starts_with("gust_pak")
            || md.target().starts_with("dds_decoder")
    });

    let layer = fmt::layer()
        .with_filter(target_filter)
        .with_filter(level_filter);

    registry().with(layer).init();
}
