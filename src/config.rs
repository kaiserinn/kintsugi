use crate::cli::{Cli, Commands};
use log::info;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use toml;

const NIXOS_DEFAULT_CONFIG: &str = "/etc/nixos";

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Config {
    pub editor: String,
    pub git: bool,
    pub nix_config_path: PathBuf,
    pub no_diff: bool,
}

impl Config {
    pub fn load_from_toml() -> Self {
        let kintsugi_config = std::env::home_dir()
            .unwrap()
            .join(".config/kintsugi/kintsugi.toml");

        if !kintsugi_config.exists() {
            info!("Config file is not found");
            info!(
                "Generating a default config at {}",
                kintsugi_config.to_str().unwrap()
            );
            Config::generate_toml(&kintsugi_config);
        }

        let kintsugi_config = std::fs::read_to_string(kintsugi_config).unwrap();

        toml::from_str(&kintsugi_config).unwrap()
    }

    pub fn merge_args(&mut self, cli: &Cli) -> Self {
        match &cli.command {
            Commands::Construct(args) => {
                if args.git {
                    self.git = true;
                }
                if args.no_diff.is_some() {
                    self.no_diff = true;
                }
            }
            Commands::Config(args) => {
                if let Some(editor) = &args.editor {
                    self.editor = editor.to_string();
                }
            }
        };

        self.clone()
    }

    pub fn merge_env(&mut self) -> Self {
        // TODO: Automate
        if let Ok(editor_env) = std::env::var("KINTSUGI_EDITOR") {
            self.editor = editor_env;
        }
        if let Ok(git_env) = std::env::var("KINTSUGI_GIT") {
            if let Ok(val) = git_env.parse::<bool>() {
                self.git = val;
            }
        }
        if let Ok(nix_config_path_env) = std::env::var("KINTSUGI_NIX_CONFIG_PATH") {
            self.nix_config_path = PathBuf::from(nix_config_path_env);
        }
        if let Ok(no_diff_env) = std::env::var("KINTSUGI_NO_DIFF") {
            if let Ok(val) = no_diff_env.parse::<bool>() {
                self.no_diff = val;
            }
        }

        self.clone()
    }

    fn generate_toml(path: &std::path::Path) {
        let defaults = Config::default();
        let defaults = toml::to_string(&defaults).unwrap();

        std::fs::create_dir_all(path.parent().unwrap()).unwrap();

        std::fs::write(path, defaults).unwrap();
    }
}

impl Default for Config {
    fn default() -> Self {
        let editor = std::env::var("VISUAL")
            .or_else(|_| std::env::var("EDITOR"))
            .unwrap_or_else(|_| "vim".to_string());

        Config {
            editor,
            git: false,
            nix_config_path: PathBuf::from(NIXOS_DEFAULT_CONFIG),
            no_diff: false,
        }
    }
}
