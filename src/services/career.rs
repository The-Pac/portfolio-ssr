use leptos::lazy;
use crate::models::career::CareerNode;
use crate::models::database_error::{CareerError, ModelError};
use sqlx::SqlitePool;

#[lazy]
pub async fn get_career_nodes(
    connection_database: &SqlitePool,
) -> Result<Vec<CareerNode>, CareerError> {
    let career_nodes = sqlx::query_as::<_, CareerNode>(
        r#"
            SELECT
                c.id,
                c.title,
                c.year,
                c.parent_id,
                t.name AS technology_name,
                tc.title AS technology_category_title,
                COALESCE(cl.path, tl.path) AS logo_path,
                COALESCE(cl.name, tl.name) AS logo_name
            FROM careers c
            LEFT JOIN technology t ON c.technology_id = t.id
            LEFT JOIN technology_category tc ON t.technology_category_id = tc.id
            LEFT JOIN logo cl ON c.logo_id = cl.id
            LEFT JOIN logo tl ON t.logo_id = tl.id
        "#,
    )
    .fetch_all(connection_database)
    .await
    .map_err(|e| ModelError::DatabaseError(e.into()))?;

    Ok(career_nodes)
}
