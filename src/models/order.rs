use chrono::NaiveDateTime;
use leptos::ServerFnError;
use serde::{Deserialize, Serialize};

use super::user::User;

cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use sqlx::{FromRow, Type};
    } else {

        use dummy_macros::*;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Order {
    pub id: i64,
    pub customer_id: i64,
    pub cashier_id: Option<i64>,
    pub operator_id: Option<i64>,
    pub processor_id: Option<i64>,
    pub status: OrderStatus,
    pub created_at: NaiveDateTime,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq, PartialOrd)]
#[repr(i32)]
pub enum OrderStatus {
    Created = 0,
    Paid = 1,
    Uploaded = 2,
    InProcess = 3,
    Processed = 4,
}

impl Default for OrderStatus {
    fn default() -> Self {
        Self::Created
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use sqlx::MySqlPool;

        impl Order {
            pub async fn get_customer(&self, pool: &MySqlPool) -> Result<User,ServerFnError> {
                sqlx::query_as::<_,User>("SELECT * FROM users WHERE id = ?").bind(self.customer_id).fetch_one(pool).await.map_err(|e| crate::to_server_fn_error(e))
            }
            pub async fn get_cashier(&self, pool: &MySqlPool) -> Result<Option<User>,ServerFnError> {
                let Some(cashier_id) = self.cashier_id else {
                    return Ok(None);
                };
                sqlx::query_as::<_,User>("SELECT * FROM users WHERE cashier_id = ?")
                    .bind(cashier_id)
                    .fetch_one(pool)
                    .await
                    .map_err(|e| crate::to_server_fn_error(e))
                    .map(|u| Some(u))
            }
            pub async fn get_operator(&self, pool: &MySqlPool) -> Result<Option<User>,ServerFnError> {
                let Some(operator_id) = self.operator_id else {
                    return Ok(None);
                };
                sqlx::query_as::<_,User>("SELECT * FROM users WHERE cashier_id = ?")
                    .bind(operator_id)
                    .fetch_one(pool)
                    .await
                    .map_err(|e| crate::to_server_fn_error(e))
                    .map(|u| Some(u))
            }
            pub async fn get_processor(&self, pool: &MySqlPool) -> Result<Option<User>,ServerFnError> {
                let Some(processor_id) = self.processor_id else {
                    return Ok(None);
                };
                sqlx::query_as::<_,User>("SELECT * FROM users WHERE processor_id = ?")
                    .bind(processor_id)
                    .fetch_one(pool)
                    .await
                    .map_err(|e| crate::to_server_fn_error(e))
                    .map(|u| Some(u))            }
        }
    }
}
