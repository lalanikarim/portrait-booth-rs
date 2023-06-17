use cfg_if::cfg_if;
pub mod components;
//pub mod error_template;
pub mod models;
cfg_if! { if #[cfg(feature = "hydrate")] {
    use leptos::*;
    use wasm_bindgen::prelude::wasm_bindgen;
    use crate::components::app::*;

    #[wasm_bindgen]
    pub fn hydrate() {
        // initializes logging using the `log` crate
        _ = console_log::init_with_level(log::Level::Debug);
        console_error_panic_hook::set_once();

        leptos::mount_to_body(move |cx| {
            view! { cx, <App/> }
        });
    }
}}
cfg_if! {
    if #[cfg(feature = "ssr")] {

        pub mod auth;
        pub mod server;
        pub use server::{pool,get_totp_duration,to_server_fn_error};

    }
}
