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
pub mod utils;

pub fn config(args: ConfigArgs, config: &Config) {
    let _ = args;

    Command::new(&config.editor)
        .current_dir(&config.nix_config_path)
        .status()
        .unwrap();
}

pub fn construct(args: ConstructArgs, config: &Config) {
    let op = args.operation.as_ref().unwrap_or(&Operations::Switch);

    Command::new("jj")
        .arg("diff")
        .arg("--stat")
        .arg("--no-pager")
        .current_dir(&config.nix_config_path)
        .status()
        .unwrap();

    if !utils::confirm("Continue?", true) {
        return;
    }

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

    info!("Executing `jj describe --message {commit_message}`");
    Command::new("jj")
        .arg("describe")
        .arg("--message")
        .arg(commit_message)
        .current_dir(&config.nix_config_path)
        .status()
        .unwrap();

    info!("Executing `jj new`");
    Command::new("jj")
        .arg("new")
        .current_dir(&config.nix_config_path)
        .status()
        .unwrap();

    info!("Executing `jj bookmark set master -r @`");
    Command::new("jj")
        .arg("bookmark")
        .arg("set")
        .arg("master")
        .arg("-r")
        .arg("@")
        .current_dir(&config.nix_config_path)
        .status()
        .unwrap();

    info!("Executing `jj git push`");
    Command::new("jj")
        .arg("git")
        .arg("push")
        .current_dir(&config.nix_config_path)
        .status()
        .unwrap();
}

pub fn init_log() {
    Builder::new()
        .format_target(false)
        .filter_level(log::LevelFilter::Info)
        .parse_default_env()
        .init();
}
