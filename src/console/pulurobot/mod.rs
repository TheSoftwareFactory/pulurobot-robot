mod pulurobot;
mod error;

use std::net::{TcpStream, Shutdown};

pub enum RobotErrorType {
    Connection,
    Write,
    Read,
    NotYetImplemented
}

pub struct RobotError {
    pub err_type: RobotErrorType,
}

pub trait RobotErrorHandler {
    fn new(error_type: RobotErrorType) -> Self;
}

pub struct RobotLocation {
    pub x: i32,
    pub y: i32,
}

pub enum RobotState {
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

pub struct Robot {
    stream: TcpStream 
}

pub trait PuluRobot {
    fn connect(ipaddr: &str, port: &str) -> Result<Robot, RobotError>;
    fn disconnect(&mut self);
    fn listen(&mut self) -> Result<String, RobotError>;
    fn get_location() -> Result<RobotLocation, RobotError>;
    fn get_state() -> Result<RobotState, RobotError>;
    fn free(&mut self) -> Result<(), RobotError>;
    fn goto(&mut self, x: i32, y: i32) -> Result<(), RobotError>;
    fn localize(&mut self) -> Result<(), RobotError>;  
    fn stop(&mut self) -> Result<(), RobotError>;
}





