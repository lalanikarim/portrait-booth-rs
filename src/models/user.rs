use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize)]
pub enum Role {
    Manager,
    Operator,
    Customer,
    Processor,
}

impl Default for Role {
    fn default() -> Self {
        Role::Customer
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct User {
    id: usize,
    username: String,
    #[serde(skip)]
    password_hash: String,
    role: Role,
    name: String,
}
