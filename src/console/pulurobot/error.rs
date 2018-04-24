use super::{RobotErrorHandler, RobotError, RobotErrorType};

impl RobotErrorHandler for RobotError {
    fn new(error_type: RobotErrorType) -> Self {
        RobotError {
            err_type: error_type,
        }
    }
}
