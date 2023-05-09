pub mod components;
pub mod routes;

use components::App;

#[macro_use]
extern crate dotenv_codegen;

pub static BASE_URL: &str = dotenv!("API_BASE_URL");

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
