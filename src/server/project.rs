use leptos::server;
use crate::models::database_error::{ProjectError};
use crate::models::project::Project;


#[server]
pub async fn get_all_projects() -> Result<Vec<Project>, ProjectError> {
    #[cfg(feature = "ssr")]
    {
        use crate::libs::database::get_database;
        use crate::services;

        let database = get_database();
        return services::project::get_all_projects(&database).await;
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}