use chrono::{DateTime, Local, NaiveDate};
use serde::{Deserialize, Serialize};

cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use sqlx::{FromRow, Type};
        use super::order_item::OrderItem;
    } else {

        use dummy_macros::*;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Order {
    pub id: u64,
    pub customer_id: u64,
    pub cashier_id: Option<u64>,
    pub operator_id: Option<u64>,
    pub processor_id: Option<u64>,
    pub no_of_photos: u64,
    pub order_total: u64,
    pub mode_of_payment: PaymentMode,
    pub order_ref: Option<String>,
    pub payment_ref: Option<String>,
    pub status: OrderStatus,
    pub created_at: DateTime<Local>,
    pub payment_at: Option<NaiveDate>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum PaymentMode {
    NotSelected = 0,
    Cash = 1,
    Stripe = 2,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum OrderStatus {
    Created = 0,
    PaymentPending = 1,
    PaymentError = 2,
    Paid = 3,
    Uploaded = 4,
    InProcess = 5,
    Processed = 6,
}

impl Default for OrderStatus {
    fn default() -> Self {
        Self::Created
    }
}

impl Default for PaymentMode {
    fn default() -> Self {
        Self::Cash
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use sqlx::MySqlPool;
        use crate::models::user::User;
        use leptos::ServerFnError;
        use crate::to_server_fn_error;
    }
}
#[cfg(feature = "ssr")]
impl Order {
    pub async fn get_customer(&self, pool: &MySqlPool) -> Result<User, ServerFnError> {
        User::get_by_id(self.customer_id, pool).await
    }
    pub async fn get_cashier(&self, pool: &MySqlPool) -> Result<Option<User>, ServerFnError> {
        let Some(cashier_id) = self.cashier_id else {
                    return Ok(None);
                };
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
            .bind(cashier_id)
            .fetch_one(pool)
            .await
            .map_err(|e| crate::to_server_fn_error(e))
            .map(|u| Some(u))
    }
    pub async fn get_operator(&self, pool: &MySqlPool) -> Result<Option<User>, ServerFnError> {
        let Some(operator_id) = self.operator_id else {
                    return Ok(None);
                };
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
            .bind(operator_id)
            .fetch_one(pool)
            .await
            .map_err(|e| crate::to_server_fn_error(e))
            .map(|u| Some(u))
    }
    pub async fn get_processor(&self, pool: &MySqlPool) -> Result<Option<User>, ServerFnError> {
        let Some(processor_id) = self.processor_id else {
                    return Ok(None);
                };
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
            .bind(processor_id)
            .fetch_one(pool)
            .await
            .map_err(|e| crate::to_server_fn_error(e))
            .map(|u| Some(u))
    }

    pub async fn create(
        customer_id: u64,
        no_of_photos: u64,
        pool: &MySqlPool,
    ) -> Result<u64, ServerFnError> {
        let unit_price: u64 = dotenv!("PHOTO_UNIT_PRICE")
            .parse()
            .expect("Unit Price should be a valid number");
        let order_total = no_of_photos * unit_price;
        sqlx::query!("INSERT INTO `orders` (customer_id,no_of_photos,order_total,mode_of_payment,status,created_at) VALUES (?, ?, ?, ?, ?, ?)",
                    customer_id,
                    no_of_photos,
                    order_total,
                    PaymentMode::NotSelected,
                    OrderStatus::Created,
                    Local::now())
                    .execute(pool)
                    .await
                    .map(|result| result.last_insert_id())
                    .map_err(|e| to_server_fn_error(e))
    }
    pub async fn update(
        id: i64,
        no_of_photos: i64,
        pool: &MySqlPool,
    ) -> Result<u64, ServerFnError> {
        let unit_price: i64 = dotenv!("PHOTO_UNIT_PRICE")
            .parse()
            .expect("Unit Price should be a valid number");
        let order_total = no_of_photos * unit_price;
        sqlx::query!(
            "UPDATE `orders` SET no_of_photos = ?,order_total = ? WHERE id = ?",
            id,
            no_of_photos,
            order_total,
        )
        .execute(pool)
        .await
        .map(|result| result.last_insert_id())
        .map_err(|e| to_server_fn_error(e))
    }

    pub async fn collect_payment_cash(
        &self,
        cashier_id: i64,
        pool: &MySqlPool,
    ) -> Result<bool, ServerFnError> {
        sqlx::query!("UPDATE `orders` SET cashier_id = ?, mode_of_payment = ?, status = ? WHERE id = ? and status = ?",
                    cashier_id,
                    PaymentMode::Cash as i32,
                    OrderStatus::Paid as i32,
                    self.id,
                    OrderStatus::Created as i32)
                    .execute(pool)
                    .await.map(|result| result.rows_affected() > 0)
                    .map_err(|e| to_server_fn_error(e))
    }

    pub async fn start_payment_stripe(
        &self,
        order_ref: String,
        pool: &MySqlPool,
    ) -> Result<bool, ServerFnError> {
        sqlx::query!("UPDATE `orders` SET mode_of_payment = ?, order_ref = ?, status = ? WHERE id = ? and status = ?",
                    PaymentMode::Stripe as i32,
                    order_ref,
                    OrderStatus::PaymentPending as i32,
                    self.id,
                    OrderStatus::Created as i32)
                    .execute(pool)
                    .await.map(|result| result.rows_affected() > 0)
                    .map_err(|e| to_server_fn_error(e))
    }

    pub async fn mark_stripe_payment_complete(
        &self,
        payment_ref: String,
        pool: &MySqlPool,
    ) -> Result<bool, ServerFnError> {
        sqlx::query!("UPDATE `orders` SET payment_ref = ?, status = ? WHERE id = ? and mode_of_payment = ? and status = ?",
                    payment_ref,
                    OrderStatus::Paid as i32,
                    self.id,
                    PaymentMode::Stripe as i32,
                    OrderStatus::PaymentPending)
                    .execute(pool)
                    .await.map(|result| result.rows_affected() > 0)
                    .map_err(|e| to_server_fn_error(e))
    }
    pub async fn mark_stripe_payment_error(
        &self,
        error: String,
        pool: &MySqlPool,
    ) -> Result<bool, ServerFnError> {
        sqlx::query!("UPDATE `orders` SET payment_ref = ?, status = ? WHERE id = ? and mode_of_payment = ? and status = ?",
                    error,
                    OrderStatus::PaymentError as i32,
                    self.id,
                    PaymentMode::Stripe as i32,
                    OrderStatus::PaymentPending)
                    .execute(pool)
                    .await.map(|result| result.rows_affected() > 0)
                    .map_err(|e| to_server_fn_error(e))
    }
    pub async fn mark_order_uploaded(
        &self,
        operator_id: i64,
        pool: &MySqlPool,
    ) -> Result<bool, ServerFnError> {
        sqlx::query!(
            "UPDATE `orders` SET operator_id = ?, status = ? WHERE id = ? and status = ?",
            operator_id,
            OrderStatus::Uploaded as i32,
            self.id,
            OrderStatus::Paid
        )
        .execute(pool)
        .await
        .map(|result| result.rows_affected() > 0)
        .map_err(|e| to_server_fn_error(e))
    }
    pub async fn mark_order_in_progress(
        &self,
        processor_id: i64,
        pool: &MySqlPool,
    ) -> Result<bool, ServerFnError> {
        sqlx::query!(
            "UPDATE `orders` SET processor_id = ?, status = ? WHERE id = ? and status = ?",
            processor_id,
            OrderStatus::InProcess as i32,
            self.id,
            OrderStatus::Uploaded
        )
        .execute(pool)
        .await
        .map(|result| result.rows_affected() > 0)
        .map_err(|e| to_server_fn_error(e))
    }
    pub async fn mark_order_processed(
        &self,
        processor_id: i64,
        pool: &MySqlPool,
    ) -> Result<bool, ServerFnError> {
        sqlx::query!(
            "UPDATE `orders` SET status = ? WHERE id = ? and processor_id = ? and status = ?",
            OrderStatus::Processed as i32,
            self.id,
            processor_id,
            OrderStatus::InProcess
        )
        .execute(pool)
        .await
        .map(|result| result.rows_affected() > 0)
        .map_err(|e| to_server_fn_error(e))
    }

    pub async fn get_order_items(&self, pool: &MySqlPool) -> Result<Vec<OrderItem>, ServerFnError> {
        sqlx::query_as::<_, OrderItem>("SELECT * FROM `order_items` where order_id = ?")
            .bind(self.id)
            .fetch_all(pool)
            .await
            .map_err(|e| crate::to_server_fn_error(e))
    }

    pub async fn add_order_item(
        &self,
        original_url: String,
        pool: &MySqlPool,
    ) -> Result<bool, ServerFnError> {
        sqlx::query!(
            "INSERT into `order_items` (order_id,original_url,created_at) values (?, ?, ?)",
            self.id,
            original_url,
            Local::now()
        )
        .execute(pool)
        .await
        .map(|result| result.rows_affected() > 0)
        .map_err(|e| crate::to_server_fn_error(e))
    }
}
