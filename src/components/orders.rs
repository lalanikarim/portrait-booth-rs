use serde::{Deserialize, Serialize};

pub mod cashier_actions;
pub mod confirmation;
pub mod create_order;
pub mod customer_actions;
pub mod manager_actions;
pub mod operator_actions;
pub mod order_details;
pub mod order_list;
pub mod order_search;
pub mod orders_view;
pub mod search_results;
pub mod search_view;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnitPrice(u64, u64);
