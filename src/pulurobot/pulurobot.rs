use super::*;
use std::net::{TcpStream, Shutdown};
use std::time::Duration;
use std::io::{Read, Write};
use byteorder::{BigEndian, ReadBytesExt};

impl PuluRobot for Robot {
    fn connect(&mut self) -> Result<(), RobotError> {
        let robo_addr = (self.config.robot_address.to_owned() + ":" + &self.config.robot_port).parse().unwrap();
        self.stream = match TcpStream::connect_timeout(&robo_addr, Duration::new(5,0)) {
            Ok(s) => s,
            Err(_) => { return Err( RobotError::new(RobotErrorType::Connection) ) } 
        };
        return Ok(())
    }

    fn from_config(config_path: &str) -> Result<Robot, RobotError> {

        let mut config = Config::new();

        config = match config.from_file(config_path) {
            Ok(s) => s,
            Err(_) => { panic!("Problems reading from config file"); }
        };

        let robo_addr = (config.robot_address.to_owned() + ":" + &config.robot_port).parse().unwrap();
        match TcpStream::connect_timeout(&robo_addr, Duration::new(5,0)) {
            Ok(s) => {
                let robot = Robot {
                    stream: s,
                    config_path: String::from(config_path),
                    config: config,
                };
                return Ok(robot);
            },
            Err(_) => Err( RobotError::new(RobotErrorType::Connection) )
        }
    }

    fn disconnect(&mut self) {
        self.stream.shutdown(Shutdown::Both).unwrap();
    }

    fn get_location(&mut self) -> Result<RobotLocation, RobotError> {
        let x:i32;
        let y:i32;

        match self.connect() {
            Ok(_) => {},
            Err(_) => { return Err( RobotError::new(RobotErrorType::Connection) ) }
        };

        loop {
            let mut cmd_buf = [0; 3];
            self.stream.read_exact(&mut cmd_buf).unwrap();

            let len: i32 = ((cmd_buf[1] as i32) << 8) | (cmd_buf[2] as i32);

            let mut buf: Vec<u8> = vec![0;len as usize];
            self.stream.read_exact(&mut buf[..]).unwrap();

            if cmd_buf[0] == 130 {

                let mut buf_x = &buf[2..7];
                let mut buf_y = &buf[6..11];

                x = buf_x.read_i32::<BigEndian>().unwrap();
                y = buf_y.read_i32::<BigEndian>().unwrap();

                break;
            }
        }

        self.disconnect();

        return Ok(RobotLocation { x: x, y: y });
    }


    fn get_state() -> Result<RobotState, RobotError> {
        return Err( RobotError::new(RobotErrorType::NotYetImplemented) )
    }

    fn free(&mut self) -> Result<(), RobotError> {

        let mut buf = [0; 4];

        buf[0] = 58;
        buf[1] = 0;
        buf[2] = 1;
        buf[3] = 5;

        match self.connect() {
            Ok(_) => {},
            Err(_) => { return Err( RobotError::new(RobotErrorType::Connection) ) }
        };
        
        match self.stream.write_all(&mut buf) {
            Ok(_) => {},
            Err(_) => { return Err( RobotError::new(RobotErrorType::Write) ) }
        }

        self.disconnect();

        Ok(())
    }

    // Routes the robot to a point defined in the config file
    fn goto_point(&mut self, point: &str) -> Result<(), RobotError> {
        // Get point from config file   
        match self.config.get_point(point) {
            // Order robot to go the that location
            Ok(p) => match self.goto(p.0, p.1) {
                Ok(_) => Ok(()),
                Err(_) => Err( RobotError::new(RobotErrorType::Write) )
            },
            Err(_) => Err( RobotError::new(RobotErrorType::Read) )
        }
    }

    // Routes the robot to specific coordinates
    fn goto(&mut self, x: i32, y: i32) -> Result<(), RobotError> {

        match self.connect() {
            Ok(_) => {},
            Err(_) => { return Err( RobotError::new(RobotErrorType::Connection) ) }
        };
 


        let mut buf = [0; 12];

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

        match self.stream.write_all(&mut buf) {
            Ok(_) => {},
            Err(_) => { return Err( RobotError::new(RobotErrorType::Write) ) }
        }

        self.disconnect();

        Ok(())
    } 

    fn localize(&mut self) -> Result<(), RobotError> {
        let mut buf = [0; 4];

        buf[0] = 58;
        buf[1] = 0;
        buf[2] = 1;
        buf[3] = 3;

        match self.connect() {
            Ok(_) => {},
            Err(_) => { return Err( RobotError::new(RobotErrorType::Connection) ) }
        };
 
    
        match self.stream.write_all(&mut buf) {
            Ok(_) => {},
            Err(_) => { return Err( RobotError::new(RobotErrorType::Write) ) }
        }

        self.disconnect();

        Ok(())
    }

    fn stop(&mut self) -> Result<(), RobotError> {
        let mut buf = [0; 4];

        buf[0] = 58;
        buf[1] = 0;
        buf[2] = 1;
        buf[3] = 8;

        match self.connect() {
            Ok(_) => {},
            Err(_) => { return Err( RobotError::new(RobotErrorType::Connection) ) }
        };
        
        match self.stream.write_all(&mut buf) {
            Ok(_) => {},
            Err(_) => { return Err( RobotError::new(RobotErrorType::Write) ) }
        }
        
        self.disconnect();

        Ok(())
    }

    fn save_location(&mut self, location: &str) -> Result<(), RobotError> { 
        let robo_location = self.get_location()?;

        match self.config.set_point(location, robo_location.x, robo_location.y) {
            Ok(_) => {
                match self.config.write(&self.config_path) {
                    Ok(_) => Ok(()),
                    Err(_) => Err( RobotError::new(RobotErrorType::Write) )
                }
            },
            Err(_) => Err( RobotError::new(RobotErrorType::Write) )
        }
    }
}
