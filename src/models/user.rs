use serde::{Deserialize, Serialize};

impl Default for Role {
    fn default() -> Self {
        Role::Customer
    }
}

impl Default for UserStatus {
    fn default() -> Self {
        UserStatus::NotActivatedYet
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use sqlx::{FromRow, Type};
        use leptos::ServerFnError;
    } else {
        use dummy_macros::*;
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Default, FromRow)]
pub struct User {
    pub id: u64,
    pub email: Option<String>,
    pub phone: Option<String>,
    #[serde(skip)]
    pub password_hash: Option<String>,
    #[serde(skip)]
    pub otp_secret: Option<String>,
    pub role: Role,
    pub status: UserStatus,
    pub name: String,
}
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Type)]
#[repr(i16)]
pub enum Role {
    Anonymous = 0,
    Customer = 1,
    Cashier = 2,
    Operator = 3,
    Processor = 4,
    Manager = 5,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Type)]
#[repr(i16)]
pub enum UserStatus {
    Active = 1,
    NotActivatedYet = 0,
    Disabled = 2,
}

cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use sqlx::MySqlPool;
    }
}
#[cfg(feature = "ssr")]
impl User {
    pub fn anonymous() -> Self {
        Self {
            id: 0,
            email: None,
            phone: None,
            password_hash: None,
            otp_secret: None,
            role: Role::Anonymous,
            status: UserStatus::Disabled,
            name: "Guest".to_owned(),
        }
    }

    pub async fn get_by_username(
        username: String,
        pool: &MySqlPool,
    ) -> Result<Self, ServerFnError> {
        sqlx::query_as!(User,"SELECT id,name,email,phone,password_hash,otp_secret,role as `role:_`,status as `status: _` from `users` WHERE email = ?", username)
            .fetch_one(pool)
            .await
            .map_err(|e| crate::to_server_fn_error(e))
    }

    pub async fn get_by_id(id: u64, pool: &MySqlPool) -> Result<Self, ServerFnError> {
        sqlx::query_as!(User,"SELECT id,name,email,phone,password_hash,otp_secret,role as `role:_`,status as `status: _` from `users` WHERE id = ?", id)
            .fetch_one(pool)
            .await
            .map_err(|e| crate::to_server_fn_error(e))
    }

    pub async fn create(
        email: Option<String>,
        phone: Option<String>,
        password: String,
        name: String,
        role: Role,
        pool: &MySqlPool,
    ) -> Result<u64, ServerFnError> {
        sqlx::query!(
            "INSERT INTO users (email,phone,password_hash,name,role) values (?,?,?,?,?)",
            email,
            phone,
            password,
            name,
            role
        )
        .execute(pool)
        .await
        .map(|result| result.last_insert_id())
        .map_err(|e| crate::to_server_fn_error(e))
    }
}
