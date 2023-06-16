use axum::{
    body::Body as AxumBody,
    extract::{Path, RawQuery, State},
    response::{IntoResponse, Response},
};
use http::{HeaderMap, Request};
use leptos::{log, provide_context, *};
use leptos_axum::handle_server_fns_with_context;

use crate::{
    auth::{AuthSession, Session},
    components::app::App,
};

use super::app_state::AppState;

pub async fn server_fn_handler(
    //Extension(pool): Extension<MySqlPool>,
    State(AppState { pool, .. }): State<AppState>,
    session: Session,
    auth_session: AuthSession,
    path: Path<String>,
    headers: HeaderMap,
    raw_query: RawQuery,
    request: Request<AxumBody>,
) -> impl IntoResponse {
    log!("{:?}", path);

    handle_server_fns_with_context(
        path,
        headers,
        raw_query,
        move |cx| {
            provide_context(cx, session.clone());
            provide_context(cx, auth_session.clone());
            provide_context(cx, pool.clone());
        },
        request,
    )
    .await
}

pub async fn leptos_routes_handler(
    //Extension(pool): Extension<MySqlPool>,
    //Extension(options): Extension<Arc<LeptosOptions>>,
    State(AppState {
        leptos_options: options,
        pool,
    }): State<AppState>,
    session: Session,
    auth_session: AuthSession,
    req: Request<AxumBody>,
) -> Response {
    let handler = leptos_axum::render_app_to_stream_with_context(
        options.clone(),
        move |cx| {
            provide_context(cx, session.clone());
            provide_context(cx, auth_session.clone());
            provide_context(cx, pool.clone());
        },
        |cx| view! { cx, <App/> },
    );
    handler(req).await.into_response()
}
