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
    pub file_name: String,
    pub original_get_url: String,
    pub original_put_url: String,
    pub original_uploaded: bool,
    pub original_uploaded_at: Option<NaiveDateTime>,
    pub thumbnail_get_url: String,
    pub thumbnail_put_url: String,
    pub thumbnail_uploaded: bool,
    pub thumbnail_uploaded_at: Option<NaiveDateTime>,
    pub processed_get_url: String,
    pub processed_put_url: String,
    pub processed_uploaded: bool,
    pub processed_uploaded_at: Option<NaiveDateTime>,
    pub created_at: NaiveDateTime,
    pub processed_at: Option<NaiveDateTime>,
}

#[cfg(feature = "ssr")]
impl OrderItem {
    pub async fn set_original_uploaded(&self, pool: &MySqlPool) -> Result<bool, ServerFnError> {
        sqlx::query!("UPDATE `order_items` SET original_uploaded = true, original_uploaded_at = ? WHERE id = ?",
            Local::now(), 
            self.id)
            .execute(pool)
            .await
            .map_err(|e| to_server_fn_error(e))
            .map(|result| result.rows_affected() > 0)
    }
    pub async fn set_thumbnail_uploaded(&self, pool: &MySqlPool) -> Result<bool, ServerFnError> {
        sqlx::query!("UPDATE `order_items` SET thumbnail_uploaded = true, thumbnail_uploaded_at = ? WHERE id = ?",
            Local::now(), 
            self.id)
            .execute(pool)
            .await
            .map_err(|e| to_server_fn_error(e))
            .map(|result| result.rows_affected() > 0)
    }
    pub async fn set_processed_uploaded(&self, pool: &MySqlPool) -> Result<bool, ServerFnError> {
        sqlx::query!("UPDATE `order_items` SET processed_uploaded = true, processed_uploaded_at = ? WHERE id = ?",
            Local::now(), 
            self.id)
            .execute(pool)
            .await
            .map_err(|e| to_server_fn_error(e))
            .map(|result| result.rows_affected() > 0)
    }
}
