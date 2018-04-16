#[macro_use]
extern crate serde_derive;

extern crate serde;
extern crate serde_json;
extern crate byteorder;

use std::net::{TcpStream, Shutdown};
use std::fs::{File, OpenOptions};
use std::path::Path;
use std::io;
use std::io::{BufWriter,BufReader,BufRead,Write,Read};
use byteorder::{BigEndian, ReadBytesExt};
use std::thread;
use std::sync::mpsc::{self, TryRecvError};

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

enum RobotState {
    Undef = -1,
	Idle = 0,
	Think = 1,
	Fwd = 2,
	Rev = 3,
	Left = 4,
	Right = 5,
	Charging = 6,
    Daijuing = 7
}

impl From<i8> for RobotState {
    fn from(t:i8) -> RobotState {
        match t {
            -1 => RobotState::Undef,
            0 => RobotState::Idle,
            1 => RobotState::Think,
            2 => RobotState::Fwd,
            3 => RobotState::Rev,
            4 => RobotState::Left,
            5 => RobotState::Right,
            6 => RobotState::Charging,
            7 => RobotState::Daijuing,
            _ => panic!("Could not find RobotState")
        }
    }
}

fn main() {

    // Read configuration file ----- 
    
    print!("Reading config file...");

    let mut running = true;

    let config_path = "config/config";

    let mut config:Config;

    if Path::new(config_path).exists() {
        let mut config_file = match File::open(config_path) {
            Ok(s) => s,
            Err(_) => { panic!("Error: Config file found, but unable to open"); }
        };

        let mut config_data = String::new();

        match config_file.read_to_string(&mut config_data) {
            Ok(_) => {},
            Err(_) => { panic!("Error: Unable to read config file"); }
        };

        config = match serde_json::from_str(&config_data) {
            Ok(s) => s,
            Err(_) => panic!("Error: Unable to deserialize config file")
        };

        print!("SUCCESS\n");

    } else {
        // TODO Launch setup tool?
        panic!("Configuration file not found. Please use the setup tool.");
    }

    // If the `name` field is empty, assume that it hasn't been configured?
    match config.name.is_empty() {
        // TODO Launch setup tool?
        true => panic!("Error: Possible missing fields in config file."),
        false => {} 
    }

    // Check connection
    let robo_addr = config.robot_address.to_owned() + ":" + &config.robot_port;

    print!("Testing connecting to robot at {}...", robo_addr);
    let robo_stream = match TcpStream::connect(robo_addr.as_str()) {
        Ok(s) => s,
        Err(_) => panic!("Failed to connect to Robot")
    };

    print!("SUCCESS\n");

    robo_stream.shutdown(Shutdown::Both).unwrap();

    let mut io_writer = BufWriter::new(io::stdout());
    let mut io_reader = BufReader::new(io::stdin());
    let mut io_buf = String::new();

    print!("\n");
    while running {
        io_buf.clear();

        io_writer.write("> ".as_bytes()).unwrap();
        io_writer.flush().unwrap();

        io_reader.read_line(&mut io_buf).unwrap();

        // Remove trailing newline
        io_buf.pop();
        let input: Vec<&str> = io_buf.split(" ").collect();

        match input[0] {
            "quit" => { println!("Bye!"); running = false; },
            "help" => handle_help(),
            "listen" => handle_listen(&mut config),
            "free" => handle_free(&mut config),
            "localize" => handle_localize(&mut config),
            "stop" => handle_stop(&mut config),
            "save" => {
                if input.len() == 2 {
                    match input[1] {
                        "a" => handle_save_location("a", &mut config),
                        "b" => handle_save_location("b", &mut config),
                        s => println!("Unknown location: {}", s),
                    }
                } else {
                    println!("Command 'save' takes 1 parameter");
                }
            },
            "goto" => {
                if input.len() == 2 {
                    match input[1] {
                        "a" => handle_goto_location("a", &mut config),
                        "b" => handle_goto_location("b", &mut config),
                        s => println!("Unknown location: {}", s),
                    }
                } else {
                    println!("Command 'goto' takes 1 parameter");
                }

            },
            //"location" => handle_location(robo_stream),
            s => println!("Unknown command: {}", s),
        }

    }
}

fn handle_help() {
    println!("Current implemented commands:

    quit        Terminates the program
    help        Prints this help message 

    listen      Streams information broadcasted by the robot (still needs work).
                Press [Enter] to stop the stream.
    

    free        Will unlock the wheels of the robot, to be able to freely move it around
    stop        Will tell the robot to stop whatever it is currently doing

    save [a|b]  Saves robots current coordinates as location A or B
    goto [a|b]  Will try to route to location A or B respectively

    "); 
}

fn handle_listen(config:&mut Config) { 
    // Check connection
    let robo_addr = config.robot_address.to_owned() + ":" + &config.robot_port;
    let mut stream = match TcpStream::connect(robo_addr.as_str()) {
        Ok(s) => s,
        Err(_) => panic!("Failed to connect to Robot")
    };

    println!("Listening to robot.. Press [Enter] to stop listening\n");

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        loop {
            // Read from robot
            let mut cmd_buf = [0; 3];
            stream.read_exact(&mut cmd_buf).unwrap();
            let cmd = cmd_buf[0];
            let len: i32 = ((cmd_buf[1] as i32) << 8) | (cmd_buf[2] as i32);

            let mut buf: Vec<u8> = vec![0;len as usize];
            stream.read_exact(&mut buf[..]).unwrap();

            match cmd {
                130 => { // Position
                    let mut buf_angle = &buf[0..2];
                    let mut buf_x = &buf[2..6];
                    let mut buf_y = &buf[6..10];

                    let angle = buf_angle.read_i16::<BigEndian>().unwrap();
                    let x = buf_x.read_i32::<BigEndian>().unwrap();
                    let y = buf_y.read_i32::<BigEndian>().unwrap();

                    let real_angle = (angle as f32) / 65536.0 * 360.0;

                    println!("[130:{}] Location: x={}, y={}, angle={}", len, x, y, real_angle);
                },
                139 => { // State
                    let state_num = buf[0];
                    //let state = RobotState::from(buf[0] as i8);
                    println!("[139:{}] {}", len, state_num); 
                },
                140 => { // No idea what this is.. Possible the size of the robot?
                    let mut buf_xs = &buf[0..2];
                    let mut buf_ys = &buf[2..4];
                    let mut buf_xoffs = &buf[4..6];
                    let mut buf_yoffs = &buf[6..8];

                    let xs = buf_xs.read_i16::<BigEndian>().unwrap();
                    let ys = buf_ys.read_i16::<BigEndian>().unwrap();
                    let xoffs = buf_xoffs.read_i16::<BigEndian>().unwrap();
                    let yoffs = buf_yoffs.read_i16::<BigEndian>().unwrap();

                    println!("[140:{}] Something fetched: ({}, {}, {}, {})", len, xs, ys, xoffs, yoffs);
                },
                134 => { // Battery
                    let mut charging:bool = false;
                    let mut finished:bool = false;

                    let buf_charging = buf[0]&1;
                    let buf_finished = buf[0]&2; // Finished charging
                    let buf_voltage = ((buf[1] as i32) << 8) | (buf[2] as i32)/1000;
                    let percentage = buf[3];

                    if buf_charging == 1 {
                        charging = true;
                    }

                    if buf_finished == 1 {
                        finished = true;
                    }

                    println!("[134:{}] Battery {}% (charging={} finished={} voltage={}", len, percentage, charging, finished, buf_voltage);
                }
                _ => {}
            }

            match rx.try_recv() {
                Ok(_) | Err(TryRecvError::Disconnected) => {
                    stream.shutdown(Shutdown::Both).unwrap();
                    break;
                }
                Err(TryRecvError::Empty) => {}
            }
        }
    });

    let mut buf = String::new();
    let reader = io::stdin();
    let _ = reader.lock().read_line(&mut buf);
    let _ = tx.send(());
}

fn handle_free(config:&mut Config) {

    let robo_addr = config.robot_address.to_owned() + ":" + &config.robot_port;
    let mut stream = match TcpStream::connect(robo_addr.as_str()) {
        Ok(s) => s,
        Err(_) => panic!("Failed to connect to Robot")
    };

    stream.set_write_timeout(None).expect("set_write_timeout call failed");
    
    let mut buf = [0; 4];

    buf[0] = 58;
    buf[1] = 0;
    buf[2] = 1;
    buf[3] = 5;
    
    match stream.write_all(&mut buf) {
        Ok(_) => {},
        Err(e) => println!("{:?}", e)
    };

    stream.shutdown(Shutdown::Both).unwrap();
}

fn handle_goto_location(location:&str, config:&mut Config) {

    let robo_addr = config.robot_address.to_owned() + ":" + &config.robot_port;
    let mut stream = match TcpStream::connect(robo_addr.as_str()) {
        Ok(s) => s,
        Err(_) => panic!("Failed to connect to Robot")
    };
    //let mut stream = TcpStream::connect("192.168.43.23:22222").unwrap();
    stream.set_nonblocking(false).expect("set_nonblocking failed");
    
    let mut buf = [0; 12];
    let x:i32;
    let y:i32;

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

    match stream.write_all(&mut buf) {
        Ok(_) => {},
        Err(e) => println!("{:?}", e)
    };

    stream.shutdown(Shutdown::Both).unwrap();
}

fn handle_localize(config:&mut Config) {
    let robo_addr = config.robot_address.to_owned() + ":" + &config.robot_port;
    let mut stream = match TcpStream::connect(robo_addr.as_str()) {
        Ok(s) => s,
        Err(_) => panic!("Failed to connect to Robot")
    };

    stream.set_write_timeout(None).expect("set_write_timeout call failed");
    
    let mut buf = [0; 4];

    buf[0] = 58;
    buf[1] = 0;
    buf[2] = 1;
    buf[3] = 3;
    
    match stream.write_all(&mut buf) {
        Ok(_) => {},
        Err(e) => println!("{:?}", e)
    };

    stream.shutdown(Shutdown::Both).unwrap();
}

fn handle_stop(config:&mut Config) {
    let robo_addr = config.robot_address.to_owned() + ":" + &config.robot_port;
    let mut stream = match TcpStream::connect(robo_addr.as_str()) {
        Ok(s) => s,
        Err(_) => panic!("Failed to connect to Robot")
    };

    stream.set_write_timeout(None).expect("set_write_timeout call failed");
    
    let mut buf = [0; 4];

    buf[0] = 58;
    buf[1] = 0;
    buf[2] = 1;
    buf[3] = 8;
    
    match stream.write_all(&mut buf) {
        Ok(_) => {},
        Err(e) => println!("{:?}", e)
    };

    stream.shutdown(Shutdown::Both).unwrap();
}

fn handle_save_location(location:&str, config:&mut Config) {

    println!("Saving location..");
    let x:i32;
    let y:i32;

    let robo_addr = config.robot_address.to_owned() + ":" + &config.robot_port;
    let mut stream = match TcpStream::connect(robo_addr.as_str()) {
        Ok(s) => s,
        Err(_) => panic!("Failed to connect to Robot")
    };

    // Read current location
    loop {
        let mut cmd_buf = [0; 3];
        stream.read_exact(&mut cmd_buf).unwrap();

        let len: i32 = ((cmd_buf[1] as i32) << 8) | (cmd_buf[2] as i32);

        let mut buf: Vec<u8> = vec![0;len as usize];
        stream.read_exact(&mut buf[..]).unwrap();

        if cmd_buf[0] == 130 {

            let mut buf_x = &buf[2..7];
            let mut buf_y = &buf[6..11];

            x = buf_x.read_i32::<BigEndian>().unwrap();
            y = buf_y.read_i32::<BigEndian>().unwrap();

            break;
        }
    }

    stream.shutdown(Shutdown::Both).unwrap();

    if location == "a" {
        config.location_a_x = Some(x);
        config.location_a_y = Some(y);
    } else if location == "b" {
        config.location_b_x = Some(x);
        config.location_b_y = Some(y);
    }

    // Write to config file 
    let config_path = "config/config";
    let config_file = match OpenOptions::new().write(true).truncate(true).open(config_path) {
        Ok(s) => s,
        Err(e) => panic!("{:?}", e),
    };

    //match config_file.write_all(new_config.as_bytes()) {
    match serde_json::ser::to_writer_pretty(&config_file, &config) {
        Ok(_) => println!("Saved location {}: x={}, y={}", location.to_uppercase(), x, y),
        Err(e) => println!("{:?}", e)
    };

    config_file.sync_all().unwrap();
}


