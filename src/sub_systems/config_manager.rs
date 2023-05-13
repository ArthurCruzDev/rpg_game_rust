use ggez::Context;
use log::*;
use serde::{Deserialize, Serialize};
use std::{fs::File, io::Read, io::Write, path::Path, process::exit};

const CONFIG_FILE_NAME: &str = "/config.cfg";
const CONFIG_FILE_PATH: &str = "./";

#[derive(Serialize, Deserialize, Debug)]
pub struct Resolution {
    width: f32,
    height: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigManager {
    fullscreen_on: bool,
    resolution: Resolution,
    music_volume: f32,
    se_volume: f32,
}

impl Default for ConfigManager {
    fn default() -> Self {
        ConfigManager {
            fullscreen_on: false,
            resolution: Resolution {
                width: 800f32,
                height: 600f32,
            },
            music_volume: 100f32,
            se_volume: 100f32,
        }
    }
}

impl ConfigManager {
    pub fn read_configuration_from_file() -> ConfigManager {
        let resouce_path: String = CONFIG_FILE_PATH.to_owned();
        match File::open(resouce_path + CONFIG_FILE_NAME) {
            Ok(mut file) => {
                let mut buffer = Vec::new();
                match file.read_to_end(&mut buffer) {
                    Ok(read_buffer) => {
                        info!("read {} bytes from config file", read_buffer);
                        let read_config: ConfigManager =
                            match toml::from_slice::<ConfigManager>(&buffer) {
                                Ok(read_config) => read_config,
                                Err(error) => {
                                    error!("Failed to parse configuration file: {}", error);
                                    let default = ConfigManager::default();
                                    ConfigManager::write_configuration_to_file(&default);
                                    default
                                }
                            };
                        read_config
                    }
                    Err(error) => {
                        error!(
                            "An error has occurred while trying to read the game's config file: {}",
                            error.to_string()
                        );
                        exit(-1);
                    }
                }
            }
            Err(error) => match error.kind() {
                std::io::ErrorKind::NotFound => {
                    let default = ConfigManager::default();
                    ConfigManager::write_configuration_to_file(&default);
                    default
                }
                e => {
                    info!("Error: {}", e.to_string());
                    exit(-1);
                }
            },
        }
    }

    pub fn write_configuration_to_file(config_manager: &ConfigManager) {
        let mut file: std::fs::File;
        let resouce_path: String = CONFIG_FILE_PATH.to_owned();
        match Path::new((resouce_path.clone() + CONFIG_FILE_NAME).as_str()).exists() {
            true => {
                match std::fs::OpenOptions::new()
                    .write(true)
                    .append(false)
                    .read(false)
                    .create(false)
                    .open((resouce_path + CONFIG_FILE_NAME).as_str())
                {
                    Ok(opened_file) => {
                        file = opened_file;
                    }
                    Err(error) => {
                        error!("Failed to open existing config file: {}", error.to_string());
                        exit(-1);
                    }
                }
            }
            false => match std::fs::OpenOptions::new()
                .write(true)
                .append(false)
                .read(false)
                .create(true)
                .open((resouce_path + CONFIG_FILE_NAME).as_str())
            {
                Ok(opened_file) => {
                    file = opened_file;
                }
                Err(error) => {
                    error!("Failed to create new config file: {}", error.to_string());
                    exit(-1);
                }
            },
        }

        let toml_value = match toml::Value::try_from(config_manager) {
            Ok(value) => value,
            Err(error) => {
                error!("Failed to convert config info to TOML: {}", error);
                exit(-1);
            }
        };

        let config_string = match toml::to_string_pretty(&toml_value) {
            Ok(created_string) => created_string,
            Err(error) => {
                error!("Failed to serialize configuration: {}", error);
                exit(-1);
            }
        };
        match file.write_all(config_string.as_bytes()) {
            Ok(()) => {
                info!("Successfully written config file");
            }
            Err(error) => {
                error!("Failed to write config file: {}", error);
                exit(-1);
            }
        }
    }
}
