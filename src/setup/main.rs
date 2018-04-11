/*
 * Setup configuration for the robot
 * Author: Brian Alberg <brian@alberg.org>
 *
 */

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

use std::fs::File;
use std::io::{BufWriter,Write,Read};
use std::io;
use std::path::Path;
use std::net::TcpStream;

#[derive(Deserialize, Debug, Default)]
struct Config {
    name: String,
    manufacturer: String,
    server_ip: String,
    private_key: String,
    robot_port: String,
    location_a: String,
    location_b: String,
    location_c: String
}


fn main() {

    println!("Launching Robo-Setup");

    let mut writer = BufWriter::new(io::stdout());
    let config_path = "config/config";
    let mut config:Config = Default::default();

    let config_file = match File::create(config_path) {
        Ok(s) => s,
        Err(_) => panic!("Unable to create configuration file"),
    };

    let reader = io::stdin();

    writer.write("Name of robot: ".as_bytes()).unwrap();
    writer.flush().unwrap();

    reader.read_line(&mut config.name);
    config.name.pop(); // Remove trailing newline

    writer.write("Manufacturer: ".as_bytes()).unwrap();
    writer.flush().unwrap();

    reader.read_line(&mut config.manufacturer);
    config.manufacturer.pop(); // Remove trailing newline

    writer.write("Server IP [192.168.88.162]: ".as_bytes()).unwrap();
    writer.flush().unwrap();

    reader.read_line(&mut config.server_ip);
    config.server_ip.pop(); // Remove trailing newline

    if config.server_ip.is_empty() {
        config.server_ip = String::from("192.168.88.162");
    }

    /*writer.write("Private key: ".as_bytes()).unwrap();
    writer.flush().unwrap();

    reader.read_line(&mut config.private_key);
    config.private_key.pop(); // Remove trailing newline*/

    writer.write("Robot TCP Port [22222]: ".as_bytes()).unwrap();
    writer.flush().unwrap();

    reader.read_line(&mut config.robot_port);
    config.robot_port.pop(); // Remove trailing newline

    if config.robot_port.is_empty() {
        config.robot_port = String::from("22222");
    }

    writer.write("Connecting to robot...".as_bytes()).unwrap();
    writer.flush().unwrap();

    let server_ip = config.server_ip.push(':');
    let robo_addr = config.server_ip + &config.robot_port;
    
    let mut robo_stream = match TcpStream::connect(robo_addr.as_str()) {
        Ok(s) => s,
        Err(_) => panic!("Failed to connect to Robot")
    };

    writer.write("SUCCESS\n".as_bytes()).unwrap();
    writer.flush().unwrap();

    // Now for the location information 
    // Return current location 
    let mut send_str: Vec<u8> = vec![59,2,10];

    send_str.push(59);
    send_str.push(2);
    send_str.push(10);
    
    match robo_stream.write(&[59,2,10]) {
    //match robot_stream.write(send_str) {
        Ok(s) => println!("Sent {} bytes of something", s),
        Err(_) => println!("Unable to write to robot TCP connection")
    }

    /*let mut location_a = String::new();
    match robo_stream.read_to_string(&mut location_a) {
        Ok(s) => println!("Received {} bytes: {}", s, location_a),
        Err(_) => println!("Problem receiving")
    }*/


}
