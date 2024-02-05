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

    // Add an item
    println!("Adding an item...");
    let mut item = Item::new(-1, "Test1");
    let new_item_id = item_service.add_item(&item).await
        .unwrap_or_else(|e| { eprintln!("{}", e); -1});
    item.set_id(new_item_id);

    // Update an item
    println!("Updating an item...");
    item.set_descr("Test33");
    let mut rows_affected = item_service.update_item(&item).await
        .unwrap_or_else(|e| { eprintln!("{}", e); 0});
    println!("Rows affected: {}", rows_affected);

    // Delete an item
    println!("Deleting an item...");
    rows_affected = item_service.delete_item(new_item_id).await
        .unwrap_or_else(|e| { eprintln!("{}", e); 0});
    println!("Rows affected: {}", rows_affected);
}


