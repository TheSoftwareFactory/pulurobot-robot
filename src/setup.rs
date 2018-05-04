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

use std::io::{BufWriter,Write,BufReader,BufRead};
use std::io;
use std::path::Path;

use pulurobot::{Config, ConfigHandler};

fn main() {

    println!("Launching PuluRobot Setup");

    let mut writer = BufWriter::new(io::stdout());
    let config_path = "config/config";
    let mut config:Config = Config::new();

    // Check if config file exists
    if Path::new(config_path).exists() {
        // Try to open
        config = match config.from_file(config_path) {
            Ok(s) => s,
            Err(_) => {
                // On fail, create new config file
                match Config::create(config_path) {
                    Ok(c) => c,
                    Err(_) => panic!("Unable to create configuration file"),
                }
            },
        }
    } else {
        // Create new config file
        config = match Config::create(config_path) {
            Ok(s) => s,
            Err(_) => panic!("Unable to create configuration file"),
        };
    }

    let mut reader = BufReader::new(io::stdin());
    let mut read_buffer = String::new();

    // Handle Name
    if config.name.is_empty() {
        writer.write("Name of robot: ".as_bytes()).unwrap();
    } else {
        writer.write((String::from("Name of robot [") + &config.name + "]: ").as_bytes()).unwrap();
    }
    writer.flush().unwrap();

    reader.read_line(&mut read_buffer).unwrap();
    read_buffer.pop(); // Remove trailing newline

    if !read_buffer.is_empty() {
        config.name = read_buffer.clone();  
    }

    read_buffer.clear();

    // Handle Manufacturer
    if config.manufacturer.is_empty() {
        writer.write("Manufacturer: ".as_bytes()).unwrap();
    } else {
        writer.write((String::from("Manufacturer: [") + &config.manufacturer + "]: ").as_bytes()).unwrap();
    }
    writer.flush().unwrap();

    reader.read_line(&mut read_buffer).unwrap();
    read_buffer.pop(); // Remove trailing newline

    if !read_buffer.is_empty() {
        config.manufacturer = read_buffer.clone();
    }

    read_buffer.clear();

    // Handle Robot IP
    if config.manufacturer.is_empty() {
        writer.write("Robot IP: ".as_bytes()).unwrap();
    } else {
        writer.write((String::from("Robot IP [") + &config.robot_address + "]: ").as_bytes()).unwrap();
    }
    writer.flush().unwrap();

    reader.read_line(&mut read_buffer).unwrap();
    read_buffer.pop(); // Remove trailing newline

    if !read_buffer.is_empty() {
        config.robot_address = read_buffer.clone();
    }

    read_buffer.clear();

    // Handle Robot Port
    if config.robot_port.is_empty() {
        writer.write("Robot Port: ".as_bytes()).unwrap();
    } else {
        writer.write((String::from("Robot Port [") + &config.robot_port + "]: ").as_bytes()).unwrap();
    }
    writer.flush().unwrap();

    reader.read_line(&mut read_buffer).unwrap();
    read_buffer.pop(); // Remove trailing newline

    if !read_buffer.is_empty() {
        config.robot_port = read_buffer.clone();
    }

    read_buffer.clear();

    // Write to config file
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
