use leptos::server;
use crate::models::database_error::{ProjectError};
use crate::models::recommandation::RecommendationWithLogo;

#[server]
pub async fn get_all_recommendations() -> Result<Vec<RecommendationWithLogo>, ProjectError> {
    #[cfg(feature = "ssr")]
    {
        use crate::libs::database::get_database;
        use crate::services;

        let database = get_database();
        return services::recommendation::get_all_recommendations(&database).await;
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}