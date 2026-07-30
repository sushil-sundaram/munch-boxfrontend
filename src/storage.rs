use gloo_storage::{LocalStorage, Storage};
use shared::GroceryItem;

const GROCERY_KEY: &str = "munchbox_groceries";

// Load items from the browser's local storage
pub fn load_groceries() -> Vec {
    // If it fails to load (e.g., nothing is saved yet), return an empty list
    LocalStorage::get(GROCERY_KEY).unwrap_or_else(|_| Vec::new())
}

// Save items to the browser's local storage
pub fn save_groceries(items: &Vec) {
    // We ignore the Result here for simplicity, but in a production app 
    // you might want to handle storage full errors!
    let _ = LocalStorage::set(GROCERY_KEY, items);
}