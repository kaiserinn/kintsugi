use clap::Parser;
use kintsugi::{cli::*, config::Config};
use log::debug;

fn main() {
    kintsugi::init_log();

    let cli = Cli::parse();

    #[rustfmt::skip]
    let config = Config::load_from_toml()
        .merge_args(&cli)
        .merge_env();

    debug!("{config:#?}");

    match cli.command {
        Commands::Config(args) => {
            kintsugi::config(args, &config);
        }
        Commands::Construct(args) => {
            kintsugi::construct(args, &config);
        }
    };
}
