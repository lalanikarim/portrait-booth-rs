use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use leptos::ServerFnError;
        use sqlx::MySqlPool;
        use sqlx::FromRow;
        use chrono::Local;
        use crate::server::to_server_fn_error;
    } else {
        use dummy_macros::*;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct OrderItem {
    pub id: u64,
    pub order_id: u64,
    pub original_url: String,
    pub thumbnail_url: Option<String>,
    pub processed_url: Option<String>,
    pub created_at: NaiveDateTime,
    pub processed_at: Option<NaiveDateTime>,
}

#[cfg(feature = "ssr")]
impl OrderItem {
    pub async fn add_thumbnail(
        &self,
        thumbnail_url: String,
        pool: &MySqlPool,
    ) -> Result<bool, ServerFnError> {
        sqlx::query!(
            "UPDATE `order_items` set thumbnail_url = ? where id = ?",
            thumbnail_url,
            self.id
        )
        .execute(pool)
        .await
        .map(|result| result.rows_affected() > 0)
        .map_err(|e| to_server_fn_error(e))
    }
    pub async fn add_processed(
        &self,
        processed_url: String,
        pool: &MySqlPool,
    ) -> Result<bool, ServerFnError> {
        sqlx::query!(
            "UPDATE `order_items` set processed_url = ?, processed_at = ? where id = ?",
            processed_url,
            Local::now(),
            self.id
        )
        .execute(pool)
        .await
        .map(|result| result.rows_affected() > 0)
        .map_err(|e| to_server_fn_error(e))
    }
}
