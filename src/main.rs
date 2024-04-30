mod components {
  pub mod component_tuple_structs;
  pub mod graphcet;
  pub mod net_button;
  pub mod net_user_control;
}
use components::graphcet::Graphcet;

mod services {
  pub mod logging_service;
  pub mod uuid_service;
}

fn main() {
  wasm_logger::init(wasm_logger::Config::default());
  yew::Renderer::<Graphcet>::new().render();
}
