use gloo_net::http::Request;
use serde_json::json;
use shared::{GroceryItem, Recipe};

const BACKEND_URL: &str = "http://localhost:3000";

/// Send local grocery items to the Axum backend, upsert them in SQLite,
/// and return the merged list from the server.
pub async fn sync_groceries_with_backend(items: &[GroceryItem]) -> Result<Vec<GroceryItem>, String> {
    let response = Request::post(&format!("{}/api/groceries/sync", BACKEND_URL))
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
    let response = Request::post(&format!("{}/api/recipes/scrape", BACKEND_URL))
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
    let response = Request::delete(&format!("{}/api/recipes/{}", BACKEND_URL, id))
        .send()
        .await
        .map_err(|e| format!("Network error: {}", e))?;

    if !response.ok() {
        return Err(format!("Failed to delete recipe from backend: {}", response.status()));
    }

    Ok(())
}
