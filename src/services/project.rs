use leptos::lazy;
use crate::models::database_error::{ModelError, ProjectError};
use crate::models::project::{Project, ProjectRow, ProjectStatus};
use crate::models::technology::{TechnologyRow, TechnologyWithLogo};
use sqlx::SqlitePool;
#[lazy]
pub async fn get_all_projects(
    connection_database: &SqlitePool,
) -> Result<Vec<Project>, ProjectError> {
    let project_rows = sqlx::query_as::<_, ProjectRow>(
        r#"
        SELECT
            p.id,
            p.title,
            p.description,
            ps.name AS status_name,
            p.status_id,
            p.url_to_project,
            p.stacks
        FROM project p
        INNER JOIN project_status ps ON p.status_id = ps.id
        ORDER BY p.id DESC
        "#,
    )
    .fetch_all(connection_database)
    .await
    .map_err(|e| ModelError::DatabaseError(e.into()))?;

    let mut projects = Vec::new();

    for project_row in project_rows {
        let stack_ids: Vec<i64> =
            serde_json::from_str(&project_row.stacks)
                .map_err(|e| ProjectError::StacksParseError {
                    project_id: project_row.id,
                    message: e.to_string(),
                })?;

        let technologies = if !stack_ids.is_empty() {
            let placeholders = stack_ids.iter().map(|_| "?").collect::<Vec<_>>().join(",");

            let query_str = format!(
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
                WHERE t.id IN ({})
                "#,
                placeholders
            );

            let mut query = sqlx::query_as::<_, TechnologyRow>(&query_str);
            for id in &stack_ids {
                query = query.bind(id);
            }

            query
                .fetch_all(connection_database)
                .await
                .map_err(|e| ModelError::DatabaseError(e.into()))?
        } else {
            Vec::new()
        };

        projects.push(Project {
            id: project_row.id,
            title: project_row.title,
            description: project_row.description,
            status: ProjectStatus::from_id(project_row.status_id),
            status_id: project_row.status_id,
            url_to_project: project_row.url_to_project,
            technologies: technologies
                .into_iter()
                .map(|t| TechnologyWithLogo {
                    id: t.technology_id,
                    name: t.technology_name,
                    category_title: t.category_title,
                    logo_path: t.logo_path,
                    logo_name: t.logo_name,
                })
                .collect(),
        });
    }

    Ok(projects)
}
