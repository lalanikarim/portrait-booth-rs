use serde::{Deserialize, Serialize};

impl Default for Role {
    fn default() -> Self {
        Role::Customer
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
    pub username: String,
    #[serde(skip)]
    pub password_hash: String,
    pub role: Role,
    pub name: String,
}
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize, Deserialize, Type)]
pub enum Role {
    Manager,
    Operator,
    Customer,
    Processor,
}

cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use sqlx::MySqlPool;
        use axum_login::{secrecy::SecretVec, AuthUser};
        impl User {
            pub async fn get(id: i64, pool: &MySqlPool) -> Option<Self> {
                let user = sqlx::query_as::<_,User>("SELECT * FROM users WHERE id = ?")
                    .bind(id)
                    .fetch_one(pool)
                    .await
                    .ok()?;
                Some(user)
            }

            pub async fn create(username: String, password: String, name: String, role: Role, pool: &MySqlPool) -> Result<u64, sqlx::Error> {
                sqlx::query("INSERT INTO users (username, password,name,role) values (?,?,?,?)")
                    .bind(username)
                    .bind(password)
                    .bind(name)
                    .bind(role)
                    .execute(pool)
                    .await
                    .map(|result| result.last_insert_id())
            }
        }
        impl AuthUser<i64> for User {
            fn get_id(&self) -> i64 {
                self.id
            }

            fn get_password_hash(&self) ->SecretVec<u8> {
                SecretVec::new(self.password_hash.clone().into())
            }
        }
    }
}
