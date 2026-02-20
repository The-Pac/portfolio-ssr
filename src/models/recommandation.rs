use serde::{Deserialize, Serialize};
use crate::models::logo::Logo;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recommendation {
    pub logo: Logo,
    pub author: Option<String>,
    pub texte: Option<String>,
}

#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendationRow {
    pub id: i64,
    pub logo_id: i64,
    pub author: Option<String>,
    pub texte: Option<String>,
    pub logo_name: String,
    pub logo_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecommendationWithLogo {
    pub id: i64,
    pub logo_id: i64,
    pub logo: Logo,
    pub author: Option<String>,
    pub texte: Option<String>,
}