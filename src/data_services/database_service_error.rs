use core::fmt;

pub struct DatabaseServiceError {
    message: String
}

impl DatabaseServiceError {
    pub fn new(message: &str) ->  Self {
        DatabaseServiceError {
            message: message.to_string()
        }
    }
}

impl fmt::Display for DatabaseServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl fmt::Debug for DatabaseServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

