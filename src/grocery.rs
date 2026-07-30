use yew::prelude::*;
use shared::GroceryItem;
use uuid::Uuid;
use crate::storage::{load_groceries, save_groceries};

#[function_component(GroceryList)]
pub fn grocery_list() -> Html {
    // 1. Initialize state from LocalStorage
    let groceries = use_state(|| load_groceries());
    let input_value = use_state(|| String::new());

    // 2. Handler: Add a new item
    let on_add = {
        let groceries = groceries.clone();
        let input_value = input_value.clone();
        Callback::from(move |_| {
            let name = (*input_value).trim().to_string();
            if !name.is_empty() {
                let mut current_items = (*groceries).clone();
                
                current_items.push(GroceryItem {
                    id: Uuid::new_v4().to_string(),
                    name: name.clone(),
                    bought: false,
                });
                
                save_groceries(¤t_items);   // Save offline
                groceries.set(current_items);     // Trigger UI re-render
                input_value.set(String::new());   // Clear the input
            }
        })
    };

    // 3. Handler: Toggle 'bought' status
    let on_toggle = {
        let groceries = groceries.clone();
        Callback::from(move |id: String| {
            let mut current_items = (*groceries).clone();
            if let Some(item) = current_items.iter_mut().find(|i| i.id == id) {
                item.bought = !item.bought;
            }
            save_groceries(¤t_items);
            groceries.set(current_items);
        })
    };

    // 4. Handler: Update input state as user types
    let on_input = {
        let input_value = input_value.clone();
        Callback::from(move |e: InputEvent| {
            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
            input_value.set(input.value());
        })
    };

    // 5. Render the Midnight Cartoon UI
    html! {
        
            
            // Input Area
            
                
                
                    {"➕ Add"}
                
            

            // List Area
            
                {
                    for groceries.iter().map(|item| {
                        let id = item.id.clone();
                        let is_bought = item.bought;
                        let toggle = on_toggle.reform(move |_| id.clone());
                        
                        html! {
                            
                                
                                    { &item.name }
                                
                                
                                    { if is_bought { "✅" } else { "🛒" } }
                                
                            
                        }
                    })
                }
            
        
    }
}