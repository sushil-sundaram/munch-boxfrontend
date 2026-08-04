use gloo_net::http::Request;
use serde_json::json;
use shared::{GroceryItem, Recipe};

fn get_backend_url() -> String {
    if let Some(window) = web_sys::window() {
        if let Ok(location) = window.location().hostname() {
            if !location.is_empty() {
                return format!("http://{}:3000", location);
            }
        }
    }
    "http://localhost:3000".to_string()
}

/// Send local grocery items to the Axum backend, upsert them in SQLite,
/// and return the merged list from the server.
pub async fn sync_groceries_with_backend(items: &[GroceryItem]) -> Result<Vec<GroceryItem>, String> {
    let backend_url = get_backend_url();
    let response = Request::post(&format!("{}/api/groceries/sync", backend_url))
        .json(items)
        .map_err(|e| format!("Failed to serialize payload: {}", e))?
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if !response.ok() {
        return Err(format!("Server status error: {}", response.status()));
    }

    let synced_items: Vec<GroceryItem> = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse response JSON: {}", e))?;

    Ok(synced_items)
}

/// Send a web or YouTube URL to Axum backend to scrape structured recipe details
pub async fn scrape_recipe_from_url(url: &str) -> Result<Recipe, String> {
    let backend_url = get_backend_url();
    let response = Request::post(&format!("{}/api/recipes/scrape", backend_url))
        .json(&json!({ "url": url }))
        .map_err(|e| format!("Failed to serialize payload: {}", e))?
        .send()
        .await
        .map_err(|e| format!("Network error connecting to backend: {}", e))?;

    if !response.ok() {
        let text = response.text().await.unwrap_or_default();
        return Err(format!("Scraping failed ({}): {}", response.status(), text));
    }

    let recipe: Recipe = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse scraped recipe response: {}", e))?;

    Ok(recipe)
}

/// Delete a recipe from the Axum backend
pub async fn delete_recipe_from_backend(id: &str) -> Result<(), String> {
    let backend_url = get_backend_url();
    let response = Request::delete(&format!("{}/api/recipes/{}", backend_url, id))
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if !response.ok() {
        return Err(format!("Failed to delete recipe from backend: {}", response.status()));
    }

    Ok(())
}
