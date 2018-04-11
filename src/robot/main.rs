#[macro_use]
extern crate serde_derive;

//extern crate ws;
extern crate serde;
extern crate serde_json;
extern crate byteorder;

use std::fs::OpenOptions;
use std::net::TcpStream;
use std::fs::File;
use std::path::Path;
use std::io;
use std::io::{BufWriter,BufReader,BufRead,Write,Read};
use byteorder::{BigEndian, ReadBytesExt};
//use std::io::prelude::*;

//use ws::*;

#[derive(Deserialize, Serialize, Debug)]
struct Config {
    name: String,
    manufacturer: String,
    //server_address: String,
    //server_port: String,
    //private_key: String,
    robot_address: String,
    robot_port: String,
    location_a_x: Option<i32>,
    location_a_y: Option<i32>,
    location_b_x: Option<i32>,
    location_b_y: Option<i32>,
}

fn main() {

    // Read configuration file ----- 
    
    println!("Reading config file");

    let mut running = true;

    let config_path = "config/config";

    let mut config:Config;

    if Path::new(config_path).exists() {
        let mut config_file = File::open(config_path).unwrap();
        let mut config_data = String::new();
        config_file.read_to_string(&mut config_data).unwrap();

        config = serde_json::from_str(&config_data).unwrap();

    } else {
        // TODO Launch setup tool
        panic!("Configuration file not found. Please use the setup tool.");
    }


    // If the `name` field is empty, assume that it hasn't been configured?
    match config.name.is_empty() {
        // TODO Launch setup tool
        true => panic!("Misconfiguration error. Please use the setup tool."),
        false => println!("Robot name: {}", config.name)
    }

    // Establish connection with host (robot) through TCP
    
    let robo_addr = config.robot_address.to_owned() + ":" + &config.robot_port;

    println!("Connecting to robot at {}", robo_addr);
    let robo_stream = match TcpStream::connect(robo_addr.as_str()) {
        Ok(s) => s,
        Err(_) => panic!("Failed to connect to Robot")
    };

    println!("Connected!\n");

    let mut io_writer = BufWriter::new(io::stdout());
    let mut io_reader = BufReader::new(io::stdin());
    let mut io_buf = String::new();

    while running {
        io_writer.write("> ".as_bytes()).unwrap();
        io_writer.flush().unwrap();

        io_reader.read_line(&mut io_buf).unwrap();

        // Remove trailing newline
        io_buf.pop();

        match io_buf.as_ref() {
            "quit" => { println!("Bye!"); running = false; },
            "help" => handle_help(),
            "listen" => handle_listen(&robo_stream),
            "save a" => handle_save_location("a", &mut config, &robo_stream),
            "save b" => handle_save_location("b", &mut config, &robo_stream),
            "goto a" => handle_goto_location("a", &mut config, &robo_stream),
            "goto b" => handle_goto_location("b", &mut config, &robo_stream),
            //"location" => handle_location(robo_stream),
            s => println!("Unknown command: {}", s),
        }

        io_buf.clear()

    }


    /*match robo_stream.write(b"Hello world") {
        Ok(s) => println!("Sent {} bytes of something", s),
        Err(_) => println!("Unable to write to robot TCP connection")
    }*/


    // Establish connection with server
    /*listen("127.0.0.1:3002", |out| {
        move |msg| {
            out.send(msg)
        }
    })*/
}

fn handle_help() {
    println!("Current implemented commands:

    quit    Terminates the program
    help    Prints this help message 
    listen  Prints information broadcasted by the robot (only partially working)
    "); 
}

fn handle_listen(mut stream:&TcpStream) { 
    loop {
        let mut cmd_buf = [0; 3];
        stream.read_exact(&mut cmd_buf);
        let cmd = cmd_buf[0];
        let len: i32 = ((cmd_buf[1] as i32) << 8) | (cmd_buf[2] as i32);

        let mut buf: Vec<u8> = vec![0;len as usize];
        stream.read_exact(&mut buf[..]);

        match cmd {
            130 => {
                let mut buf_angle = &buf[0..2];
                let mut buf_x = &buf[2..7];
                let mut buf_y = &buf[6..11];

                let angle = buf_angle.read_i16::<BigEndian>().unwrap();
                let x = buf_x.read_i32::<BigEndian>().unwrap();
                let y = buf_y.read_i32::<BigEndian>().unwrap();

                let real_angle = (angle as f32) / 65536.0 * 360.0;

                println!("[130:{}] Location: x={}, y={}, angle={}", len, x, y, real_angle);
            },
            140 => {
                println!("Some state was fetched");
            },
            _ => {}
        }
    }
}

fn handle_goto_location(location:&str, config:&mut Config, mut stream:&TcpStream) {
    
    let mut buf = [0; 12];
    let mut x:i32 = 0;
    let mut y:i32 = 0;

    if location == "a" {
        x = match config.location_a_x {
            Some(ax) => ax,
            None => panic!("No x coordinate for location A found in config"),
        };

         y = match config.location_a_y {
            Some(ay) => ay,
            None => panic!("No y coordinate for location A found in config"),
        };

    } else if location == "b" {
        x = match config.location_b_x {
            Some(bx) => bx,
            None => panic!("No x coordinate for location B found in config"),
        };

        y = match config.location_b_y {
            Some(by) => by,
            None => panic!("No y coordinate for location B found in config"),
        };
    } else {
        println!("Unknown location");
        return;
    }

    buf[0] = 56;

    buf[1] = 0;
    buf[2] = 9;

    buf[3] = (x>>24) as u8;
    buf[4] = (x>>16) as u8;
    buf[5] = (x>>8) as u8;
    buf[6] = (x>>0) as u8;

    buf[7] = (y>>24) as u8;
    buf[8] = (y>>16) as u8;
    buf[9] = (y>>8) as u8;
    buf[10] = (y>>0) as u8;

    buf[11] = 0;

    stream.write(&mut buf).unwrap();

}

fn handle_save_location(location:&str, config:&mut Config, mut stream:&TcpStream) {

    println!("Saving location..");
    let mut x:i32 = 0;
    let mut y:i32 = 0;

    // Read current location
    loop {
        let mut cmd_buf = [0; 3];
        stream.read_exact(&mut cmd_buf);

        let cmd = cmd_buf[0];
        let len: i32 = ((cmd_buf[1] as i32) << 8) | (cmd_buf[2] as i32);

        let mut buf: Vec<u8> = vec![0;len as usize];
        stream.read_exact(&mut buf[..]);

        if cmd_buf[0] == 130 {

            let mut buf_x = &buf[2..7];
            let mut buf_y = &buf[6..11];

            x = buf_x.read_i32::<BigEndian>().unwrap();
            y = buf_y.read_i32::<BigEndian>().unwrap();

            break;
        }
    }

    if location == "a" {
        config.location_a_x = Some(x);
        config.location_a_y = Some(y);
    } else if location == "b" {
        config.location_b_x = Some(x);
        config.location_b_y = Some(y);
    }

    // Serialize to json
    let new_config = serde_json::to_string(&config).unwrap();

    // Write to config file 
    let config_path = "config/config";
    let mut config_file = match OpenOptions::new().write(true).truncate(true).open(config_path) {
        Ok(s) => s,
        Err(e) => panic!("{:?}", e),
    };

    match config_file.write_all(new_config.as_bytes()) {
        Ok(_) => println!("Saved location {}: x={}, y={}", location.to_uppercase(), x, y),
        Err(e) => println!("{:?}", e)
    };

    config_file.sync_all();
}
