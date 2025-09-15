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

pub fn dev() {
    Command::new("nix")
        .arg("develop")
        .arg("-c")
        .arg("fish")
        .status()
        .unwrap();
}

pub fn config(args: ConfigArgs, config: &Config) {
    let _ = args;

    Command::new(&config.editor)
        .current_dir(&config.nix_config_path)
        .status()
        .unwrap();
}

pub fn construct(args: ConstructArgs, config: &Config) {
    let op = args.operation.as_ref().unwrap_or(&Operations::Switch);

    if !config.no_diff {
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
    }

    let status = match *op {
        Operations::Home => {
            info!(
                "Executing `NH_FLAKE={} nh home switch`",
                &config.nix_config_path.to_string_lossy()
            );

            Command::new("nh")
                .arg("home")
                .arg("switch")
                .env("NH_FLAKE", &config.nix_config_path)
                .status()
                .unwrap()
        }
        _ => {
            info!(
                "Executing `NH_FLAKE={} nh os {op}`",
                &config.nix_config_path.to_string_lossy()
            );

            Command::new("nh")
                .arg("os")
                .arg(op.to_string())
                .env("NH_FLAKE", &config.nix_config_path)
                .status()
                .unwrap()
        }
    };

    if config.git && *op != Operations::Test && status.success() {
        git(args, config);
    }
}

// TODO: Handle cases where git is not properly initialized
// TODO: Backup
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

    info!("Executing `jj bookmark set trunk -r @`");
    Command::new("jj")
        .arg("bookmark")
        .arg("set")
        .arg("trunk")
        .arg("-r")
        .arg("@")
        .current_dir(&config.nix_config_path)
        .status()
        .unwrap();

    info!("Executing `jj git push -b trunk --allow-new`");
    Command::new("jj")
        .arg("git")
        .arg("push")
        .arg("-b")
        .arg("trunk")
        .arg("--allow-new")
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
