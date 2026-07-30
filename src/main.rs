mod storage;
mod grocery;

use yew::prelude::*;
use grocery::GroceryList; // Bring the component into scope

#[function_component(App)]
fn app() -> Html {
    let active_tab = use_state(|| "recipes".to_string());

    // 2. CALLBACK: A function to change the active tab when a button is clicked.
    let switch_tab = {
        let active_tab = active_tab.clone();
        move |tab_name: &str| {
            let active_tab = active_tab.clone();
            let tab_name = tab_name.to_string(); // <--- Convert to owned String here
            Callback::from(move |_| active_tab.set(tab_name.clone())) // <--- Clone the string for each click
        }
    };
    
    let tab_class = |tab: &str| {
        if *active_tab == tab {
            "active"
        } else {
            ""
        }
    };

    html! {
        <div class="app">
            <header class="header">
                <span class="header-emoji">{"🍳"}</span>
                <span>{"MunchBox"}</span>
            </header>

            <main class="content">
                {
                    if *active_tab == "recipes" {
                        html! {
                            <>
                                <h2>{"📖 Recipe Book"}</h2>
                                <p>{"(Recipes will go here in Module 5!)"}</p>
                            </>
                        }
                    } else if *active_tab == "plan" {
                        html! {
                            <>
                                <h2>{"📅 Weekly Calendar"}</h2>
                                <p>{"(Planning grid goes here!)"}</p>
                            </>
                        }
                    } else {
                        html! {
                            <>
                                <h2>{"🛒 Grocery Checklist"}</h2>
                                <p>{"(Auto-generated list goes here!)"}</p>
                            </>
                        }
                    }
                }
                {
        if *active_tab == "groceries" {
            html! {  }
        } else {
            html! { 
{"Placeholder for "}{ &*active_tab }
 }
        }
    }
            </main>

            <nav class="nav">
                <button class={tab_class("recipes")} onclick={switch_tab("recipes")}>
                    <span>{"📖"}</span>
                    <span>{"Recipes"}</span>
                </button>
                <button class={tab_class("plan")} onclick={switch_tab("plan")}>
                    <span>{"📅"}</span>
                    <span>{"Planner"}</span>
                </button>
                <button class={tab_class("grocery")} onclick={switch_tab("grocery")}>
                    <span>{"🛒"}</span>
                    <span>{"Grocery"}</span>
                </button>
            </nav>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
