use std::fmt::Display;

use serde::{Deserialize, Serialize};

cfg_if::cfg_if! {
    if #[cfg(feature = "ssr")] {
        use sqlx::MySqlPool;
        use leptos::ServerFnError;
        use sqlx::FromRow;
        use crate::to_server_fn_error;
    } else {
        use dummy_macros::*;
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Setting {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum SettingKey {
    AllowOrderCreation,
}

impl Display for SettingKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Setting {
    pub fn is_true(&self) -> bool {
        self.value == String::from("1")
    }
}

#[cfg(feature = "ssr")]
impl Setting {
    pub async fn toggle_allow_order_creation(
        toggle: bool,
        pool: &MySqlPool,
    ) -> Result<bool, ServerFnError> {
        sqlx::query!(
            "REPLACE INTO `settings` (`name`,`value`) VALUES (?,?)",
            SettingKey::AllowOrderCreation.to_string(),
            if toggle { "1" } else { "0" }
        )
        .execute(pool)
        .await
        .map_err(to_server_fn_error)
        .map(|record| record.rows_affected() > 0)
    }

    pub async fn get_allow_order_creation(pool: &MySqlPool) -> Result<Setting, ServerFnError> {
        sqlx::query_as!(
            Setting,
            "SELECT name as `name:_`, value FROM `settings` WHERE `name` = ?",
            SettingKey::AllowOrderCreation.to_string(),
        )
        .fetch_optional(pool)
        .await
        .map_err(to_server_fn_error)
        .map(|setting| match setting {
            Some(setting) => setting,
            None => Setting {
                name: SettingKey::AllowOrderCreation.to_string(),
                value: String::from("1"),
            },
        })
    }
}
