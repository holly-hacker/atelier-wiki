mod config;
mod extract;
mod extract_images;
mod typedefs;
mod utils;

use std::path::{Path, PathBuf};

use anyhow::Context;
use argh::FromArgs;
use config::Config;
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

    /// the path to an optional config file, defaults to `config.toml`
    #[argh(option, short = 'c')]
    config: Option<PathBuf>,

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
    trace!("Trace logging enabled");

    debug!("Looking for config file");
    let config_path = args.config.unwrap_or_else(|| PathBuf::from("config.toml"));
    let mut config = None;
    if config_path.exists() {
        info!("Loading config from {:?}", config_path);
        match load_config(&config_path) {
            Ok(loaded_config) => {
                debug!(?loaded_config);
                config = Some(loaded_config);
            }
            Err(error) => tracing::error!("Failed to load config: {:?}", error),
        }
    } else {
        info!("No config file found, using default config");
    }

    let time_before_command_handling = std::time::Instant::now();
    let result = match args.subcommand {
        Subcommand::Extract(args) => args.handle().context("Run extract command"),
        Subcommand::ExtractImages(args) => {
            args.handle(config).context("Run extract-images command")
        }
        Subcommand::TypeDefs(args) => args.handle().context("Extract typescript defs"),
    };
    let time_elapsed = time_before_command_handling.elapsed();

    if let Err(error) = result {
        tracing::error!("An error occured: {:?}", error);
    }

    info!("Time elapsed: {:?}", time_elapsed);
}

fn load_config(path: &Path) -> anyhow::Result<Config> {
    let content = std::fs::read_to_string(path).context("read config file")?;
    let config = toml::from_str(&content).context("parse config file")?;
    Ok(config)
}
