use anyhow::{anyhow, Context, Result};
use serde::{Deserialize, Serialize};
use std::fs::{read_to_string, DirBuilder, OpenOptions};
use std::io::Write;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub projects_dir: PathBuf,
    pub hacks_dir: PathBuf,
}

impl Default for Config {
    fn default() -> Config {
        let projects_dir = dirs::home_dir().unwrap().join("projects");
        let hacks_dir = projects_dir.join("hacks");

        Config {
            projects_dir,
            hacks_dir,
        }
    }
}

impl Config {
    fn get_filepath() -> Result<std::path::PathBuf> {
        let mut config_dir =
            dirs::config_dir().ok_or_else(|| anyhow!("could not find config dir"))?;
        config_dir.push("hrs");
        Ok(config_dir)
    }

    pub fn load() -> Result<Config> {
        let mut config_file = Config::get_filepath()?;
        config_file.push("hrs.conf");
        let config_data = read_to_string(&config_file)
            .with_context(|| format!("Failed to read config file from {:?}", &config_file))?;
        toml::from_str(&config_data)
            .with_context(|| format!("Could not read config data from string: {}", &config_data))
    }

    pub fn write(&self) -> Result<()> {
        let mut config_file = Config::get_filepath()?;
        DirBuilder::new()
            .recursive(true)
            .create(config_file.clone())?;
        config_file.push("hrs.conf");
        let mut file_handle = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(config_file)?;
        file_handle.write_all(toml::to_string_pretty(&self)?.as_bytes())?;
        file_handle.flush()?;
        Ok(())
    }
}
