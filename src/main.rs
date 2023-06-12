use cfg_if::cfg_if;
cfg_if! { if #[cfg(feature = "ssr")] {

#[macro_use]
extern crate dotenv_codegen;

use rand::Rng;
use axum_login::{axum_sessions::SessionLayer, AuthLayer, MySqlStore};
use axum::extract::State;
use axum::{extract::Extension, Router};
use leptos::*;
use leptos_axum::{generate_route_list, LeptosRoutes};
use portrait_booth::components::app::*;
use portrait_booth::components::login::*;
use portrait_booth::fileserv::file_and_error_handler;
use std::sync::Arc;
pub mod models;
pub mod auth;
pub mod state;
use crate::models::user::*;
use axum::body::Body as AxumBody;
use axum::extract::{Path, RawQuery};
use axum::http::Request;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use http::HeaderMap;
use leptos_axum::handle_server_fns_with_context;
use sqlx::mysql::MySqlPoolOptions;
use sqlx::MySqlPool;
async fn server_fn_handler(
    Extension(pool): Extension<MySqlPool>,
    //auth: AuthContext,
    path: Path<String>,
    headers: HeaderMap,
    raw_query: RawQuery,
    request: Request<AxumBody>,
) -> impl IntoResponse {
    log!("{:?}", path);

    //log!("auth: {:#?}",auth);
    handle_server_fns_with_context(
        path,
        headers,
        raw_query,
        move |cx| {
            provide_context(cx, pool.clone());
        },
        request,
    )
    .await
}

type AuthContext = axum_login::extractors::AuthContext<i64,User,MySqlStore<User>>;
async fn leptos_routes_handler(
    Extension(pool): Extension<MySqlPool>,
    State(options): State<Arc<LeptosOptions>>,
    //auth: AuthContext,
    req: Request<AxumBody>,
) -> Response {
    //log!("auth: {:#?}",auth);
    let handler = leptos_axum::render_app_to_stream_with_context(
        (*options).clone(),
        move |cx| {
            provide_context(cx, pool.clone());
        },
        |cx| view! { cx, <App/> },
    );
    handler(req).await.into_response()
}

#[tokio::main]
async fn main() {
    simple_logger::init_with_level(log::Level::Debug).expect("couldn't initialize logging");

    let dburl = dotenv!("DATABASE_URL");
    let pool = MySqlPoolOptions::new()
        .connect(dburl)
        .await
        .expect("Could not connect to MySQL");

    sqlx::migrate!().run(&pool).await.expect("Could not run SQLX migrations");
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

    let secret:Vec<u8> = (0..64).map(|_| rand::thread_rng().gen::<u8>()).collect();

    let session_store = axum_login::axum_sessions::async_session::MemoryStore::new();
        let session_layer = SessionLayer::new(session_store, &secret).with_secure(false);

        let user_store = MySqlStore::<User>::new(pool.clone());
        let auth_layer = AuthLayer::new(user_store, &secret);

    // build our application with a route
let app = Router::new()
        .route("/api/*fn_name", get(server_fn_handler).post(server_fn_handler))
        //.leptos_routes_with_handler(routes, get(leptos_routes_handler) )
        .leptos_routes(leptos_options.clone(), routes, |cx| view!{cx, <App/>} )
        .fallback(file_and_error_handler)
        .layer(auth_layer)
        .layer(session_layer)
        .layer(Extension(pool))
        .layer(Extension(Arc::new(leptos_options)));
        //.with_state(leptos_options);
    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    log!("listening on http://{}", &addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
   }

}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for a purely client-side app
    // see lib.rs for hydration function instead
}
