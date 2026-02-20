use leptos::lazy;
use crate::models::database_error::{ModelError, ProjectError};
use sqlx::SqlitePool;
use crate::models::logo::Logo;
use crate::models::recommandation::{RecommendationWithLogo, RecommendationRow};

#[lazy]
pub async fn get_all_recommendations(
    connection_database: &SqlitePool,
) -> Result<Vec<RecommendationWithLogo>, ProjectError> {
    let recommendation_rows = sqlx::query_as::<_, RecommendationRow>(
        r#"
        SELECT
            r.id,
            r.logo_id,
            r.author,
            r.texte,
            lo.name AS logo_name,
            lo.path AS logo_path
        FROM recommendation r
        INNER JOIN logo lo ON r.logo_id = lo.id
        ORDER BY r.id DESC
        "#,
    )
        .fetch_all(connection_database)
        .await
        .map_err(|e| ModelError::DatabaseError(e.into()))?;

    let recommendations = recommendation_rows
        .into_iter()
        .map(|recommendation_row| RecommendationWithLogo {
            id: recommendation_row.id,
            logo_id: recommendation_row.logo_id,
            logo: Logo {
                path: recommendation_row.logo_path,
                name: recommendation_row.logo_name,
            },
            author: recommendation_row.author,
            texte: recommendation_row.texte,
        })
        .collect();

    Ok(recommendations)
}