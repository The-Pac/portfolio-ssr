use serde::{Deserialize, Serialize};


#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Logo{
    pub path: String,
    pub name: String,
}

#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogoRow {
    pub id: i64,
    pub path: String,
    pub name: String,
}