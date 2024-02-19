use std::collections::HashMap;
use async_trait::async_trait;
use crate::data_services::database_service_error::DatabaseServiceError;

pub type Blob = Vec<u8>;
pub type BlobList = Vec<Blob>;

#[derive(Clone)]
pub enum DatabaseRowValue {
    Integer(i64),
    Text(String),
    Real(f64),
    Blob(Blob),
    Bool(bool)
}

#[derive(Clone)]
pub struct DatabaseRow {
    columns: HashMap<String, DatabaseRowValue>
}

impl DatabaseRow {
    pub fn new() -> Self {
        Self {
            columns: HashMap::new()
        }
    }

    pub fn add_column(&mut self, column_name: &str, value: DatabaseRowValue) {
        self.columns.insert(column_name.to_owned(), value);
    }

    pub fn get_integer(&self, column_name: &str) -> Result<i64, DatabaseServiceError> {
        match self.columns.get(column_name).unwrap() {
            DatabaseRowValue::Integer(i) => Ok(*i),
            _ => Err(DatabaseServiceError::new(format!("The column {} is not Integer type", column_name).as_str()))
        }
    }

    pub fn get_text(&self, column_name: &str) -> Result<&str, DatabaseServiceError> {
        match self.columns.get(column_name).unwrap() {
            DatabaseRowValue::Text(t) => Ok(t),
            _ => Err(DatabaseServiceError::new(format!("The column {} is not Text type", column_name).as_str()))
        }
    }

    pub fn get_real(&self, column_name: &str) -> Result<f64, DatabaseServiceError> {
        match self.columns.get(column_name).unwrap() {
            DatabaseRowValue::Real(i) => Ok(*i),
            _ => Err(DatabaseServiceError::new(format!("The column {} is not Real type", column_name).as_str()))
        }
    }

    pub fn get_blob(&self, column_name: &str) -> Result<Blob, DatabaseServiceError> {
        match self.columns.get(column_name).unwrap() {
            DatabaseRowValue::Blob(i) => Ok(i.to_vec()),
            _ => Err(DatabaseServiceError::new(format!("The column {} is not Blob type", column_name).as_str()))
        }
    }

    pub fn get_bool(&self, column_name: &str) -> Result<bool, DatabaseServiceError> {
        match self.columns.get(column_name).unwrap() {
            DatabaseRowValue::Bool(i) => Ok(*i),
            _ => Err(DatabaseServiceError::new(format!("The column {} is not Bool type", column_name).as_str()))
        }
    }
}

#[async_trait]
pub trait DatabaseServiceTrait {
    async fn insert(&mut self,
                    query: &str,
                    blobs: Option<BlobList>) -> Result<i64, DatabaseServiceError>;
    async fn update(&mut self,
                    query: &str,
                    blobs: Option<BlobList>) -> Result<u64, DatabaseServiceError>;
    async fn delete(&mut self, query: &str) -> Result<u64, DatabaseServiceError>;
    async fn select(&mut self, query: &str) -> Result<Vec<DatabaseRow>, DatabaseServiceError>;
}
