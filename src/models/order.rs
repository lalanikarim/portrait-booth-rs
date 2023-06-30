use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use sqlx::{FromRow, Type};
        use crate::models::order_item::OrderItem;
        use crate::server::to_server_fn_error;
        use super::user_order::UserOrder;
        use super::pricing::Pricing;
        use super::order_item::Mode;
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
    pub created_at: NaiveDateTime,
    pub payment_at: Option<NaiveDateTime>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq, PartialOrd)]
#[repr(i8)]
pub enum PaymentMode {
    NotSelected = 0,
    Cash = 1,
    Stripe = 2,
    Override = 3,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type, PartialEq, PartialOrd)]
#[repr(i8)]
pub enum OrderStatus {
    Created = 0,
    PaymentPending = 1,
    PaymentError = 2,
    Paid = 3,
    ReadyToUpload = 4,
    Uploading = 5,
    Uploaded = 6,
    ReadyToProcess = 7,
    InProcess = 8,
    Processed = 9,
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
        use chrono::Local;
    }
}
#[cfg(feature = "ssr")]
impl Order {
    pub async fn get_orders_for_customer(
        customer_id: u64,
        pool: &MySqlPool,
    ) -> Result<Vec<UserOrder>, ServerFnError> {
        let result = sqlx::query_as!(
            UserOrder,
            "SELECT o.id, o.customer_id, o.no_of_photos, o.order_total, o.mode_of_payment as `mode_of_payment:_`, o.status as `status:_`, u.name, u.email, u.phone FROM `orders` o inner join `users` u where o.customer_id = u.id and u.id = ?",
            customer_id
        )
        .fetch_all(pool)
        .await
        .map_err(|e| to_server_fn_error(e));
        result
    }
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
            .map_err(|e| to_server_fn_error(e))
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
            .map_err(|e| to_server_fn_error(e))
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
            .map_err(|e| to_server_fn_error(e))
            .map(|u| Some(u))
    }

    pub async fn create(
        customer_id: u64,
        no_of_photos: u64,
        pool: &MySqlPool,
    ) -> Result<Option<Order>, ServerFnError> {
        let Pricing {
            base_price,
            unit_price,
        } = Self::get_unit_price().expect("PHOTO_UNIT_PRICE env variable should be present");
        let order_total = no_of_photos * unit_price + base_price;
        let order_create = sqlx::query!("INSERT INTO `orders` (customer_id,no_of_photos,order_total,mode_of_payment,status,created_at) VALUES (?, ?, ?, ?, ?, ?)",
                    customer_id,
                    no_of_photos,
                    order_total,
                    PaymentMode::NotSelected,
                    OrderStatus::Created,
                    Local::now())
                    .execute(pool)
                    .await
                    .map(|result| result.last_insert_id());
        match order_create {
            Err(e) => Err(to_server_fn_error(e)),
            Ok(order_id) => Order::get_by_id(order_id, pool)
                .await
                .map_err(|e| to_server_fn_error(e)),
        }
    }

    pub async fn get_by_id(id: u64, pool: &MySqlPool) -> Result<Option<Order>, ServerFnError> {
        sqlx::query_as!(Order, "SELECT id, customer_id, cashier_id, operator_id, processor_id, no_of_photos, order_total, mode_of_payment as `mode_of_payment:_`, order_ref, payment_ref, status as `status:_`, created_at, payment_at FROM `orders` where id = ?", id)
            .fetch_optional(pool)
            .await
            .map_err(|e| to_server_fn_error(e))
    }

    pub async fn update_status(
        id: u64,
        from: OrderStatus,
        to: OrderStatus,
        pool: &MySqlPool,
    ) -> Result<bool, ServerFnError> {
        sqlx::query!(
            "UPDATE `orders` SET `status` = ? WHERE `status` = ? AND `id` = ?",
            to,
            from,
            id
        )
        .execute(pool)
        .await
        .map(|result| result.rows_affected() > 0)
        .map_err(|e| to_server_fn_error(e))
    }
    pub async fn delete(
        id: u64,
        customer_id: u64,
        pool: &MySqlPool,
    ) -> Result<bool, ServerFnError> {
        sqlx::query!(
            "DELETE FROM `orders` WHERE id = ? and customer_id = ?",
            id,
            customer_id
        )
        .execute(pool)
        .await
        .map(|result| result.rows_affected() > 0)
        .map_err(|e| to_server_fn_error(e))
    }
    pub async fn update(
        id: u64,
        no_of_photos: u64,
        pool: &MySqlPool,
    ) -> Result<u64, ServerFnError> {
        let Pricing {
            base_price,
            unit_price,
        } = Self::get_unit_price().expect("PHOTO_UNIT_PRICE env variable should be present");
        let order_total = no_of_photos * unit_price + base_price;
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
    pub async fn start_payment_cash(
        id: u64,
        customer_id: u64,
        pool: &MySqlPool,
    ) -> Result<bool, ServerFnError> {
        sqlx::query!("UPDATE `orders` SET mode_of_payment = ?, status = ? WHERE id = ? and customer_id = ? and status = ?",
                    PaymentMode::Cash as i32,
                    OrderStatus::PaymentPending as i32,
                    id,
                    customer_id,
                    OrderStatus::Created as i32)
                    .execute(pool)
                    .await.map(|result| result.rows_affected() > 0)
                    .map_err(|e| to_server_fn_error(e))
    }
    pub async fn collect_payment_cash(
        id: u64,
        cashier_id: u64,
        pool: &MySqlPool,
    ) -> Result<bool, ServerFnError> {
        sqlx::query!("UPDATE `orders` SET cashier_id = ?,  status = ? WHERE id = ? and status = ? and mode_of_payment = ?",
                    cashier_id,
                    OrderStatus::Paid as i32,
                    id,
                    OrderStatus::PaymentPending as i32,
                    PaymentMode::Cash as i32)
                    .execute(pool)
                    .await.map(|result| result.rows_affected() > 0)
                    .map_err(|e| to_server_fn_error(e))
    }

    pub async fn start_payment_stripe(
        id: u64,
        customer_id: u64,
        order_ref: String,
        pool: &MySqlPool,
    ) -> Result<bool, ServerFnError> {
        use base64::{engine::general_purpose, Engine as _};
        let order_ref: String = general_purpose::URL_SAFE_NO_PAD.encode(order_ref);
        sqlx::query!("UPDATE `orders` SET mode_of_payment = ?, order_ref = ?, status = ? WHERE id = ? and status = ? and customer_id = ?",
                    PaymentMode::Stripe as i32,
                    order_ref,
                    OrderStatus::PaymentPending as i32,
                    id,
                    OrderStatus::Created as i32,
                    customer_id)
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

    pub async fn get_order_items(
        &self,
        mode: Mode,
        pool: &MySqlPool,
    ) -> Result<Vec<OrderItem>, ServerFnError> {
        sqlx::query_as!(OrderItem,"SELECT id,order_id,mode as `mode: _`, file_name, get_url,put_url,uploaded as `uploaded: _`,uploaded_at,created_at FROM `order_items` WHERE `order_id` = ? and `mode` = ?",self.id,mode)
            .fetch_all(pool)
            .await
            .map_err(|e| to_server_fn_error(e))
    }
    pub async fn remaining_order_items(
        &self,
        mode: Mode,
        pool: &MySqlPool,
    ) -> Result<u64, ServerFnError> {
        sqlx::query!(
            "SELECT COUNT(1) AS count FROM `order_items` WHERE `order_id` = ? AND `mode` = ?",
            self.id,
            mode
        )
        .fetch_one(pool)
        .await
        .map_err(|e| to_server_fn_error(e))
        .map(|result| self.no_of_photos - result.count as u64)
    }

    pub async fn add_order_item(
        &self,
        file_name: String,
        mode: Mode,
        get_url: String,
        put_url: String,
        pool: &MySqlPool,
    ) -> Result<OrderItem, ServerFnError> {
        match sqlx::query!("SELECT COUNT(1) as count FROM `order_items` WHERE order_id = ? AND mode = ?",self.id,mode).fetch_one(pool).await {
            Err(e) => Err(to_server_fn_error(e)),
            Ok(result) => match self.no_of_photos > result.count as u64 {
                false => Err(ServerFnError::ServerError("No more uploads allowed".to_string())),
                true => {
                    match sqlx::query!(
                        "INSERT into `order_items` (order_id,file_name,mode,get_url,put_url,created_at) values (?, ?, ?, ?, ?, ? )",
                        self.id,
                        file_name,
                        mode,
                        get_url,
                        put_url,
                        Local::now()
                    )
                    .execute(pool)
                    .await {
                            Err(e) => Err(to_server_fn_error(e)),
                            Ok(result) => OrderItem::get_by_id(result.last_insert_id(),&pool).await
                        }
                }
            }
        }
    }

    pub async fn update_order_confirmation(
        order_ref: String,
        payment_ref: String,
        pool: &MySqlPool,
    ) -> Result<bool, ServerFnError> {
        sqlx::query!(
            "UPDATE `orders` set payment_ref = ?, status = ? where mode_of_payment = ? and order_ref = ? and status = ?",
            payment_ref,
            OrderStatus::Paid,
            PaymentMode::Stripe,
            order_ref,
            OrderStatus::PaymentPending
        )
        .execute(pool)
        .await
        .map(|result| result.rows_affected() > 0)
        .map_err(|e| to_server_fn_error(e))
    }

    pub async fn get_by_order_confirmation(
        order_ref: String,
        pool: &MySqlPool,
    ) -> Result<Option<Order>, ServerFnError> {
        sqlx::query_as!(Order, "SELECT id, customer_id, cashier_id, operator_id, processor_id, no_of_photos, order_total, mode_of_payment as `mode_of_payment:_`, order_ref, payment_ref, status as `status:_`, created_at, payment_at FROM `orders` where order_ref = ?", order_ref)
            .fetch_optional(pool)
            .await
            .map_err(|e| to_server_fn_error(e))
    }

    pub fn get_unit_price() -> Result<Pricing, ServerFnError> {
        let unit_price = dotenvy::var("PHOTO_UNIT_PRICE")
            .unwrap_or("5".into())
            .parse()
            .map_err(|e| crate::to_server_fn_error(e));
        let base_price = dotenvy::var("PHOTO_ZERO_PRICE")
            .unwrap_or("5".into())
            .parse()
            .map_err(|e| crate::to_server_fn_error(e));
        match (base_price, unit_price) {
            (Ok(base_price), Ok(unit_price)) => Ok(Pricing {
                base_price,
                unit_price,
            }),
            _ => Err(ServerFnError::ServerError(
                "Unable to find or parse price".to_string(),
            )),
        }
    }
}
