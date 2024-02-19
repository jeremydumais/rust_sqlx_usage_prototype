pub mod data_services {
    pub mod database_service;
    pub mod database_service_base;
    pub mod database_service_error;
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
    let mut item = Item::new(-1, "Test1", 2.34, true, Some(vec![41, 52, 64, 75]));
    let new_item_id = item_service.add_item(&item).await
        .unwrap_or_else(|e| { eprintln!("{}", e); -1});
    item.set_id(new_item_id);

    // Update an item
    println!("Updating an item...");
    item.set_descr("Test33");
    item.set_amount(1.45);
    item.set_active(false);
    item.set_picture(&vec![47u8, 65u8, 78u8, 35u8, 75u8]);
    let rows_affected = item_service.update_item(&item).await
        .unwrap_or_else(|e| { eprintln!("{}", e); 0});
    println!("Rows affected: {}", rows_affected);

    // Show all items
    println!("Show all items...");
    let items = item_service.get_all_items().await
        .unwrap_or_else(|e| { eprintln!("{}", e); vec![]});
    for item in items {
        println!("{}", item.get_descr());
    }

    // Delete an item
    //println!("Deleting an item...");
    //rows_affected = item_service.delete_item(new_item_id).await
        //.unwrap_or_else(|e| { eprintln!("{}", e); 0});
    //println!("Rows affected: {}", rows_affected);
}


