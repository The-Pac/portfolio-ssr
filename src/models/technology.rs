use serde::{Deserialize, Serialize};

#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechnologyCategory {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
}

#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Technology {
    pub id: i64,
    pub name: String,
    pub technology_category_id: i64,
    pub logo_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechnologyWithLogo {
    pub id: i64,
    pub name: String,
    pub category_title: String,
    pub logo_path: String,
    pub logo_name: String,
}

#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TechnologyRow {
    pub technology_id: i64,
    pub technology_name: String,
    pub category_title: String,
    pub logo_path: String,
    pub logo_name: String,
}
