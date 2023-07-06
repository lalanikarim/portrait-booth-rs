use serde::{Deserialize, Serialize};

use super::order::{OrderStatus, PaymentMode};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow, PartialEq)]
pub struct UserOrder {
    pub id: u64,
    pub name: String,
    pub email: String,
    pub phone: Option<String>,
    pub customer_id: u64,
    pub no_of_photos: u64,
    pub order_total: u64,
    pub mode_of_payment: PaymentMode,
    pub status: OrderStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct OrderSearchForm {
    pub order_no: Option<u64>,
    pub name: Option<String>,
    pub email: Option<String>,
    pub phone: Option<String>,
}
cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use sqlx::{FromRow, MySql, MySqlPool};
        use leptos::ServerFnError;
        use crate::server::to_server_fn_error;
    } else {
        use dummy_macros::*;
    }
}

#[cfg(feature = "ssr")]
impl UserOrder {
    pub async fn get_by_order_id(
        order_id: u64,
        pool: &MySqlPool,
    ) -> Result<UserOrder, ServerFnError> {
        sqlx::query_as!(
            UserOrder,
            "select u.name, u.email, u.phone, o.id,o.no_of_photos,o.status as `status:_`,o.customer_id,o.order_total,o.mode_of_payment as `mode_of_payment:_`
            from orders o inner join users u on o.customer_id = u.id where o.id = ?",
            order_id
        )
        .fetch_one(pool)
        .await.map_err(to_server_fn_error)
    }
    pub async fn search_orders(
        OrderSearchForm {
            order_no,
            name,
            email,
            phone,
        }: OrderSearchForm,
        pool: &MySqlPool,
    ) -> Result<Vec<Self>, ServerFnError> {
        if order_no.is_none() && name.is_none() && email.is_none() && phone.is_none() {
            return Err(ServerFnError::MissingArg(
                "Search query should include at least one of order_no, name, email, or phone"
                    .to_string(),
            ));
        }
        let mut search_query = String::new();
        let mut is_first_search_term = true;
        search_query.push_str(
            r#"
            select u.name, u.email, u.phone, o.*
            from orders o inner join users u on o.customer_id = u.id
            where "#,
        );
        if order_no.is_some() {
            search_query.push_str(" o.id = ? ");
            is_first_search_term = false;
        }
        if name.is_some() {
            if !is_first_search_term {
                search_query.push_str(" and ");
            }
            search_query.push_str(" lower(u.name) like lower(?) ");
            is_first_search_term = false;
        }
        if email.is_some() {
            if !is_first_search_term {
                search_query.push_str(" and ");
            }
            search_query.push_str(" lower(u.email) = lower(?) ");
            is_first_search_term = false;
        }
        if phone.is_some() {
            if !is_first_search_term {
                search_query.push_str(" and ");
            }
            search_query.push_str(" lower(u.phone) = lower(?) ");
        }
        search_query.push_str(" order by case when o.status = 3 then 1 when o.status = 1 then 2 when o.status = 6 then 3 else 10+o.status end, o.id desc");
        let mut search_query = sqlx::query_as::<MySql, UserOrder>(search_query.as_str());
        if let Some(order_no) = order_no {
            search_query = search_query.bind(order_no);
        }
        if let Some(name) = name {
            search_query = search_query.bind(format!("%{}%", name));
        }
        if let Some(email) = email {
            search_query = search_query.bind(email);
        }
        if let Some(phone) = phone {
            search_query = search_query.bind(phone);
        }
        search_query
            .fetch_all(pool)
            .await
            .map_err(to_server_fn_error)
    }
}
