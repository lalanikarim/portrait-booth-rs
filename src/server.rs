pub mod app_state;
pub mod fileserv;
pub mod handlers;
pub mod storage;
pub mod stripe;

use crate::server::{
    app_state::AppState,
    fileserv::file_and_error_handler,
    handlers::{leptos_routes_handler, server_fn_handler},
};
use axum::{routing::get, Router};
use axum_session::{SessionConfig, SessionLayer, SessionMySqlPool, SessionStore};
use axum_session_auth::AuthConfig;
use leptos::*;

use crate::{auth::AuthSessionLayer, components::app::*};
use leptos_axum::{generate_route_list, LeptosRoutes};
use sqlx::mysql::MySqlPoolOptions;

pub async fn server_main() {
    use dotenv;
    simple_logger::init_with_level(log::Level::Debug).expect("couldn't initialize logging");

    let env_vars = [
        "APP_URL",
        "DATABASE_URL",
        "STRIPE_KEY",
        "PHOTO_PRICING_ID",
        "PHOTO_UNIT_PRICE",
        "TOTP_DURATION",
    ];

    env_vars.iter().for_each(|key| {
        let error = format!("{} env variable should be present", key);
        dotenv::var(key).expect(&error);
    });

    let dburl = dotenv::var("DATABASE_URL").expect("DATABASE_URL env variable should be present");
    let pool = MySqlPoolOptions::new()
        .connect(&dburl)
        .await
        .expect("Could not connect to MySQL");
    let session_config = SessionConfig::default().with_table_name("axum_sessions");
    let auth_config = AuthConfig::<u64>::default().with_anonymous_user_id(Some(0));
    let session_store =
        SessionStore::<SessionMySqlPool>::new(Some(pool.clone().into()), session_config);
    session_store.initiate().await.unwrap();

    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Could not run SQLX migrations");
    // Setting get_configuration(None) means we'll be using cargo-leptos's env values
    // For deployment these variables are:
    // <https://github.com/leptos-rs/start-axum#executing-a-server-on-a-remote-machine-without-the-toolchain>
    // Alternately a file can be specified such as Some("Cargo.toml")
    // The file would need to be included with the executable when moved to deployment
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = conf.leptos_options;
    let addr = leptos_options.site_addr;
    let routes = generate_route_list(|cx| view! { cx, <App/> }).await;

    let app_state = AppState {
        leptos_options,
        pool: pool.clone(),
    };
    // build our application with a route
    let app = Router::new()
        .route(
            "/api/*fn_name",
            get(server_fn_handler).post(server_fn_handler),
        )
        .leptos_routes_with_handler(routes, get(leptos_routes_handler))
        .fallback(file_and_error_handler)
        .layer(AuthSessionLayer::new(Some(pool.clone())).with_config(auth_config))
        .layer(SessionLayer::new(session_store))
        .with_state(app_state);
    //.layer(Extension(Arc::new(leptos_options.clone())))
    //.layer(Extension(pool));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
use std::error::Error;
pub fn to_server_fn_error(e: impl Error) -> ServerFnError {
    ServerFnError::ServerError(e.to_string())
}
use sqlx::MySqlPool;
pub fn pool(cx: leptos::Scope) -> Result<MySqlPool, leptos::ServerFnError> {
    leptos::use_context::<MySqlPool>(cx)
        .ok_or("db pool missing")
        .map_err(|e| leptos::ServerFnError::ServerError(e.to_string()))
}
pub fn get_totp_duration() -> u64 {
    let dur = dotenv::var("TOTP_DURATION").unwrap_or("3600".into());
    let dur = dur.parse().expect("TOTP_DURATION should be set");
    leptos::log!("TOTP_DURATION: {}s", dur);
    dur
}
