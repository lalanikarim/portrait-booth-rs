use serde::{Deserialize, Serialize};

pub mod create_order;
pub mod order_details;
pub mod order_list;
pub mod orders_view;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnitPrice(u64);
