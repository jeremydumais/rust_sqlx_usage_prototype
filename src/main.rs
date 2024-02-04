pub mod data_services {
    pub mod database_service;
    pub mod database_service_trait;
    pub mod item_storage_service;
}
pub mod models {
    pub mod item;
}

use crate::models::item::Item;
use crate::data_services::database_service::DatabaseService;
use crate::data_services::item_storage_service::ItemStorageService;
use std::env;
use dotenv::dotenv;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db_url = &env::var("DATABASE_URL").unwrap();
    let mut db_service = DatabaseService::new(db_url);
    let _ = db_service.connect().await
        .unwrap_or_else(|e| { eprintln!("{}", e);});

    let mut item_service = ItemStorageService::new(Box::new(db_service));
    let _ = item_service.add_item(&Item::new(-1, "Test2")).await
        .unwrap_or_else(|e| { eprintln!("{}", e); -1});
}


