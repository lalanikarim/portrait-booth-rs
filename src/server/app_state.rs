use axum::extract::FromRef;
use leptos::LeptosOptions;
use sqlx::MySqlPool;

#[derive(Clone, Debug, FromRef)]
pub struct AppState {
    pub leptos_options: LeptosOptions,
    pub pool: MySqlPool,
}
