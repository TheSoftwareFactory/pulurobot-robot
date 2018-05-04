use super::{RobotErrorHandler, RobotError, RobotErrorType, ConfigErrorHandler, ConfigError, ConfigErrorType};

impl RobotErrorHandler for RobotError {
    fn new(error_type: RobotErrorType) -> Self {
        RobotError {
            err_type: error_type,
        }
    }
}

impl ConfigErrorHandler for ConfigError {
    fn new(error_type: ConfigErrorType) -> Self {
        ConfigError {
            err_type: error_type,
        }
    }
}
