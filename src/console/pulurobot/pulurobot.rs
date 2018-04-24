use super::{PuluRobot, Robot, RobotState, RobotError, RobotLocation, RobotErrorHandler, RobotErrorType};
use std::net::{TcpStream, Shutdown, SocketAddr};
use std::time::Duration;
use std::io::Write;

impl PuluRobot for Robot {
    fn connect(ipaddr: &str, port: &str) -> Result<Robot, RobotError> {
        let robo_addr = (ipaddr.to_owned() + ":" + &port).parse().unwrap();
        match TcpStream::connect_timeout(&robo_addr, Duration::new(5,0)) {
            Ok(s) => {
                let robot = Robot {
                    stream: s,
                };
                return Ok(robot);
            },
            Err(_) => Err( RobotError::new(RobotErrorType::Connection) )
        }
    }

    fn disconnect(&mut self) {
        self.stream.shutdown(Shutdown::Both).unwrap();
    }

    fn get_location() -> Result<RobotLocation, RobotError> {
        return Err( RobotError::new(RobotErrorType::NotYetImplemented) )
    }

    fn listen(&mut self) -> Result<String, RobotError> {
        return Err( RobotError::new(RobotErrorType::NotYetImplemented) )
        /*let mut cmd_buf = [0; 3];

        self.stream.read_exact(&mut cmd_buf).unwrap();

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
        }*/
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
        
        match self.stream.write_all(&mut buf) {
            Ok(_) => Ok(()),
            Err(_) =>  Err( RobotError::new(RobotErrorType::Write) )
        }
    }

    fn goto(&mut self, x: i32, y: i32) -> Result<(), RobotError> {
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
            Ok(_) => Ok(()),
            Err(_) => Err( RobotError::new(RobotErrorType::Write) )
        }
    } 

    fn localize(&mut self) -> Result<(), RobotError> {
        let mut buf = [0; 4];

        buf[0] = 58;
        buf[1] = 0;
        buf[2] = 1;
        buf[3] = 3;
    
        match self.stream.write_all(&mut buf) {
            Ok(_) => Ok(()),
            Err(_) => Err( RobotError::new(RobotErrorType::Write) )
        }
    }

    fn stop(&mut self) -> Result<(), RobotError> {
        let mut buf = [0; 4];

        buf[0] = 58;
        buf[1] = 0;
        buf[2] = 1;
        buf[3] = 8;
        
        match self.stream.write_all(&mut buf) {
            Ok(_) => Ok(()),
            Err(_) => Err( RobotError::new(RobotErrorType::Write) )
        }
    }
}
