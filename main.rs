use graphcet::Graphcet;

use yew::prelude::*;
mod graphcet; // Import the module if `MyComponent` is in a separate file

#[function_component]
fn App() -> Html {
    html! {
        <Graphcet /> // Use `MyComponent` here
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<Graphcet>::new().render();
}
