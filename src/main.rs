/*
 * Console Client for Pulu Robot
 * Author: Brian Alberg <brian@alberg.org>
 */


#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate byteorder;

mod pulurobot;

use pulurobot::{Robot, PuluRobot};
use std::net::{TcpStream, Shutdown};
use std::io;
use std::io::{BufWriter,BufReader,BufRead,Write,Read};
use byteorder::{BigEndian, ReadBytesExt};
use std::thread;
use std::sync::mpsc::{self, TryRecvError};

fn main() {

    let mut running = true;

    let mut io_writer = BufWriter::new(io::stdout());

    // Setup and test connection
    io_writer.write("Testing connection to robot...".as_bytes()).unwrap();
    io_writer.flush().unwrap();

    let mut robot = match Robot::from_config("config/config") {
        Ok(s) => { 
            io_writer.write("OK\n".as_bytes()).unwrap(); 
            io_writer.flush().unwrap();
            s
        },
        Err(_) => {
            io_writer.write("FAILED\n".as_bytes()).unwrap();
            io_writer.flush().unwrap();
            panic!("Unable to connect to robot")
        }
    };

    robot.disconnect();

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

        // Handle input commands
        match input[0] {
            "quit" => { println!("Bye!"); running = false; },
            "help" => handle_help(),
            "listen" => handle_listen(&mut robot), 
            "free" => {
                match robot.free() {
                    Ok(_) => (),
                    Err(_) => println!("Unable to send command to robot"),
                }
            },
            "localize" => {
                match robot.localize() {
                    Ok(_) => (),
                    Err(_) => println!("Unable to send command to robot"),
                }
            },
            "stop" => {
                match robot.stop() {
                    Ok(_) => (),
                    Err(_) => println!("Unable to send command to robot"),
                }
            },
            "save" => {
                if input.len() == 2 {
                    match input[1] {
                        "a" => robot.save_location("a"),
                        "b" => robot.save_location("b"),
                        s => {
                            println!("Unknown location: {}", s);
                            Ok(())
                        },
                    };
                } else {
                    println!("Command 'save' takes 1 parameter");
                }
            },
            "goto" => {
                if input.len() == 2 {
                    match input[1] {
                        "a" => robot.goto_point("a"),
                        "b" => robot.goto_point("b"),
                        s => {
                            println!("Unknown location: {}", s);
                            Ok(())
                        },
                    };
                } else {
                    println!("Command 'goto' takes 1 parameter");
                }
            },
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

fn handle_listen(robot:&mut Robot) { 
    // Check connection
    let robo_addr = robot.config.robot_address.to_owned() + ":" + &robot.config.robot_port;
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
                },
                138 => { // 3D TOF HMAP
                    println!("[{}:{}] 3D TOF HMAP", cmd, len);
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
                _ => {
                    println!("[{}:{}] Unhandled command", cmd, len);
                }
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

