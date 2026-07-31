use std::collections::HashMap;
use yew::prelude::*;
use shared::{GroceryItem, MealPlanEntry};
use uuid::Uuid;
use crate::storage::{load_meal_plans, load_recipes, save_groceries, save_meal_plans};

const DAYS: [&str; 7] = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];
const MEAL_TYPES: [&str; 3] = ["Breakfast", "Lunch", "Dinner"];

#[function_component(MealPlanner)]
pub fn meal_planner() -> Html {
    let recipes = use_state(|| load_recipes());
    let meal_plans = use_state(|| load_meal_plans());
    let notification = use_state(|| Option::<String>::None);

    // Refresh recipes and meal plans whenever component receives focus/updates
    {
        let recipes = recipes.clone();
        let meal_plans = meal_plans.clone();
        use_effect_with((), move |_| {
            recipes.set(load_recipes());
            meal_plans.set(load_meal_plans());
            || ()
        });
    }

    // Handler: Assign a recipe to a day and meal slot
    let on_select_meal = {
        let meal_plans = meal_plans.clone();
        let recipes = recipes.clone();
        Callback::from(move |(day, meal_type, recipe_id): (String, String, String)| {
            let mut current = (*meal_plans).clone();
            
            // Remove existing entry for this day + meal_type slot
            current.retain(|entry| !(entry.day == day && entry.meal_type == meal_type));

            if !recipe_id.is_empty() {
                if let Some(recipe) = recipes.iter().find(|r| r.id == recipe_id) {
                    current.push(MealPlanEntry {
                        id: Uuid::new_v4().to_string(),
                        day: day.clone(),
                        meal_type: meal_type.clone(),
                        recipe_id: recipe.id.clone(),
                        recipe_title: recipe.title.clone(),
                    });
                }
            }

            save_meal_plans(&current);
            meal_plans.set(current);
        })
    };

    // Handler: Weekly Quantity Aggregation Engine
    let on_generate_groceries = {
        let meal_plans = meal_plans.clone();
        let recipes = recipes.clone();
        let notification = notification.clone();

        Callback::from(move |_| {
            let mut aggregated_map: HashMap<(String, String), (String, String, f64, bool)> = HashMap::new();

            for plan in meal_plans.iter() {
                if let Some(recipe) = recipes.iter().find(|r| r.id == plan.recipe_id) {
                    for ing in &recipe.ingredients {
                        let name_key = ing.name.trim().to_lowercase();
                        let unit_key = ing.unit.trim().to_lowercase();

                        let entry = aggregated_map.entry((name_key, unit_key)).or_insert((
                            ing.name.trim().to_string(),
                            ing.unit.trim().to_string(),
                            0.0,
                            ing.is_pantry_staple,
                        ));

                        entry.2 += ing.quantity;
                        if ing.is_pantry_staple {
                            entry.3 = true;
                        }
                    }
                }
            }

            let mut final_groceries: Vec<GroceryItem> = Vec::new();
            for ((_, _), (display_name, display_unit, qty, is_staple)) in aggregated_map {
                final_groceries.push(GroceryItem {
                    id: Uuid::new_v4().to_string(),
                    name: display_name,
                    quantity: qty,
                    unit: display_unit,
                    bought: false,
                    is_pantry_staple: is_staple,
                });
            }

            final_groceries.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

            save_groceries(&final_groceries);
            let count = final_groceries.len();
            notification.set(Some(format!(
                "Aggregated {} unique items into Grocery List.",
                count
            )));
        })
    };

    html! {
        <div class="planner-container">
            <div class="planner-header-actions">
                <button class="btn-generate-groceries" onclick={on_generate_groceries}>
                    {"Generate Weekly Groceries"}
                </button>
            </div>

            if let Some(msg) = &*notification {
                <div class="toast-notification">
                    <span>{ msg }</span>
                </div>
            }

            <div class="planner-grid">
                {
                    for DAYS.iter().map(|&day| {
                        html! {
                            <div class="day-card">
                                <h3 class="day-title">{ day }</h3>
                                <div class="day-slots">
                                    {
                                        for MEAL_TYPES.iter().map(|&meal_type| {
                                            let current_entry = meal_plans.iter().find(|e| e.day == day && e.meal_type == meal_type);
                                            let selected_id = current_entry.map(|e| e.recipe_id.clone()).unwrap_or_default();

                                            let day_s = day.to_string();
                                            let meal_s = meal_type.to_string();
                                            let select_meal_cb = on_select_meal.clone();

                                            let on_change = Callback::from(move |e: Event| {
                                                let select: web_sys::HtmlSelectElement = e.target_unchecked_into();
                                                select_meal_cb.emit((day_s.clone(), meal_s.clone(), select.value()));
                                            });

                                            html! {
                                                <div class="meal-slot">
                                                    <span class="slot-label">{ meal_type }</span>
                                                    <select value={selected_id.clone()} onchange={on_change}>
                                                        <option value="" selected={selected_id.is_empty()}>{"-- Select Recipe --"}</option>
                                                        {
                                                            for recipes.iter().map(|r| {
                                                                let is_sel = selected_id == r.id;
                                                                html! {
                                                                    <option value={r.id.clone()} selected={is_sel}>{ &r.title }</option>
                                                                }
                                                            })
                                                        }
                                                    </select>
                                                </div>
                                            }
                                        })
                                    }
                                </div>
                            </div>
                        }
                    })
                }
            </div>
        </div>
    }
}
