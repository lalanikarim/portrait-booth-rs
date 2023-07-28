use super::order::OrderStatus;
use serde::{Deserialize, Serialize};

cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use sqlx::FromRow;
        use leptos::ServerFnError;
        use sqlx::MySqlPool;
        use crate::to_server_fn_error;
    } else {
        use dummy_macros::*;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct OrderCountByStatus {
    pub status: OrderStatus,
    pub count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct PaymentCollection {
    pub name: String,
    pub email: Option<String>,
    pub count: i64,
    pub total: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct OrderCountByProcessor {
    pub name: String,
    pub email: String,
    pub order_count: i64,
    pub photos_count: Option<i64>,
}

pub struct Report {}

#[cfg(feature = "ssr")]
impl Report {
    pub async fn get_order_count_by_status(
        pool: &MySqlPool,
    ) -> Result<Vec<OrderCountByStatus>, ServerFnError> {
        sqlx::query_as!(OrderCountByStatus,"SELECT `status` as `status:_`, count(1) as count FROM `orders` GROUP BY `status` ORDER BY `status` DESC").fetch_all(pool).await.map_err(to_server_fn_error)
    }

    pub async fn get_collection_by_staff(
        pool: &MySqlPool,
    ) -> Result<Vec<PaymentCollection>, ServerFnError> {
        sqlx::query_as!(
            PaymentCollection,
            r#"select ifnull(u.name,'Stripe') as name, u.email, 
        count(1) as `count`, cast(sum(o.order_total) as signed) as `total`
        from orders o 
        left join users u on u.id = o.cashier_id 
        where o.status >= 3
        GROUP by u.name, u.email  "#
        )
        .fetch_all(pool)
        .await
        .map_err(to_server_fn_error)
    }

    pub async fn get_order_count_by_processor(
        pool: &MySqlPool,
    ) -> Result<Vec<OrderCountByProcessor>, ServerFnError> {
        sqlx::query_as!(
            OrderCountByProcessor,
            r#"select u.name, u.email,  
            cast(count(1) as signed) as order_count,
            cast(sum(o.no_of_photos) as signed) as photos_count 
            from users u 
            inner join orders o on o.processor_id = u.id 
            where o.status = ?
            group by u.name, u.email, o.status
            order by u.name, o.status"#,
            OrderStatus::ReadyForDelivery
        )
        .fetch_all(pool)
        .await
        .map_err(to_server_fn_error)
    }
}
