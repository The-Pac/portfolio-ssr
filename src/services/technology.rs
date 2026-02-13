use crate::models::database_error::{ModelError, TechnologyError};
use crate::models::technology::{TechnologyCategory, TechnologyRow, TechnologyWithLogo};
use sqlx::SqlitePool;
use std::collections::HashMap;

pub async fn get_all_technologies(
    connection_database: &SqlitePool,
) -> Result<Vec<TechnologyWithLogo>, TechnologyError> {
    let technologies = sqlx::query_as::<_, TechnologyRow>(
        r#"
        SELECT
            t.id AS technology_id,
            t.name AS technology_name,
            tc.title AS category_title,
            l.path AS logo_path,
            l.name AS logo_name
        FROM technology t
        INNER JOIN technology_category tc ON t.technology_category_id = tc.id
        INNER JOIN logo l ON t.logo_id = l.id
        ORDER BY tc.title, t.name
        "#,
    )
    .fetch_all(connection_database)
    .await
    .map_err(|e| ModelError::DatabaseError(e.into()))?;

    Ok(technologies
        .into_iter()
        .map(|t| TechnologyWithLogo {
            id: t.technology_id,
            name: t.technology_name,
            category_title: t.category_title,
            logo_path: t.logo_path,
            logo_name: t.logo_name,
        })
        .collect())
}

pub async fn get_technologies_by_category(
    connection_database: &SqlitePool,
) -> Result<HashMap<String, Vec<TechnologyWithLogo>>, TechnologyError> {
    let technologies = sqlx::query_as::<_, TechnologyRow>(
        r#"
        SELECT
            t.id AS technology_id,
            t.name AS technology_name,
            tc.title AS category_title,
            l.path AS logo_path,
            l.name AS logo_name
        FROM technology t
        INNER JOIN technology_category tc ON t.technology_category_id = tc.id
        INNER JOIN logo l ON t.logo_id = l.id
        ORDER BY tc.title, t.name
        "#,
    )
    .fetch_all(connection_database)
    .await
    .map_err(|e| ModelError::DatabaseError(e.into()))?;

    let mut grouped: HashMap<String, Vec<TechnologyWithLogo>> = HashMap::new();

    for tech in technologies {
        let category = tech.category_title.clone();
        grouped
            .entry(category)
            .or_insert_with(Vec::new)
            .push(TechnologyWithLogo {
                id: tech.technology_id,
                name: tech.technology_name,
                category_title: tech.category_title,
                logo_path: tech.logo_path,
                logo_name: tech.logo_name,
            });
    }

    Ok(grouped)
}

pub async fn get_all_technology_categories(
    connection_database: &SqlitePool,
) -> Result<Vec<TechnologyCategory>, TechnologyError> {
    let categories = sqlx::query_as::<_, TechnologyCategory>(
        r#"
        SELECT id, title, description
        FROM technology_category
        ORDER BY title
        "#,
    )
    .fetch_all(connection_database)
    .await
    .map_err(|e| ModelError::DatabaseError(e.into()))?;

    Ok(categories)
}
