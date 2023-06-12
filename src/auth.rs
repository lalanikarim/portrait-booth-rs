use crate::models::user::*;
use axum::async_trait;
use axum_session::SessionMySqlPool;
use axum_session_auth::Authentication;
use leptos::{use_context, Scope, ServerFnError};
use sqlx::MySqlPool;

pub type AuthSession = axum_session_auth::AuthSession<User, i64, SessionMySqlPool, MySqlPool>;
pub fn auth(cx: Scope) -> Result<AuthSession, ServerFnError> {
    use_context::<AuthSession>(cx)
        .ok_or("Auth session missing")
        .map_err(|e| ServerFnError::ServerError(e.to_string()))
}

#[async_trait]
impl Authentication<User, i64, MySqlPool> for User {
    async fn load_user(userid: i64, pool: Option<&MySqlPool>) -> Result<User, anyhow::Error> {
        let pool = pool.unwrap();
        User::get(userid, pool)
            .await
            .ok_or_else(|| anyhow::anyhow!("Cannot get user"))
    }

    fn is_authenticated(&self) -> bool {
        true
    }

    fn is_active(&self) -> bool {
        true
    }

    fn is_anonymous(&self) -> bool {
        false
    }
}
