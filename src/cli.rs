use std::fmt;

use clap::{Args, Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(
    version,
    about,
    long_about = "Kintsugi (kin) is a command-line tool designed to streamline the \
        management of your NixOS configurations.\nInspired by the Japanese art \
        of repairing with gold, Kintsugi treats every change as a valuable \
        part of your system's history."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Rebuild NixOS's configuration.
    Construct(ConstructArgs),

    /// Open your editor in the Nix configuration directory.
    Config(ConfigArgs),
}

#[derive(Args, Debug)]
pub struct ConstructArgs {
    #[command(subcommand)]
    pub operation: Option<Operations>,

    #[arg(
        short,
        long,
        help = "Automatically commit changes after a successful rebuild",
        long_help = "Automatically commit changes using Git after a successful \
            rebuild.\n\nThe default commit message is 'System rebuild at \
            <timestamp>'.\nUse the --message flag to provide a custom one."
    )]
    pub git: bool,

    #[arg(
        short,
        long,
        help = "Provide a custom commit message",
        long_help = "Provide a custom commit message for the automatic Git \
            commit.\nThis flag has no effect unless --git is also used."
    )]
    pub message: Option<String>,
}

#[derive(Args, Debug)]
pub struct ConfigArgs {
    /// The editor to use for opening config
    #[arg(short, long)]
    pub editor: Option<String>,
}

#[derive(Subcommand, PartialEq, Debug)]
pub enum Operations {
    /// Corresponds to `nixos-rebuild switch`.
    Switch,

    /// Corresponds to `nixos-rebuild test`.
    Test,

    /// Corresponds to `nixos-rebuild boot`.
    Boot,
}

impl fmt::Display for Operations {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Operations::Switch => "switch",
                Operations::Test => "test",
                Operations::Boot => "boot",
            }
        )
    }
}
