use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use leptos::ServerFnError;
        use sqlx::MySqlPool;
        use sqlx::{FromRow,Type};
        use chrono::Local;
        use crate::server::to_server_fn_error;
    } else {
        use dummy_macros::*;
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Type, Copy)]
#[repr(i16)]
pub enum Mode {
    Original = 1,
    Processed = 2,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct OrderItem {
    pub id: u64,
    pub order_id: u64,
    pub mode: Mode,
    pub file_name: String,
    pub get_url: String,
    pub put_url: String,
    pub uploaded: bool,
    pub uploaded_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
}

#[cfg(feature = "ssr")]
impl OrderItem {
    pub async fn get_order_items_by_order_id(
        order_id: u64,
        mode: Mode,
        pool: &MySqlPool,
    ) -> Result<Vec<OrderItem>, ServerFnError> {
        sqlx::query_as!(OrderItem,"SELECT id,order_id,mode as `mode: _`, file_name, get_url,put_url,uploaded as `uploaded: _`,uploaded_at,created_at FROM `order_items` WHERE `order_id` = ? and `mode` = ?",order_id,mode)
            .fetch_all(pool)
            .await
            .map_err(to_server_fn_error)
    }
    pub async fn set_uploaded(&self, pool: &MySqlPool) -> Result<bool, ServerFnError> {
        sqlx::query!(
            "UPDATE `order_items` SET `uploaded` = true, `uploaded_at` = ? WHERE `id` = ?",
            self.id,
            Local::now()
        )
        .execute(pool)
        .await
        .map(|result| result.rows_affected() > 0)
        .map_err(to_server_fn_error)
    }
    pub async fn update_get_url(
        &self,
        get_url: String,
        pool: &MySqlPool,
    ) -> Result<bool, ServerFnError> {
        sqlx::query!(
            "UPDATE `order_items` SET `get_url` = ? WHERE `id` = ?",
            get_url,
            self.id
        )
        .execute(pool)
        .await
        .map_err(to_server_fn_error)
        .map(|result| result.rows_affected() > 0)
    }

    pub async fn get_by_id(id: u64, pool: &MySqlPool) -> Result<OrderItem, ServerFnError> {
        sqlx::query_as!(OrderItem, "SELECT id,order_id,mode as `mode: _`, file_name, get_url,put_url,uploaded as `uploaded: _`,uploaded_at,created_at FROM `order_items` WHERE `id` = ?", id)
            .fetch_one(pool)
            .await
            .map_err(to_server_fn_error)
    }

    pub async fn delete(id: u64, pool: &MySqlPool) -> Result<bool, ServerFnError> {
        sqlx::query!("DELETE FROM `order_items` WHERE `id` = ?", id)
            .execute(pool)
            .await
            .map(|result| result.rows_affected() > 0)
            .map_err(to_server_fn_error)
    }
}
