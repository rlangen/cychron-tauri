mod components {
    pub mod graphcet;
}
use components::graphcet::Graphcet;

mod services {
    pub mod uuid_service;
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<Graphcet>::new().render();
}
