use core::fmt;
use async_trait::async_trait;

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

#[async_trait]
pub trait DatabaseServiceTrait {
    async fn insert(&mut self, query: &str) -> Result<i64, DatabaseServiceError>;
    async fn update(&mut self, query: &str) -> Result<i64, DatabaseServiceError>;
    async fn delete(&self, query: &str) -> Result<i64, DatabaseServiceError>;
    //fn select(query: &str, args: &[&dyn std::fmt::Debug]) ->
}
