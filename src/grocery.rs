use yew::prelude::*;
use shared::GroceryItem;
use uuid::Uuid;
use wasm_bindgen_futures::spawn_local;
use crate::storage::{load_groceries, save_groceries};
use crate::api::sync_groceries_with_backend;
use crate::icons::{IconBroom, IconPlus, IconSync};

#[derive(Clone, PartialEq, Debug)]
pub enum SyncStatus {
    Idle,
    Syncing,
    Success,
    Error(String),
}

#[function_component(GroceryList)]
pub fn grocery_list() -> Html {
    let groceries = use_state(|| load_groceries());
    let input_name = use_state(|| String::new());
    let input_qty = use_state(|| "1".to_string());
    let input_unit = use_state(|| "Pcs".to_string());
    let hide_staples = use_state(|| false);
    let sync_status = use_state(|| SyncStatus::Idle);

    // Handler: Trigger Backend Sync
    let on_sync = {
        let groceries = groceries.clone();
        let sync_status = sync_status.clone();
        Callback::from(move |_| {
            let groceries = groceries.clone();
            let sync_status = sync_status.clone();
            sync_status.set(SyncStatus::Syncing);

            spawn_local(async move {
                let current_items = (*groceries).clone();
                match sync_groceries_with_backend(&current_items).await {
                    Ok(synced_items) => {
                        save_groceries(&synced_items);
                        groceries.set(synced_items);
                        sync_status.set(SyncStatus::Success);
                    }
                    Err(err) => {
                        sync_status.set(SyncStatus::Error(err));
                    }
                }
            });
        })
    };

    // Handler: Add a new item
    let on_add = {
        let groceries = groceries.clone();
        let input_name = input_name.clone();
        let input_qty = input_qty.clone();
        let input_unit = input_unit.clone();

        Callback::from(move |_| {
            let name = (*input_name).trim().to_string();
            if !name.is_empty() {
                let mut current_items = (*groceries).clone();
                current_items.push(GroceryItem {
                    id: Uuid::new_v4().to_string(),
                    name: name.clone(),
                    quantity: input_qty.parse::<f64>().unwrap_or(1.0),
                    unit: (*input_unit).trim().to_string(),
                    bought: false,
                    is_pantry_staple: false,
                });
                
                save_groceries(&current_items);
                groceries.set(current_items);
                input_name.set(String::new());
            }
        })
    };

    // Handler: Toggle bought status
    let on_toggle = {
        let groceries = groceries.clone();
        Callback::from(move |id: String| {
            let mut current_items = (*groceries).clone();
            if let Some(item) = current_items.iter_mut().find(|i| i.id == id) {
                item.bought = !item.bought;
            }
            save_groceries(&current_items);
            groceries.set(current_items);
        })
    };

    // Handler: Sweep completed items
    let on_sweep = {
        let groceries = groceries.clone();
        Callback::from(move |_| {
            let mut current_items = (*groceries).clone();
            current_items.retain(|item| !item.bought);
            save_groceries(&current_items);
            groceries.set(current_items);
        })
    };

    // Handler: Toggle Hide Staples
    let toggle_hide_staples = {
        let hide_staples = hide_staples.clone();
        Callback::from(move |_| hide_staples.set(!*hide_staples))
    };

    // Render Sync Badge
    let sync_badge = match &*sync_status {
        SyncStatus::Idle => html! { <span class="sync-status idle">{"Offline Mode"}</span> },
        SyncStatus::Syncing => html! { <span class="sync-status syncing">{"Syncing..."}</span> },
        SyncStatus::Success => html! { <span class="sync-status success">{"Synced with Server"}</span> },
        SyncStatus::Error(msg) => html! { <span class="sync-status error" title={msg.clone()}>{"Offline (Sync Error)"}</span> },
    };

    let filtered_groceries: Vec<GroceryItem> = groceries
        .iter()
        .filter(|item| !(*hide_staples && item.is_pantry_staple))
        .cloned()
        .collect();

    html! {
        <div class="grocery-container">
            <div class="grocery-header">
                <div class="sync-bar">
                    { sync_badge }
                    <button class="btn-sync" onclick={on_sync} disabled={*sync_status == SyncStatus::Syncing}>
                        <IconSync />
                        <span>{"Sync API"}</span>
                    </button>
                </div>
            </div>

            // Input Area with Quantity & Unit
            <div class="input-row">
                <input 
                    type="number"
                    step="any"
                    style="width: 70px;"
                    placeholder="Qty"
                    value={(*input_qty).clone()}
                    oninput={let input_qty = input_qty.clone(); Callback::from(move |e: InputEvent| {
                        let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                        input_qty.set(input.value());
                    })}
                />
                <input 
                    type="text" 
                    style="width: 75px;"
                    placeholder="Unit" 
                    value={(*input_unit).clone()}
                    oninput={let input_unit = input_unit.clone(); Callback::from(move |e: InputEvent| {
                        let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                        input_unit.set(input.value());
                    })}
                />
                <input 
                    type="text" 
                    style="flex: 1;"
                    placeholder="Add item (e.g. Milk, Avocados)..." 
                    value={(*input_name).clone()}
                    oninput={let input_name = input_name.clone(); Callback::from(move |e: InputEvent| {
                        let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                        input_name.set(input.value());
                    })}
                />
                <button class="btn-add" onclick={on_add}>
                    <IconPlus />
                    <span>{"Add"}</span>
                </button>
            </div>

            // Action Row: Sweep Completed & Hide Pantry Staples
            <div class="actions-row">
                <button class="btn-toggle-staples" onclick={toggle_hide_staples}>
                    { if *hide_staples { "Show Pantry Staples" } else { "Hide Pantry Staples" } }
                </button>
                <button class="btn-sweep" onclick={on_sweep}>
                    <IconBroom />
                    <span>{"Sweep Completed"}</span>
                </button>
            </div>

            // List Area
            <ul class="grocery-list">
                {
                    for filtered_groceries.iter().map(|item| {
                        let id = item.id.clone();
                        let is_bought = item.bought;
                        let toggle = on_toggle.reform(move |_| id.clone());
                        let item_class = if is_bought { "grocery-item bought" } else { "grocery-item" };

                        let qty_display = if item.unit.is_empty() {
                            format!("{}", item.quantity)
                        } else {
                            format!("{} {}", item.quantity, item.unit)
                        };

                        html! {
                            <li class={item_class} onclick={toggle}>
                                <div class="item-info">
                                    <span class="item-qty-badge">{ qty_display }</span>
                                    <span class="item-name">{ &item.name }</span>
                                    { if item.is_pantry_staple { html! { <span class="badge-staple">{"Pantry"}</span> } } else { html! {} } }
                                </div>
                                <span class="item-checkbox">
                                    { if is_bought { "[x]" } else { "[ ]" } }
                                </span>
                            </li>
                        }
                    })
                }
            </ul>
        </div>
    }
}