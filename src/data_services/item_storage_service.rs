use crate::{data_services::database_service_trait::{DatabaseServiceTrait, DatabaseServiceError},
    models::item::Item
};

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

    pub async fn update_item(&mut self, updated_item: &Item) -> Result<u64, DatabaseServiceError> {
        self.db_service.update(format!("UPDATE item SET descr = '{}' WHERE id = {}",
                                       updated_item.get_descr(),
                                       updated_item.get_id()).as_str()).await
    }

    pub async fn delete_item(&mut self, item_id: i64) -> Result<u64, DatabaseServiceError> {
        self.db_service.delete(format!("DELETE FROM item WHERE id = {}",
                                       item_id).as_str()).await
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

        async fn update(&mut self, query: &str) -> Result<u64, DatabaseServiceError> {
            assert_eq!(self.received_query, query);
            match &self.result {
                FakeResult::Ok(x) => Ok(*x as u64),
                FakeResult::Err(e) => Err(DatabaseServiceError::new(e.as_str()))
            }
        }

        async fn delete(&mut self, query: &str) -> Result<u64, DatabaseServiceError> {
            assert_eq!(self.received_query, query);
            match &self.result {
                FakeResult::Ok(x) => Ok(*x as u64),
                FakeResult::Err(e) => Err(DatabaseServiceError::new(e.as_str()))
            }
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

    #[tokio::test]
    async fn itemstorageservice_update_item_with_descr_test2_return_1_row_affected() {
        let mut fake_db = Box::new(FakeDataService::default());
        fake_db.received_query = "UPDATE item SET descr = 'test2' WHERE id = 1".to_owned();
        fake_db.result = FakeResult::Ok(1);
        let mut storage = ItemStorageService::new(fake_db);
        assert_eq!(1, storage.update_item(&Item::new(1, "test2")).await.unwrap());
    }

    #[tokio::test]
    async fn itemstorageservice_update_item_with_non_existing_item_return_0_row_affected() {
        let mut fake_db = Box::new(FakeDataService::default());
        fake_db.received_query = "UPDATE item SET descr = 'test2' WHERE id = 1".to_owned();
        fake_db.result = FakeResult::Ok(0);
        let mut storage = ItemStorageService::new(fake_db);
        assert_eq!(0, storage.update_item(&Item::new(1, "test2")).await.unwrap());
    }

    #[tokio::test]
    async fn itemstorageservice_update_item_with_error_execution_return_err() {
        let fake_db = Box::new(FakeDataService::new("UPDATE item SET descr = 'test2' WHERE id = 1",
            FakeResult::Err("error".to_owned())));
        let mut storage = ItemStorageService::new(fake_db);
        assert_eq!("error", storage.update_item(&Item::new(1, "test2")).await.unwrap_err().to_string());
    }

    #[tokio::test]
    async fn itemstorageservice_delete_item_with_id_2_existing_return_1_row_affected() {
        let mut fake_db = Box::new(FakeDataService::default());
        fake_db.received_query = "DELETE FROM item WHERE id = 2".to_owned();
        fake_db.result = FakeResult::Ok(1);
        let mut storage = ItemStorageService::new(fake_db);
        assert_eq!(1, storage.delete_item(2).await.unwrap());
    }

    #[tokio::test]
    async fn itemstorageservice_delete_item_with_id_2_non_existing_return_0_row_affected() {
        let mut fake_db = Box::new(FakeDataService::default());
        fake_db.received_query = "DELETE FROM item WHERE id = 2".to_owned();
        fake_db.result = FakeResult::Ok(0);
        let mut storage = ItemStorageService::new(fake_db);
        assert_eq!(0, storage.delete_item(2).await.unwrap());
    }

    #[tokio::test]
    async fn itemstorageservice_delete_item_with_error_execution_return_err() {
        let fake_db = Box::new(FakeDataService::new("DELETE FROM item WHERE id = 1",
            FakeResult::Err("error".to_owned())));
        let mut storage = ItemStorageService::new(fake_db);
        assert_eq!("error", storage.delete_item(1).await.unwrap_err().to_string());
    }
}

