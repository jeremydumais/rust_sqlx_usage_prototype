use crate::data_services::database_service_trait::{DatabaseServiceError, DatabaseServiceTrait};
use sqlx::sqlite::SqlitePool;
use async_trait::async_trait;

pub struct DatabaseService {
    database_file_path: String,
    pool: Option<SqlitePool>
}

impl DatabaseService {
    #[must_use]
    pub fn new(database_file_path: &str) -> Self {
        DatabaseService {
            database_file_path: database_file_path.to_string(),
            pool: None
        }
    }

    pub async fn connect(&mut self) -> Result<(), DatabaseServiceError> {
        self.pool = SqlitePool::connect(self.database_file_path.as_str()).await
            .map(|result| Some(result))
            .map_err(|e| DatabaseServiceError::new(e.to_string().as_str()))?;
        Ok(())
    }

    fn get_not_connected_msg(&self) -> String {
        return "Database is not connected. Call the connect method first.".to_owned();
    }
}

#[async_trait]
impl DatabaseServiceTrait for DatabaseService {
    async fn insert(&mut self, query: &str) -> Result<i64, DatabaseServiceError> {
        if self.pool.is_none() {
            return Err(DatabaseServiceError::new(self.get_not_connected_msg().as_str()));
        }
        let mut conn = self.pool.as_mut().unwrap().acquire().await
            .map_err(|e| DatabaseServiceError::new(e.to_string().as_str()))?;
        let last_inserted_id = sqlx::query(query).execute(&mut *conn).await
            .map(|result| result.last_insert_rowid())
            .map_err(|e| DatabaseServiceError::new(e.to_string().as_str()))?;
        Ok(last_inserted_id)
    }

    async fn update(&mut self, query: &str) -> Result<u64, DatabaseServiceError> {
        if self.pool.is_none() {
            return Err(DatabaseServiceError::new(self.get_not_connected_msg().as_str()));
        }
        let mut conn = self.pool.as_mut().unwrap().acquire().await
            .map_err(|e| DatabaseServiceError::new(e.to_string().as_str()))?;
        let rows_affected = sqlx::query(query).execute(&mut *conn).await
            .map(|result| result.rows_affected())
            .map_err(|e| DatabaseServiceError::new(e.to_string().as_str()))?;
        Ok(rows_affected)
    }

    async fn delete(&mut self, query: &str) -> Result<u64, DatabaseServiceError> {
        if self.pool.is_none() {
            return Err(DatabaseServiceError::new(self.get_not_connected_msg().as_str()));
        }
        let mut conn = self.pool.as_mut().unwrap().acquire().await
            .map_err(|e| DatabaseServiceError::new(e.to_string().as_str()))?;
        let rows_affected = sqlx::query(query).execute(&mut *conn).await
            .map(|result| result.rows_affected())
            .map_err(|e| DatabaseServiceError::new(e.to_string().as_str()))?;
        Ok(rows_affected)
    }
}
