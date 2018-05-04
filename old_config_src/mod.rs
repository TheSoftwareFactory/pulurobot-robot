mod config;
mod error;

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub name: String,
    pub manufacturer: String,
    //server_address: String,
    //server_port: String,
    //private_key: String,
    pub robot_address: String,
    pub robot_port: String,
    pub location_a_x: i32,
    pub location_a_y: i32,
    pub location_b_x: i32,
    pub location_b_y: i32,
}

pub enum ConfigErrorType {
    Read,
    Write,
    FileNotFound,
    Deserialization,
    Serialization,
    SyntaxError,
    MissingFields
}

pub struct ConfigError {
    pub err_type: ConfigErrorType,
}

pub trait ConfigErrorHandler {
    fn new(error_type: ConfigErrorType) -> Self;
}

pub trait ConfigHandler {
    fn new() -> Self;
    fn from_file(self, config_path: &str) -> Result<Config, ConfigError>; 
    fn write(self) -> Result<(), ConfigError>;  
    fn set_location(self, name: &str, x: i32, y: i32) -> Result<(), ConfigError>;
    //fn get_location(self, name: &str) -> Result<(i32,i32), ConfigError>;
}



