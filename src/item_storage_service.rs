use crate::{database_service_trait::{DatabaseServiceTrait, DatabaseServiceError}, item::Item};

pub struct ItemStorageService {
    db_service: Box<dyn DatabaseServiceTrait>
}

impl ItemStorageService {
    pub fn new(db_service: Box<dyn DatabaseServiceTrait>) -> Self {
        ItemStorageService {
            db_service
        }
    }

    pub async fn add_item(&mut self, item: &Item) -> Result<i64, DatabaseServiceError> {
        self.db_service.insert(format!("INSERT INTO item (descr) VALUES('{}')",
                                       item.get_descr()
                                       ).as_str()).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;

    pub enum FakeResult {
        Ok(i64),
        Err(String)
    }

    pub struct FakeDataService {
        pub received_query: String,
        pub result: FakeResult
    }

    impl FakeDataService {
        pub fn new(query: &str, result: FakeResult) -> Self {
            Self {
                received_query: query.to_owned(),
                result
            }
        }

        pub fn default() -> Self {
            FakeDataService {
                received_query: "".to_owned(),
                result: FakeResult::Ok(0)
            }
        }
    }

    #[async_trait]
    impl DatabaseServiceTrait for FakeDataService {
        async fn insert(&mut self, query: &str) -> Result<i64, DatabaseServiceError> {
            assert_eq!(self.received_query, query);
            match &self.result {
                FakeResult::Ok(x) => Ok(*x),
                FakeResult::Err(e) => Err(DatabaseServiceError::new(e.as_str()))
            }
        }

        async fn update(&self, _query: &str) -> Result<i64, DatabaseServiceError> {
            Ok(1)
        }

        async fn delete(&self, _query: &str) -> Result<i64, DatabaseServiceError> {
            Ok(1)
        }
    }

    #[test]
    fn itemstorageservice_new_with_fake_db_return_success() {
        let fake_db = Box::new(FakeDataService::default());
        let _ = ItemStorageService::new(fake_db);
    }

    #[tokio::test]
    async fn itemstorageservice_add_item_with_descr_test_return_ok() {
        let mut fake_db = Box::new(FakeDataService::default());
        fake_db.received_query = "INSERT INTO item (descr) VALUES('test')".to_owned();
        fake_db.result = FakeResult::Ok(1);
        let mut storage = ItemStorageService::new(fake_db);
        assert_eq!(1, storage.add_item(&Item::new(-1, "test")).await.unwrap());
    }

    #[tokio::test]
    async fn itemstorageservice_add_item_with_error_execution_return_err() {
        let fake_db = Box::new(FakeDataService::new("INSERT INTO item (descr) VALUES('test')",
            FakeResult::Err("error".to_owned())));
        let mut storage = ItemStorageService::new(fake_db);
        assert_eq!("error", storage.add_item(&Item::new(-1, "test")).await.unwrap_err().to_string());
    }
}
