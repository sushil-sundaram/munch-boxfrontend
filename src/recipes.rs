use yew::prelude::*;
use shared::{IngredientItem, Recipe};
use uuid::Uuid;
use wasm_bindgen_futures::spawn_local;
use crate::api::{delete_recipe_from_backend, scrape_recipe_from_url};
use crate::icons::{IconGlobe, IconPlus, IconTrash, IconUtensils};
use crate::storage::{load_recipes, save_recipes};

#[derive(Clone, PartialEq)]
struct IngredientDraft {
    name: String,
    quantity: String,
    unit: String,
    is_pantry_staple: bool,
}

#[function_component(RecipeBook)]
pub fn recipe_book() -> Html {
    let recipes = use_state(|| load_recipes());
    let show_form = use_state(|| false);
    let show_url_form = use_state(|| false);
    let url_input = use_state(|| String::new());
    let is_scraping = use_state(|| false);
    let notification = use_state(|| Option::<String>::None);

    // Form fields for custom recipe
    let title_input = use_state(|| String::new());
    let instructions_input = use_state(|| String::new());
    let rating_input = use_state(|| 5u8);
    let ingredient_drafts = use_state(|| vec![
        IngredientDraft { name: String::new(), quantity: "1".to_string(), unit: "Pcs".to_string(), is_pantry_staple: false }
    ]);

    // Handler: Toggle Add Custom Form
    let toggle_form = {
        let show_form = show_form.clone();
        let show_url_form = show_url_form.clone();
        Callback::from(move |_| {
            show_form.set(!*show_form);
            show_url_form.set(false);
        })
    };

    // Handler: Toggle Import URL Form
    let toggle_url_form = {
        let show_form = show_form.clone();
        let show_url_form = show_url_form.clone();
        Callback::from(move |_| {
            show_url_form.set(!*show_url_form);
            show_form.set(false);
        })
    };

    // Handler: Delete Recipe
    let on_delete_recipe = {
        let recipes = recipes.clone();
        let notification = notification.clone();
        Callback::from(move |recipe_id: String| {
            let mut current = (*recipes).clone();
            if let Some(pos) = current.iter().position(|r| r.id == recipe_id) {
                let removed = current.remove(pos);
                save_recipes(&current);
                recipes.set(current);
                notification.set(Some(format!("Deleted recipe \"{}\"", removed.title)));

                let id_c = recipe_id.clone();
                spawn_local(async move {
                    let _ = delete_recipe_from_backend(&id_c).await;
                });
            }
        })
    };

    // Handler: Update Recipe Star Rating
    let on_rate_recipe = {
        let recipes = recipes.clone();
        Callback::from(move |(recipe_id, new_rating): (String, u8)| {
            let mut current = (*recipes).clone();
            if let Some(r) = current.iter_mut().find(|r| r.id == recipe_id) {
                r.rating = new_rating;
                save_recipes(&current);
                recipes.set(current);
            }
        })
    };

    // Handler: Scrape Recipe from Web / YouTube URL
    let on_scrape_url = {
        let recipes = recipes.clone();
        let url_input = url_input.clone();
        let is_scraping = is_scraping.clone();
        let notification = notification.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let url = (*url_input).trim().to_string();
            if !url.is_empty() {
                let recipes = recipes.clone();
                let url_input = url_input.clone();
                let is_scraping = is_scraping.clone();
                let notification = notification.clone();

                is_scraping.set(true);
                notification.set(Some("Scraping recipe content & ingredients...".to_string()));

                spawn_local(async move {
                    match scrape_recipe_from_url(&url).await {
                        Ok(scraped) => {
                            let mut current = (*recipes).clone();
                            let ing_count = scraped.ingredients.len();
                            let title = scraped.title.clone();

                            current.push(scraped);
                            save_recipes(&current);
                            recipes.set(current);

                            url_input.set(String::new());
                            is_scraping.set(false);
                            notification.set(Some(format!("Imported \"{}\" ({} ingredients)", title, ing_count)));
                        }
                        Err(err) => {
                            is_scraping.set(false);
                            notification.set(Some(format!("Error: {}", err)));
                        }
                    }
                });
            }
        })
    };

    // Handler: Add Ingredient Row
    let add_ingredient_row = {
        let ingredient_drafts = ingredient_drafts.clone();
        Callback::from(move |_| {
            let mut drafts = (*ingredient_drafts).clone();
            drafts.push(IngredientDraft {
                name: String::new(),
                quantity: "1".to_string(),
                unit: "Pcs".to_string(),
                is_pantry_staple: false,
            });
            ingredient_drafts.set(drafts);
        })
    };

    // Handler: Remove Ingredient Row
    let remove_ingredient_row = {
        let ingredient_drafts = ingredient_drafts.clone();
        Callback::from(move |index: usize| {
            let mut drafts = (*ingredient_drafts).clone();
            if drafts.len() > 1 {
                drafts.remove(index);
                ingredient_drafts.set(drafts);
            }
        })
    };

    // Handler: Add Custom Recipe
    let on_add_recipe = {
        let recipes = recipes.clone();
        let show_form = show_form.clone();
        let title = title_input.clone();
        let rating = rating_input.clone();
        let instructions = instructions_input.clone();
        let ingredient_drafts = ingredient_drafts.clone();
        let notification = notification.clone();

        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let t = (*title).trim().to_string();
            if !t.is_empty() {
                let parsed_ingredients: Vec<IngredientItem> = ingredient_drafts
                    .iter()
                    .filter(|d| !d.name.trim().is_empty())
                    .map(|d| IngredientItem {
                        name: d.name.trim().to_string(),
                        quantity: d.quantity.parse::<f64>().unwrap_or(1.0),
                        unit: d.unit.trim().to_string(),
                        is_pantry_staple: d.is_pantry_staple,
                    })
                    .collect();

                let mut current = (*recipes).clone();
                let new_recipe = Recipe {
                    id: Uuid::new_v4().to_string(),
                    title: t.clone(),
                    ingredients: parsed_ingredients,
                    instructions: (*instructions).clone(),
                    emoji: "".to_string(),
                    rating: *rating,
                };

                current.push(new_recipe);
                save_recipes(&current);
                recipes.set(current);

                // Reset form
                title.set(String::new());
                instructions.set(String::new());
                ingredient_drafts.set(vec![IngredientDraft { name: String::new(), quantity: "1".to_string(), unit: "Pcs".to_string(), is_pantry_staple: false }]);
                show_form.set(false);
                notification.set(Some(format!("Added \"{}\" to recipe book", t)));
            }
        })
    };

    html! {
        <div class="recipe-book-container">
            <div class="recipe-header-actions">
                <button class="btn-toggle-url" onclick={toggle_url_form}>
                    <IconGlobe />
                    <span>{ if *show_url_form { "Close Importer" } else { "Import via URL" } }</span>
                </button>
                <button class="btn-toggle-form" onclick={toggle_form}>
                    <IconPlus />
                    <span>{ if *show_form { "Cancel" } else { "Add Recipe" } }</span>
                </button>
            </div>

            if let Some(msg) = &*notification {
                <div class="toast-notification">
                    <span>{ msg }</span>
                </div>
            }

            if *show_url_form {
                <form class="url-scrape-form" onsubmit={on_scrape_url}>
                    <h3>{"Import Recipe from Web / YouTube"}</h3>
                    <p class="scrape-help">{"Paste any food blog URL or YouTube video link to extract recipe ingredients and instructions."}</p>
                    <div class="url-input-row">
                        <input 
                            type="url"
                            placeholder="https://www.youtube.com/watch?v=... or https://recipeblog.com/..."
                            value={(*url_input).clone()}
                            required=true
                            disabled={*is_scraping}
                            oninput={let url_input = url_input.clone(); Callback::from(move |e: InputEvent| {
                                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                url_input.set(input.value());
                            })}
                        />
                        <button type="submit" class="btn-scrape" disabled={*is_scraping}>
                            { if *is_scraping { "Scraping..." } else { "Scrape Recipe" } }
                        </button>
                    </div>
                </form>
            }

            if *show_form {
                <form class="recipe-form" onsubmit={on_add_recipe}>
                    <h3>{"Create Custom Recipe"}</h3>
                    
                    <div class="form-row flex-1">
                        <label>{"Recipe Title"}</label>
                        <input 
                            type="text" 
                            placeholder="e.g. Creamy Mushroom Risotto" 
                            value={(*title_input).clone()} 
                            required=true
                            oninput={let title = title_input.clone(); Callback::from(move |e: InputEvent| {
                                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                title.set(input.value());
                            })} 
                        />
                    </div>

                    <div class="form-row">
                        <label>{"Rating"}</label>
                        <div class="star-rating-selector">
                            {
                                for (1..=5).map(|star_val| {
                                    let rating_input = rating_input.clone();
                                    let is_active = star_val <= *rating_input;
                                    html! {
                                        <span 
                                            class={if is_active { "star active" } else { "star" }}
                                            onclick={Callback::from(move |_| rating_input.set(star_val))}
                                        >
                                            { if is_active { "★" } else { "☆" } }
                                        </span>
                                    }
                                })
                            }
                        </div>
                    </div>

                    <div class="form-section-title">{"Ingredients"}</div>
                    {
                        for ingredient_drafts.iter().enumerate().map(|(idx, draft)| {
                            let drafts_c = ingredient_drafts.clone();
                            let drafts_c2 = ingredient_drafts.clone();
                            let drafts_c3 = ingredient_drafts.clone();
                            let drafts_c4 = ingredient_drafts.clone();
                            let remove_cb = remove_ingredient_row.reform(move |_| idx);

                            html! {
                                <div class="ingredient-row-edit">
                                    <input 
                                        type="number" 
                                        step="any"
                                        style="width: 70px;"
                                        placeholder="Qty"
                                        value={draft.quantity.clone()}
                                        oninput={Callback::from(move |e: InputEvent| {
                                            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                            let mut list = (*drafts_c).clone();
                                            list[idx].quantity = input.value();
                                            drafts_c.set(list);
                                        })}
                                    />
                                    <input 
                                        type="text" 
                                        style="width: 75px;"
                                        placeholder="Unit"
                                        value={draft.unit.clone()}
                                        oninput={Callback::from(move |e: InputEvent| {
                                            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                            let mut list = (*drafts_c2).clone();
                                            list[idx].unit = input.value();
                                            drafts_c2.set(list);
                                        })}
                                    />
                                    <input 
                                        type="text" 
                                        class="flex-1"
                                        placeholder="Ingredient Name"
                                        value={draft.name.clone()}
                                        oninput={Callback::from(move |e: InputEvent| {
                                            let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                            let mut list = (*drafts_c3).clone();
                                            list[idx].name = input.value();
                                            drafts_c3.set(list);
                                        })}
                                    />
                                    <label class="staple-checkbox" title="Mark as pantry staple">
                                        <input 
                                            type="checkbox"
                                            checked={draft.is_pantry_staple}
                                            onchange={Callback::from(move |e: Event| {
                                                let input: web_sys::HtmlInputElement = e.target_unchecked_into();
                                                let mut list = (*drafts_c4).clone();
                                                list[idx].is_pantry_staple = input.checked();
                                                drafts_c4.set(list);
                                            })}
                                        />
                                        <span>{"Staple"}</span>
                                    </label>
                                    <button type="button" class="btn-remove-row" onclick={remove_cb}>
                                        <IconTrash />
                                    </button>
                                </div>
                            }
                        })
                    }

                    <button type="button" class="btn-add-row" onclick={add_ingredient_row}>
                        <IconPlus />
                        <span>{"Add Ingredient"}</span>
                    </button>

                    <div class="form-row">
                        <label>{"Instructions"}</label>
                        <textarea 
                            rows="3" 
                            placeholder="Step-by-step instructions..."
                            value={(*instructions_input).clone()}
                            oninput={let inst = instructions_input.clone(); Callback::from(move |e: InputEvent| {
                                let input: web_sys::HtmlTextAreaElement = e.target_unchecked_into();
                                inst.set(input.value());
                            })}
                        />
                    </div>

                    <button type="submit" class="btn-save-recipe">{"Save Recipe"}</button>
                </form>
            }

            <div class="recipes-grid">
                {
                    for recipes.iter().map(|recipe| {
                        let recipe_id = recipe.id.clone();
                        let delete_cb = on_delete_recipe.reform(move |_| recipe_id.clone());

                        html! {
                            <div class="recipe-card">
                                <div class="card-header">
                                    <div class="card-title-group">
                                        <div class="card-title-row">
                                            <IconUtensils />
                                            <h4 class="card-title">{ &recipe.title }</h4>
                                        </div>
                                        <div class="star-rating">
                                            {
                                                for (1..=5).map(|star_val| {
                                                    let recipe_id = recipe.id.clone();
                                                    let on_rate_recipe = on_rate_recipe.clone();
                                                    let is_active = star_val <= recipe.rating;
                                                    let rate_cb = Callback::from(move |_| {
                                                        on_rate_recipe.emit((recipe_id.clone(), star_val));
                                                    });
                                                    html! {
                                                        <span 
                                                            class={if is_active { "star active" } else { "star" }}
                                                            onclick={rate_cb}
                                                            title={format!("Rate {} stars", star_val)}
                                                        >
                                                            { if is_active { "★" } else { "☆" } }
                                                        </span>
                                                    }
                                                })
                                            }
                                        </div>
                                    </div>
                                    <button class="btn-delete-recipe" title="Delete Recipe" onclick={delete_cb}>
                                        <IconTrash />
                                    </button>
                                </div>

                                <div class="card-body">
                                    <div class="section-title">{"Ingredients"}</div>
                                    <ul class="ingredients-list">
                                        {
                                            for recipe.ingredients.iter().map(|ing| {
                                                let formatted = if ing.unit.is_empty() {
                                                    format!("{} {}", ing.quantity, ing.name)
                                                } else {
                                                    format!("{} {} {}", ing.quantity, ing.unit, ing.name)
                                                };
                                                html! {
                                                    <li>
                                                        <span>{ formatted }</span>
                                                        { if ing.is_pantry_staple { html! { <span class="badge-staple">{"Pantry"}</span> } } else { html! {} } }
                                                    </li>
                                                }
                                            })
                                        }
                                    </ul>

                                    <div class="section-title" style="margin-top: 0.6rem;">{"Instructions"}</div>
                                    <p class="instructions-text">{ &recipe.instructions }</p>
                                </div>
                            </div>
                        }
                    })
                }
            </div>
        </div>
    }
}
