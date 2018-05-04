/*
 * Setup configuration for the robot
 * Author: Brian Alberg <brian@alberg.org>
 */

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate byteorder;

mod pulurobot;

use std::io::{BufWriter,Write};
use std::io;
use std::path::Path;

use pulurobot::{Config, ConfigHandler};

fn main() {

    println!("Launching PuluRobot Setup");

    let mut writer = BufWriter::new(io::stdout());
    let config_path = "config/config";
    let mut config:Config = Config::new();

    if Path::new(config_path).exists() {
        config = match config.from_file(config_path) {
            Ok(s) => s,
            Err(_) => {
                match Config::create(config_path) {
                    Ok(c) => c,
                    Err(_) => panic!("Unable to create configuration file"),
                }
            },
        }
    } else {
        // Create config file
        config = match Config::create(config_path) {
            Ok(s) => s,
            Err(_) => panic!("Unable to create configuration file"),
        };
    }

    let reader = io::stdin();

    writer.write("Name of robot: ".as_bytes()).unwrap();
    writer.flush().unwrap();

    reader.read_line(&mut config.name).unwrap();
    config.name.pop(); // Remove trailing newline

    writer.write("Manufacturer: ".as_bytes()).unwrap();
    writer.flush().unwrap();

    reader.read_line(&mut config.manufacturer).unwrap();
    config.manufacturer.pop(); // Remove trailing newline

    writer.write("Robot IP [localhost]: ".as_bytes()).unwrap();
    writer.flush().unwrap();

    reader.read_line(&mut config.robot_address).unwrap();
    config.robot_address.pop(); // Remove trailing newline

    if config.robot_address.is_empty() {
        config.robot_address = String::from("localhost");
    }

    writer.write("Robot Port [22222]: ".as_bytes()).unwrap();
    writer.flush().unwrap();

    reader.read_line(&mut config.robot_port).unwrap();
    config.robot_port.pop(); // Remove trailing newline

    if config.robot_port.is_empty() {
        config.robot_port = String::from("22222");
    }

    writer.write("Writing to configuration file...".as_bytes()).unwrap();
    writer.flush().unwrap();
    match config.write(config_path) {
        Ok(_) => {         
            writer.write("SUCCESS\n".as_bytes()).unwrap();
            writer.flush().unwrap();

            println!("Robot Setup Completed. Have a nice day!");
        },
        Err(_) => {
            writer.write("FAILED\n".as_bytes()).unwrap();
            writer.flush().unwrap();

            println!("ERROR: Problem writing to configuration file.");
        }
    }
}
