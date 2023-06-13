use cfg_if::cfg_if;
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
        pub mod auth;

use sqlx::MySqlPool;
pub fn pool(cx: leptos::Scope) -> Result<MySqlPool, leptos::ServerFnError> {
    leptos::use_context::<MySqlPool>(cx)
        .ok_or("db pool missing")
        .map_err(|e| leptos::ServerFnError::ServerError(e.to_string()))
}
    }
}

pub fn validate_username(username: &str) -> Result<(), String> {
    todo!();
}
pub fn validate_password(
    password: String,
    confirm_password: String,
) -> Result<(), Vec<&'static str>> {
    let mut error: Vec<&str> = Vec::new();
    if password != confirm_password {
        error.push("Passwords don't match.");
    }
    if password.len() < 8 {
        error.push("Must contain at least 8 characters.");
    }
    if password.find(|c: char| c.is_lowercase()).is_none() {
        error.push("Must contain at least one lowercase letter.");
    }
    if password.find(|c: char| c.is_uppercase()).is_none() {
        error.push("Must contain at least one uppercase letter.");
    }
    if password.find(|c: char| c.is_numeric()).is_none() {
        error.push("Must contain at least one number.");
    }
    if password.find(|c: char| "!@#$%^&*".contains(c)).is_none() {
        error.push("Must contain at least one symbol: !@#$%^&*");
    }
    if error.len() == 0 {
        Ok(())
    } else {
        Err(error)
    }
}
