pub mod handlers;

use crate::server::handlers::{leptos_routes_handler, server_fn_handler};
use axum::{routing::get, Extension, Router};
use axum_session::{SessionConfig, SessionLayer, SessionMySqlPool, SessionStore};
use axum_session_auth::AuthConfig;
use leptos::*;

use leptos_axum::{generate_route_list, LeptosRoutes};
use portrait_booth::{
    auth::AuthSessionLayer,
    components::{
        app::*,
        home_page::HomePageRequest,
        login::*,
        login_otp::{LoginOtpRequest, LoginOtpVerifyRequest},
        logout::LogoutRequest,
        signup::SignupRequest,
    },
    fileserv::file_and_error_handler,
    GetUnitPrice,
};
use sqlx::mysql::MySqlPoolOptions;
use std::sync::Arc;

pub async fn server_main() {
    simple_logger::init_with_level(log::Level::Debug).expect("couldn't initialize logging");

    let dburl = dotenv!("DATABASE_URL");
    let pool = MySqlPoolOptions::new()
        .connect(dburl)
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

    _ = LoginRequest::register();
    _ = LoginOtpRequest::register();
    _ = LoginOtpVerifyRequest::register();
    _ = SignupRequest::register();
    _ = HomePageRequest::register();
    _ = LogoutRequest::register();
    _ = GetUnitPrice::register();

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
        .layer(Extension(Arc::new(leptos_options.clone())))
        .layer(Extension(pool));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
