mod database_service_trait;
mod database_service;
mod item;
mod item_storage_service;

use database_service::DatabaseService;
use item::Item;
use crate::item_storage_service::ItemStorageService;
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


