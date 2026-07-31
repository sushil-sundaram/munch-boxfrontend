use gloo_storage::{LocalStorage, Storage};
use shared::{GroceryItem, IngredientItem, MealPlanEntry, Recipe};

const GROCERY_KEY: &str = "munchbox_groceries";
const RECIPES_KEY: &str = "munchbox_recipes";
const MEAL_PLAN_KEY: &str = "munchbox_meal_plans";

// --- Groceries ---
pub fn load_groceries() -> Vec<GroceryItem> {
    LocalStorage::get(GROCERY_KEY).unwrap_or_else(|_| Vec::new())
}

pub fn save_groceries(items: &[GroceryItem]) {
    let _ = LocalStorage::set(GROCERY_KEY, items);
}

// --- Recipes ---
pub fn load_recipes() -> Vec<Recipe> {
    let recipes: Vec<Recipe> = LocalStorage::get(RECIPES_KEY).unwrap_or_default();
    if recipes.is_empty() {
        let defaults = default_recipes();
        save_recipes(&defaults);
        defaults
    } else {
        recipes
    }
}

pub fn save_recipes(recipes: &[Recipe]) {
    let _ = LocalStorage::set(RECIPES_KEY, recipes);
}

fn default_recipes() -> Vec<Recipe> {
    vec![
        Recipe {
            id: "1".to_string(),
            title: "Database Tacos".to_string(),
            ingredients: vec![
                IngredientItem { name: "Tortillas".to_string(), quantity: 2.0, unit: "Pcs".to_string(), is_pantry_staple: false },
                IngredientItem { name: "Ground Beef".to_string(), quantity: 150.0, unit: "g".to_string(), is_pantry_staple: false },
                IngredientItem { name: "Salsa".to_string(), quantity: 2.0, unit: "tbsp".to_string(), is_pantry_staple: false },
                IngredientItem { name: "Cheddar Cheese".to_string(), quantity: 50.0, unit: "g".to_string(), is_pantry_staple: false },
                IngredientItem { name: "Salt".to_string(), quantity: 1.0, unit: "pinch".to_string(), is_pantry_staple: true },
                IngredientItem { name: "Black Pepper".to_string(), quantity: 1.0, unit: "pinch".to_string(), is_pantry_staple: true },
            ],
            instructions: "1. Brown beef in skillet with salt & pepper.\n2. Warm tortillas.\n3. Assemble tacos with salsa & cheese.".to_string(),
            emoji: "🌮".to_string(),
            rating: 5,
        },
        Recipe {
            id: "2".to_string(),
            title: "Avocado Toast & Egg".to_string(),
            ingredients: vec![
                IngredientItem { name: "Bread".to_string(), quantity: 2.0, unit: "Slices".to_string(), is_pantry_staple: false },
                IngredientItem { name: "Avocado".to_string(), quantity: 1.0, unit: "Pcs".to_string(), is_pantry_staple: false },
                IngredientItem { name: "Eggs".to_string(), quantity: 2.0, unit: "Pcs".to_string(), is_pantry_staple: false },
                IngredientItem { name: "Salt".to_string(), quantity: 1.0, unit: "pinch".to_string(), is_pantry_staple: true },
                IngredientItem { name: "Black Pepper".to_string(), quantity: 1.0, unit: "pinch".to_string(), is_pantry_staple: true },
                IngredientItem { name: "Olive Oil".to_string(), quantity: 1.0, unit: "tsp".to_string(), is_pantry_staple: true },
            ],
            instructions: "1. Toast bread.\n2. Mash avocado with salt and pepper.\n3. Fry eggs in olive oil and assemble.".to_string(),
            emoji: "🥑".to_string(),
            rating: 5,
        },
        Recipe {
            id: "3".to_string(),
            title: "Garlic Butter Pasta".to_string(),
            ingredients: vec![
                IngredientItem { name: "Pasta".to_string(), quantity: 200.0, unit: "g".to_string(), is_pantry_staple: false },
                IngredientItem { name: "Garlic Cloves".to_string(), quantity: 3.0, unit: "Pcs".to_string(), is_pantry_staple: false },
                IngredientItem { name: "Butter".to_string(), quantity: 50.0, unit: "g".to_string(), is_pantry_staple: false },
                IngredientItem { name: "Parmesan Cheese".to_string(), quantity: 30.0, unit: "g".to_string(), is_pantry_staple: false },
                IngredientItem { name: "Salt".to_string(), quantity: 1.0, unit: "tsp".to_string(), is_pantry_staple: true },
                IngredientItem { name: "Black Pepper".to_string(), quantity: 1.0, unit: "pinch".to_string(), is_pantry_staple: true },
            ],
            instructions: "1. Boil pasta in salted water.\n2. Sauté garlic in butter.\n3. Toss pasta and top with cheese & pepper.".to_string(),
            emoji: "🍝".to_string(),
            rating: 5,
        },
    ]
}

// --- Meal Plans ---
pub fn load_meal_plans() -> Vec<MealPlanEntry> {
    LocalStorage::get(MEAL_PLAN_KEY).unwrap_or_default()
}

pub fn save_meal_plans(plans: &[MealPlanEntry]) {
    let _ = LocalStorage::set(MEAL_PLAN_KEY, plans);
}