use super::{ConfigErrorHandler, ConfigError, ConfigErrorType};

impl ConfigErrorHandler for ConfigError {
    fn new(error_type: ConfigErrorType) -> Self {
        ConfigError {
            err_type: error_type,
        }
    }
}
