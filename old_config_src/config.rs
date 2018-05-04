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
            location_a_x: 0,
            location_a_y: 0,
            location_b_x: 0,
            location_b_y: 0,
        }
    }

    fn from_file(self, config_path:&str) -> Result<Self, ConfigError> {

        //let config_path = "config/config";
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
            // TODO Launch setup tool?
            return Err( ConfigError::new(ConfigErrorType::FileNotFound) );
        }

        return Ok(config);
    }

    fn write(self) -> Result<(), ConfigError> {

        let config_path = "config/config";
        let config_file = match OpenOptions::new().write(true).truncate(true).open(config_path) {
            Ok(s) => s,
            Err(e) => panic!("{:?}", e),
        };

        //match config_file.write_all(new_config.as_bytes()) {
        match serde_json::ser::to_writer_pretty(&config_file, &self) {
            Ok(_) => println!("Written config file"),
            Err(e) => println!("{:?}", e)
        };

        config_file.sync_all().unwrap();

        return Ok(());
    }

    fn set_location(mut self, name: &str, x: i32, y: i32) -> Result<(), ConfigError> {

        if name == "a" {
            self.location_a_x = x;
            self.location_a_y = y;
        } else if name == "b" {
            self.location_b_x = x;
            self.location_b_y = y;
        }

        /*let config_path = "config/config";
        let config_file = match OpenOptions::new().write(true).truncate(true).open(config_path) {
            Ok(s) => s,
            Err(e) => panic!("{:?}", e),
        };*/

        //match config_file.write_all(new_config.as_bytes()) {
        /*match serde_json::ser::to_writer_pretty(&config_file, &config) {
            Ok(_) => println!("Saved location {}: x={}, y={}", location.to_uppercase(), x, y),
            Err(e) => println!("{:?}", e)
        };

        config_file.sync_all().unwrap();*/

        return Ok(());
    }

    // TODO not working, possibly just remove it
    /*fn get_location(self, name: &str) -> Result<(i32,i32), ConfigError> {

        let x:i32;
        let y:i32;

        if name == "a" {
            x = match self.location_a_x {
                Some(ax) => ax,
                None => panic!("No x coordinate for location A found in config"),
            };

             y = match self.location_a_y {
                Some(ay) => ay,
                None => panic!("No y coordinate for location A found in config"),
            };

        } else if name == "b" {
            x = match self.location_b_x {
                Some(bx) => bx,
                None => panic!("No x coordinate for location B found in config"),
            };

            y = match self.location_b_y {
                Some(by) => by,
                None => panic!("No y coordinate for location B found in config"),
            };
        } else {
            println!("Unknown location");
            return Err( ConfigError::new(ConfigErrorType::Read) );
        }

        return Ok((x,y))
    }*/
}




