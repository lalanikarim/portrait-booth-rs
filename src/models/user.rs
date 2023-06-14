use serde::{Deserialize, Serialize};

impl Default for Role {
    fn default() -> Self {
        Role::Customer
    }
}

impl Default for UserStatus {
    fn default() -> Self {
        UserStatus::NotActivatedYet
    }
}

cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use sqlx::{FromRow, Type};
    } else {

        use dummy_macros::*;
    }
}
#[derive(Debug, Clone, Serialize, Deserialize, Default, FromRow)]
pub struct User {
    pub id: i64,
    pub email: Option<String>,
    pub phone: Option<String>,
    #[serde(skip)]
    pub password_hash: Option<String>,
    #[serde(skip)]
    pub otp_secret: Option<String>,
    pub role: Role,
    pub status: UserStatus,
    pub name: String,
}
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Type)]
#[repr(i32)]
pub enum Role {
    Manager = 5,
    Operator = 3,
    Customer = 1,
    Cashier = 2,
    Processor = 4,
    Anonymous = 0,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Type)]
#[repr(i32)]
pub enum UserStatus {
    Active = 1,
    NotActivatedYet = 0,
    Disabled = 2,
}

cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use sqlx::MySqlPool;
        impl User {
            pub fn anonymous() -> Self {
                Self {
                    id: -1,
                    email: None,
                    phone: None,
                    password_hash: None,
                    otp_secret: None,
                    role: Role::Anonymous,
                    status: UserStatus::Disabled,
                    name: "Guest".to_owned()
                }
            }
            pub async fn get_by_username(username: String, pool: &MySqlPool) -> Option<Self> {
                let user = sqlx::query_as::<_,User>("SELECT * FROM users WHERE ? in (email,phone)")
                    .bind(username)
                    .fetch_one(pool)
                    .await
                    .ok()?;
                Some(user)
            }
            pub async fn get(id: i64, pool: &MySqlPool) -> Option<Self> {
                let user = sqlx::query_as::<_,User>("SELECT * FROM users WHERE id = ?")
                    .bind(id)
                    .fetch_one(pool)
                    .await
                    .ok()?;
                Some(user)
            }

            pub async fn create(email: Option<String>,phone: Option<String>, password: String, name: String, role: Role, pool: &MySqlPool) -> Result<u64, sqlx::Error> {
                sqlx::query("INSERT INTO users (email,phone,password,name,role) values (?,?,?,?,?)")
                    .bind(email)
                    .bind(phone)
                    .bind(password)
                    .bind(name)
                    .bind(role)
                    .execute(pool)
                    .await
                    .map(|result| result.last_insert_id())
            }
        }
    }
}
