use crate::data_services::database_service_base::DatabaseServiceTrait;
use crate::data_services::database_service_error::DatabaseServiceError;
use crate::models::item::Item;

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
        self.db_service.insert(format!("INSERT INTO item (descr, amount, active, picture) VALUES('{}', {}, {}, ?1)",
                                       item.get_descr(),
                                       item.get_amount(),
                                       item.get_active()
                                       ).as_str(),
                                       Some(vec![item.get_picture().to_vec()])).await
    }

    pub async fn update_item(&mut self, updated_item: &Item) -> Result<u64, DatabaseServiceError> {
        self.db_service.update(format!("UPDATE item SET descr = '{}', amount = {}, active = {}, picture = ? WHERE id = {}",
                                       updated_item.get_descr(),
                                       updated_item.get_amount(),
                                       updated_item.get_active(),
                                       updated_item.get_id()).as_str(),
                                       Some(vec![updated_item.get_picture().to_vec()])).await
    }

    pub async fn delete_item(&mut self, item_id: i64) -> Result<u64, DatabaseServiceError> {
        self.db_service.delete(format!("DELETE FROM item WHERE id = {}",
                                       item_id).as_str()).await
    }

    pub async fn get_all_items(&mut self) -> Result<Vec<Item>, DatabaseServiceError> {
        let rows = self.db_service.select("SELECT * FROM item").await?;
        let items = rows.into_iter().map(|x| Item::new(x.get_integer("id").unwrap(),
                                                       x.get_text("descr").unwrap(),
                                                       x.get_real("amount").unwrap(),
                                                       x.get_bool("active").unwrap(),
                                                       Some(x.get_blob("picture").unwrap())
                                                       )).collect();
        Ok(items)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use crate::data_services::database_service_base::{BlobList, DatabaseRow, DatabaseRowValue};

    pub enum FakeResult {
        Ok(i64),
        OkSelect(Vec<DatabaseRow>),
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
        async fn insert(&mut self,
                        query: &str,
                        _blobs: Option<BlobList>) -> Result<i64, DatabaseServiceError> {
            assert_eq!(self.received_query, query);
            match &self.result {
                FakeResult::Ok(x) => Ok(*x),
                FakeResult::Err(e) => Err(DatabaseServiceError::new(e.as_str())),
                _ => unimplemented!()
            }
        }

        async fn update(&mut self,
                        query: &str,
                        _blobs: Option<BlobList>) -> Result<u64, DatabaseServiceError> {
            assert_eq!(self.received_query, query);
            match &self.result {
                FakeResult::Ok(x) => Ok(*x as u64),
                FakeResult::Err(e) => Err(DatabaseServiceError::new(e.as_str())),
                _ => unimplemented!()
            }
        }

        async fn delete(&mut self, query: &str) -> Result<u64, DatabaseServiceError> {
            assert_eq!(self.received_query, query);
            match &self.result {
                FakeResult::Ok(x) => Ok(*x as u64),
                FakeResult::Err(e) => Err(DatabaseServiceError::new(e.as_str())),
                _ => unimplemented!()
            }
        }

        async fn select(&mut self, query: &str) -> Result<Vec<DatabaseRow>, DatabaseServiceError> {
            assert_eq!(self.received_query, query);
            match &self.result {
                FakeResult::OkSelect(x) => Ok(x.to_vec()),
                FakeResult::Err(e) => Err(DatabaseServiceError::new(e.as_str())),
                _ => unimplemented!()
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
        fake_db.received_query = "INSERT INTO item (descr, amount, active, picture) VALUES('test', 1.23, true, ?1)".to_owned();
        fake_db.result = FakeResult::Ok(1);
        let mut storage = ItemStorageService::new(fake_db);
        assert_eq!(1, storage.add_item(&Item::new(-1, "test", 1.23, true, None)).await.unwrap());
    }

    #[tokio::test]
    async fn itemstorageservice_add_item_with_error_execution_return_err() {
        let fake_db = Box::new(FakeDataService::new("INSERT INTO item (descr, amount, active, picture) VALUES('test', 1.23, true, ?1)",
            FakeResult::Err("error".to_owned())));
        let mut storage = ItemStorageService::new(fake_db);
        assert_eq!("error", storage.add_item(&Item::new(-1, "test", 1.23, true, None)).await.unwrap_err().to_string());
    }

    #[tokio::test]
    async fn itemstorageservice_update_item_with_descr_test2_return_1_row_affected() {
        let mut fake_db = Box::new(FakeDataService::default());
        fake_db.received_query = "UPDATE item SET descr = 'test2', amount = 1.23, active = true, picture = ? WHERE id = 1".to_owned();
        fake_db.result = FakeResult::Ok(1);
        let mut storage = ItemStorageService::new(fake_db);
        assert_eq!(1, storage.update_item(&Item::new(1, "test2", 1.23, true, None)).await.unwrap());
    }

    #[tokio::test]
    async fn itemstorageservice_update_item_with_non_existing_item_return_0_row_affected() {
        let mut fake_db = Box::new(FakeDataService::default());
        fake_db.received_query = "UPDATE item SET descr = 'test2', amount = 1.23, active = true, picture = ? WHERE id = 1".to_owned();
        fake_db.result = FakeResult::Ok(0);
        let mut storage = ItemStorageService::new(fake_db);
        assert_eq!(0, storage.update_item(&Item::new(1, "test2", 1.23, true, None)).await.unwrap());
    }

    #[tokio::test]
    async fn itemstorageservice_update_item_with_error_execution_return_err() {
        let fake_db = Box::new(FakeDataService::new("UPDATE item SET descr = 'test2', amount = 1.23, active = true, picture = ? WHERE id = 1",
            FakeResult::Err("error".to_owned())));
        let mut storage = ItemStorageService::new(fake_db);
        assert_eq!("error", storage.update_item(&Item::new(1, "test2", 1.23, true, None)).await.unwrap_err().to_string());
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

    #[tokio::test]
    async fn itemstorageservice_get_all_items_with_error_execution_return_err() {
        let fake_db = Box::new(FakeDataService::new("SELECT * FROM item",
            FakeResult::Err("error".to_owned())));
        let mut storage = ItemStorageService::new(fake_db);
        assert_eq!("error", storage.get_all_items().await.unwrap_err().to_string());
    }

    fn get_sample_item_row(id: i64, descr: &str) -> DatabaseRow {
        let mut retval = DatabaseRow::new();
        retval.add_column("id", DatabaseRowValue::Integer(id));
        retval.add_column("descr", DatabaseRowValue::Text(descr.to_owned()));
        retval.add_column("amount", DatabaseRowValue::Real(1.23));
        retval.add_column("active", DatabaseRowValue::Bool(true));
        retval.add_column("picture", DatabaseRowValue::Blob(vec![]));
        retval
    }

    #[tokio::test]
    async fn itemstorageservice_get_all_items_with_2_rows_return_2_items() {
        let fake_db = Box::new(FakeDataService::new("SELECT * FROM item",
            FakeResult::OkSelect(vec![
                get_sample_item_row(1, "Test1"),
                get_sample_item_row(2, "Test2")
            ])));

        let mut storage = ItemStorageService::new(fake_db);
        let items = storage.get_all_items().await.unwrap();
        assert_eq!(2, items.len());
        assert_eq!("Test1", items[0].get_descr());
        assert_eq!("Test2", items[1].get_descr());
    }
}

