use clap::Parser;
use kintsugi::{cli::*, config::Config};
use log::debug;

fn main() {
    kintsugi::init_log();

    let cli = Cli::parse();
    let config = Config::load(&cli);
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
