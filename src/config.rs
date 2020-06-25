use std::collections::HashMap;
use serde_yaml;
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub commands: HashMap<String, AppCommand>,
    pub hosts: HashMap<String, AppHost>,
}

#[derive(Debug, Deserialize)]
pub struct AppCommand {
    pub command: String,
}

#[derive(Debug, Deserialize)]
pub struct AppHost {
    pub ip: String,
    pub user: String,
}

pub fn load_config_file(config_file: &str) -> crate::AppResult<()> {
    let config_file = std::fs::File::open(config_file)?;
    let parsed_config: AppConfig = serde_yaml::from_reader(config_file)?;

    println!("Cfg: {:?}", &parsed_config);

    Ok(())
}
