use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")]{
        use leptos::LeptosOptions;
        use sqlx::MySqlPool;
        use axum::extract::FromRef;

        #[derive(FromRef, Debug, Clone)]
        pub struct AppState {
            pub leptos_options: LeptosOptions,
            pub pool: MySqlPool
        }
    }
}
