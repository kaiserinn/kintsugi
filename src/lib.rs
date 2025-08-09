use crate::{
    cli::{ConfigArgs, ConstructArgs, Operations},
    config::Config,
};
use chrono::Local;
use env_logger::Builder;
use log::info;
use std::process::Command;

pub mod cli;
pub mod config;

pub fn config(args: ConfigArgs, config: &Config) {
    let _ = args;
    let config_dir = std::env::home_dir().unwrap().join(".config/nix");

    Command::new(&config.editor)
        .current_dir(config_dir)
        .status()
        .unwrap();
}

pub fn construct(args: ConstructArgs, config: &Config) {
    let op = args.operation.as_ref().unwrap_or(&Operations::Switch);

    info!("Executing `nixos-rebuild {op}`");

    Command::new("sudo")
        .arg("nixos-rebuild")
        .arg(op.to_string())
        .arg("--flake")
        .arg(&config.nix_config_path)
        .status()
        .unwrap();

    if config.git && *op != Operations::Test {
        git(args, config);
    }
}

pub fn git(args: ConstructArgs, config: &Config) {
    let commit_message = args.message.unwrap_or_else(|| {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M").to_string();
        format!("System rebuild at {timestamp}")
    });

    info!("Adding changes to index");
    Command::new("yadm")
        .current_dir(&config.nix_config_path)
        .arg("add")
        .arg(".")
        .status()
        .unwrap();

    info!("Committing changes as: '{commit_message}'");
    Command::new("yadm")
        .arg("commit")
        .arg("--message")
        .arg(&commit_message)
        .status()
        .unwrap();

    info!("Pushing changes");
    Command::new("yadm").arg("push").status().unwrap();
}

pub fn init_log() {
    Builder::new()
        .format_target(false)
        .filter_level(log::LevelFilter::Info)
        .parse_default_env()
        .init();
}
