use serde::{Deserialize, Serialize};

cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use sqlx::{FromRow, Type};
        use leptos::ServerFnError;
        use crate::server::to_server_fn_error;
        use super::user_order::UserOrder;
    } else {
        use dummy_macros::*;
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Default, FromRow)]
pub struct User {
    pub id: u64,
    pub email: String,
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
#[derive(Default)]
pub enum Role {
    Anonymous = 0,
    #[default]
    Customer = 1,
    Cashier = 2,
    Operator = 3,
    Processor = 4,
    Manager = 5,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Type)]
#[repr(i16)]
#[derive(Default)]
pub enum UserStatus {
    Active = 1,
    #[default]
    NotActivatedYet = 0,
    Disabled = 2,
}

cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use sqlx::MySqlPool;
        use super::order::Order;
    }
}
#[cfg(feature = "ssr")]
impl User {
    pub fn anonymous() -> Self {
        Self {
            id: 0,
            role: Role::Anonymous,
            status: UserStatus::Disabled,
            name: "Guest".to_owned(),
            ..Default::default()
        }
    }

    pub async fn get_by_username(
        username: String,
        pool: &MySqlPool,
    ) -> Result<Self, ServerFnError> {
        sqlx::query_as!(User,"SELECT id,name,email,phone,password_hash,otp_secret,role as `role:_`,status as `status: _` from `users` WHERE email = ?", username)
            .fetch_one(pool)
            .await
            .map_err(to_server_fn_error)
    }

    pub async fn get_all_staff(pool: &MySqlPool) -> Result<Vec<Self>, ServerFnError> {
        sqlx::query_as!(User,"SELECT id,name,email,phone,password_hash,otp_secret,role as `role:_`,status as `status: _` from `users` WHERE role not in (?,?)", Role::Anonymous, Role::Customer).fetch_all(pool).await.map_err(to_server_fn_error)
    }

    pub async fn get_by_id(id: u64, pool: &MySqlPool) -> Result<Self, ServerFnError> {
        sqlx::query_as!(User,"SELECT id,name,email,phone,password_hash,otp_secret,role as `role:_`,status as `status: _` from `users` WHERE id = ?", id)
            .fetch_one(pool)
            .await
            .map_err(to_server_fn_error)
    }

    pub async fn change_role(id: u64, role: Role, pool: &MySqlPool) -> Result<bool, ServerFnError> {
        sqlx::query!("UPDATE `users` SET `role` = ? WHERE `id` = ?", role, id)
            .execute(pool)
            .await
            .map_err(to_server_fn_error)
            .map(|result| result.rows_affected() > 0)
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
        .map_err(to_server_fn_error)
    }

    pub async fn orders(&self, pool: &MySqlPool) -> Result<Vec<UserOrder>, ServerFnError> {
        Order::get_orders_for_customer(self.id, pool).await
    }
}
