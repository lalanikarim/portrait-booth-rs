use crate::models::user::*;
use axum::async_trait;
use axum_session::SessionMySqlPool;
use axum_session_auth::Authentication;
//use axum_session_auth::Authentication;
use leptos::{use_context, Scope, ServerFnError};
use sqlx::MySqlPool;

pub type Session = axum_session::Session<SessionMySqlPool>;
pub type AuthSessionLayer =
    axum_session_auth::AuthSessionLayer<User, u64, SessionMySqlPool, MySqlPool>;
pub type AuthSession = axum_session_auth::AuthSession<User, u64, SessionMySqlPool, MySqlPool>;
pub fn session(cx: Scope) -> Result<Session, ServerFnError> {
    use_context::<Session>(cx)
        .ok_or("session missing")
        .map_err(|e| ServerFnError::ServerError(e.to_string()))
}
pub fn auth(cx: Scope) -> Result<AuthSession, ServerFnError> {
    use_context::<AuthSession>(cx)
        .ok_or("auth session missing")
        .map_err(|e| ServerFnError::ServerError(e.to_string()))
}

#[async_trait]
impl Authentication<User, u64, MySqlPool> for User {
    async fn load_user(userid: u64, pool: Option<&MySqlPool>) -> Result<User, anyhow::Error> {
        if userid == 0 {
            Ok(User::anonymous())
        } else {
            let pool = pool.unwrap();
            User::get_by_id(userid, pool)
                .await
                .map_err(|e| anyhow::Error::msg(e.to_string()))
        }
    }

    fn is_authenticated(&self) -> bool {
        !&self.is_anonymous()
    }

    fn is_active(&self) -> bool {
        !&self.is_anonymous()
    }

    fn is_anonymous(&self) -> bool {
        self.role == Role::Anonymous
    }
}
