mod pulurobot;
mod config;
mod error;

use std::net::TcpStream;


#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub name: String,
    pub manufacturer: String,
    //server_address: String,
    //server_port: String,
    //private_key: String,
    pub robot_address: String,
    pub robot_port: String,
    pub point_a_x: i32,
    pub point_a_y: i32,
    pub point_b_x: i32,
    pub point_b_y: i32,
}

pub enum ConfigErrorType {
    Read,
    Open,
    FileNotFound,
    Deserialization,
    Serialization,
}

pub struct ConfigError {
    pub err_type: ConfigErrorType,
}

pub trait ConfigErrorHandler {
    fn new(error_type: ConfigErrorType) -> Self;
}

pub trait ConfigHandler {
    fn new() -> Self;
    fn create(config_path: &str) -> Result<Config, ConfigError>;
    fn from_file(self, config_path: &str) -> Result<Config, ConfigError>; 
    fn write(&mut self, config_path: &str) -> Result<(), ConfigError>;  
    fn set_point(&mut self, name: &str, x: i32, y: i32) -> Result<(), ConfigError>;
    fn get_point(&mut self, name: &str) -> Result<(i32,i32), ConfigError>;
}

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
    pub stream: TcpStream,
    pub config_path: String,
    pub config: Config,
}

pub trait PuluRobot {
    fn from_config(config_path: &str) -> Result<Robot, RobotError>;
    fn connect(&mut self) -> Result<(), RobotError>;
    fn disconnect(&mut self);
    fn get_location(&mut self) -> Result<RobotLocation, RobotError>;
    fn get_state() -> Result<RobotState, RobotError>;
    fn free(&mut self) -> Result<(), RobotError>;
    fn goto_point(&mut self, point: &str) -> Result<(), RobotError>;
    fn goto(&mut self, x: i32, y: i32) -> Result<(), RobotError>;
    fn localize(&mut self) -> Result<(), RobotError>;  
    fn stop(&mut self) -> Result<(), RobotError>;
    fn save_location(&mut self, location: &str) -> Result<(), RobotError>;
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



