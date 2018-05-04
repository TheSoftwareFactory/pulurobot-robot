use super::{Config, ConfigHandler, ConfigError, ConfigErrorType, ConfigErrorHandler};

use std::path::Path;
use std::fs::{File, OpenOptions};
use std::io::Read;
use serde_json;

impl ConfigHandler for Config {
    fn new() -> Self {
        return Config {
            name: String::new(),
            manufacturer: String::new(),
            robot_address: String::new(),
            robot_port: String::new(),
            point_a_x: 0,
            point_a_y: 0,
            point_b_x: 0,
            point_b_y: 0,
        }
    }

    fn create(config_path:&str) -> Result<Self, ConfigError> {
        let mut config = Config::new();
        match File::create(config_path) {
            Ok(_) => {
                // Fill file with default values
                match config.write(config_path) {
                    Ok(_) => Ok(config),
                    Err(_) => panic!("Unable to write to configuration file"),
                }
            },
            Err(_) => panic!("Unable to create configuration file"),
        }
    }

    fn from_file(self, config_path:&str) -> Result<Self, ConfigError> {

        let config:Config;

        if Path::new(config_path).exists() {
            let mut config_file = match File::open(config_path) {
                Ok(s) => s,
                Err(_) => { return Err( ConfigError::new(ConfigErrorType::FileNotFound) ) }
            };

            let mut config_data = String::new();

            match config_file.read_to_string(&mut config_data) {
                Ok(_) => {},
                Err(_) => { return Err( ConfigError::new(ConfigErrorType::Read) ) }
            };

            config = match serde_json::from_str(&config_data) {
                Ok(s) => s,
                Err(_) => { return Err( ConfigError::new(ConfigErrorType::Deserialization) ) }
            };

        } else {
            return Err( ConfigError::new(ConfigErrorType::FileNotFound) );
        }

        return Ok(config);
    }

    fn write(&mut self, config_path: &str) -> Result<(), ConfigError> {

        let config_file = match OpenOptions::new().write(true).truncate(true).open(config_path) {
            Ok(s) => s,
            Err(_) => { return Err( ConfigError::new(ConfigErrorType::Open) ) }
        };

        match serde_json::ser::to_writer_pretty(&config_file, &self) {
            Ok(_) => println!("Written config file"),
            Err(_) => { return Err( ConfigError::new(ConfigErrorType::Serialization) ) }
        };

        config_file.sync_all().unwrap();

        return Ok(());
    }

    fn set_point(&mut self, name: &str, x: i32, y: i32) -> Result<(), ConfigError> {

        if name == "a" {
            self.point_a_x = x;
            self.point_a_y = y;
        } else if name == "b" {
            self.point_b_x = x;
            self.point_b_y = y;
        }

        return Ok(());
    }

    fn get_point(&mut self, name: &str) -> Result<(i32,i32), ConfigError> {

        let x:i32;
        let y:i32;

        if name == "a" {
            x = self.point_a_x;
            y = self.point_a_y;
        } else if name == "b" {
            x = self.point_b_x;
            y = self.point_b_y;
        } else {
            return Err( ConfigError::new(ConfigErrorType::Read) );
        }

        return Ok((x,y))
    }
}



