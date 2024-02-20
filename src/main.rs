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
use std::io::BufRead;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db_url = &env::var("DATABASE_URL").unwrap();
    let mut db_service = DatabaseService::new(db_url);
    let _ = db_service.connect().await
        .unwrap_or_else(|e| { eprintln!("Unable to connect to the database : {}", e);});

    let mut item_service = ItemStorageService::new(Box::new(db_service));

    println!("Here is a sample database operations using sqlx");
    // Add an item
    println!("Adding an item with the following attributes:");
    println!("Description: Test1");
    println!("Amount: 2.34");
    println!("Active: true");
    println!("Picture: An ascii art :)\n");
    println!("Press enter to proceed...");
    wait_for_enter();
    let mut item = Item::new(-1, "Test1", 2.34, true, Some(vec![10, 32, 95, 95,
32, 32, 32, 32, 32, 32, 32, 32, 32, 95, 95, 10, 47, 32, 32, 92, 46, 45, 34, 34,
34, 45, 46, 47, 32, 32, 92, 10, 92, 32, 32, 32, 32, 45, 32, 32, 32, 45, 32, 32,
32, 32, 47, 10, 32, 124, 32, 32, 32, 111, 32, 32, 32, 111, 32, 32, 32, 124, 10,
32, 92, 32, 32, 46, 45, 39, 39, 39, 45, 46, 32, 32, 47, 10, 32, 32, 39, 45, 92,
95, 95, 89, 95, 95, 47, 45, 39, 10, 32, 32, 32, 32, 32, 96, 45, 45, 45, 96]));
    let new_item_id = item_service.add_item(&item).await
        .unwrap_or_else(|e| { eprintln!("Error while adding the item : {}", e); -1});
    if new_item_id == -1 {
        return;
    }
    item.set_id(new_item_id);
    println!("Item added with id {}!\n", new_item_id);

    // Showing the current rows
    println!("Press enter to show the database item table rows...");
    wait_for_enter();
    print_rows(&mut item_service).await;
    wait_for_enter();

    // Update an item
    println!("Updating the item with id {} with the following attributes:", new_item_id);
    println!("Description: Test2");
    println!("Amount: 1.45");
    println!("Active: false");
    println!("Picture: Another ascii art :)\n");
    println!("Press enter to proceed...");
    item.set_descr("Test2");
    item.set_amount(1.45);
    item.set_active(false);
    item.set_picture(&vec![10, 32, 32, 32, 32, 92, 92, 95, 47, 47, 10, 32, 32,
                     32, 95, 95, 47, 34, 46, 10, 32, 32, 47, 95, 95, 32, 124,
                     10, 32, 32, 124, 124, 32, 124, 124]);
    wait_for_enter();
    let mut rows_affected = item_service.update_item(&item).await
        .unwrap_or_else(|e| { eprintln!("{}", e); 0});
    println!("Rows affected: {}\n", rows_affected);

    // Showing the current rows
    println!("Press enter to show the database item table rows...");
    wait_for_enter();
    print_rows(&mut item_service).await;
    wait_for_enter();

    // Delete an item
    println!("Deleting the itemwith id {}", new_item_id);
    println!("Press enter to proceed...");
    wait_for_enter();
    rows_affected = item_service.delete_item(new_item_id).await
        .unwrap_or_else(|e| { eprintln!("{}", e); 0});
    println!("Rows affected: {}\n", rows_affected);

    // Showing the current rows
    println!("Press enter to show the database item table rows...");
    wait_for_enter();
    print_rows(&mut item_service).await;
    wait_for_enter();
}

async fn print_rows(item_service: &mut ItemStorageService) {
    let items = item_service.get_all_items().await
        .unwrap_or_else(|e| { eprintln!("Error while getting rows : {}", e); vec![]});
    if items.len() == 0 {
        println!("No row to display!");
    }
    for item in items {
        print_item(&item);
    }
}

fn print_item(item: &Item) {
    println!("id: {}, descr: {}, amount: {}, active: {}",
             item.get_id(),
             item.get_descr(),
             item.get_amount(),
             item.get_active());
    println!("picture: \n{}", String::from_utf8(item.get_picture().to_vec()).unwrap());

}

fn wait_for_enter() {
    let mut buffer = String::new();
    let stdin = std::io::stdin();

    loop {
        let result = stdin.lock().read_line(&mut buffer);
        if result.is_ok() {
            return;
        }
    }
}
