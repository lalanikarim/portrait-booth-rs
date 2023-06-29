use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pricing {
    pub base_price: u64,
    pub unit_price: u64,
}
