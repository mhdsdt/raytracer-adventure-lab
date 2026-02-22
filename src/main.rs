mod app;
mod core;
mod presets;
mod scenes;

use clap::Parser;

use app::cli::{CliArgs, LaunchConfig};
use app::shell;

fn main() -> anyhow::Result<()> {
    let args = CliArgs::parse();
    let config = LaunchConfig::from_args(args)?;
    config.print_startup_summary();
    shell::run(config)
}
