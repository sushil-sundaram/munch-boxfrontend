mod api;
mod grocery;
mod icons;
mod planner;
mod recipes;
mod storage;

use grocery::GroceryList;
use icons::{IconBook, IconCalendar, IconCart, IconChefHat};
use planner::MealPlanner;
use recipes::RecipeBook;
use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let active_tab = use_state(|| "plan".to_string());

    let switch_tab = {
        let active_tab = active_tab.clone();
        move |tab_name: &str| {
            let active_tab = active_tab.clone();
            let tab_name = tab_name.to_string();
            Callback::from(move |_| active_tab.set(tab_name.clone()))
        }
    };

    let tab_class = |tab: &str| {
        if *active_tab == tab {
            "active"
        } else {
            ""
        }
    };

    let style_display = |tab: &str| {
        if *active_tab == tab {
            "display: block;"
        } else {
            "display: none;"
        }
    };

    html! {
        <div class="app">
            <header class="header">
                <div class="logo-group">
                    <IconChefHat />
                    <span class="header-logo">{"MUNCHBOX"}</span>
                </div>
            </header>

            <main class="content">
                <div style={style_display("recipes")}>
                    <h2>{"Recipe Book"}</h2>
                    <RecipeBook />
                </div>

                <div style={style_display("plan")}>
                    <h2>{"Weekly Planner"}</h2>
                    <MealPlanner />
                </div>

                <div style={style_display("grocery")}>
                    <h2>{"Grocery Checklist"}</h2>
                    <GroceryList />
                </div>
            </main>

            <nav class="nav">
                <button class={tab_class("recipes")} onclick={switch_tab("recipes")}>
                    <IconBook />
                    <span>{"Recipes"}</span>
                </button>
                <button class={tab_class("plan")} onclick={switch_tab("plan")}>
                    <IconCalendar />
                    <span>{"Planner"}</span>
                </button>
                <button class={tab_class("grocery")} onclick={switch_tab("grocery")}>
                    <IconCart />
                    <span>{"Grocery"}</span>
                </button>
            </nav>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
