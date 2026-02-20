use crate::models::career::{CareerNode};
use leptos::server;
use crate::models::database_error::{CareerError};

#[server]
pub async fn load_career() -> Result<Vec<CareerNode>, CareerError> {
    #[cfg(feature = "ssr")]
    {
        use crate::libs::database::get_database;
        use crate::services;

        let database = get_database();
        return services::career::get_career_nodes(database).await;
    }

    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}
