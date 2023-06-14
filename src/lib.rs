use cfg_if::cfg_if;
use leptos::ServerFnError;
pub mod components;
//pub mod error_template;
pub mod fileserv;
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
        #[macro_use]
        extern crate dotenv_codegen;

        pub mod auth;

        use sqlx::MySqlPool;
        pub fn pool(cx: leptos::Scope) -> Result<MySqlPool, leptos::ServerFnError> {
            leptos::use_context::<MySqlPool>(cx)
                .ok_or("db pool missing")
                .map_err(|e| leptos::ServerFnError::ServerError(e.to_string()))
        }
        pub fn get_totp_duration() -> u64 {
            let dur = dotenv!("TOTP_DURATION");
            let dur = dur.parse().expect("TOTP_DURATION should be set");
            leptos::log!("TOTP_DURATION: {}s",dur);
            dur
        }

        use std::error::Error;
        pub fn to_server_fn_error(e: impl Error) -> ServerFnError {
            ServerFnError::ServerError(e.to_string())
        }
    }
}
