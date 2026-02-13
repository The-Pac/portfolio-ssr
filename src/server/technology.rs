use std::collections::HashMap;
use leptos::server;
use crate::models::database_error::{TechnologyError};
use crate::models::technology::{Technology, TechnologyCategory, TechnologyWithLogo};

#[server]
pub async fn get_all_technology_categories() -> Result<Vec<TechnologyCategory>, TechnologyError> {
    #[cfg(feature = "ssr")]
    {
        use crate::libs::database::get_database;
        use crate::services;

        let database = get_database();
        return services::technology::get_all_technology_categories(&database).await;
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}


#[server]
pub async fn get_technologies_by_category() -> Result<HashMap<String, Vec<TechnologyWithLogo>>, TechnologyError> {
    #[cfg(feature = "ssr")]
    {
        use crate::libs::database::get_database;
        use crate::services;

        let database = get_database();
        return services::technology::get_technologies_by_category(&database).await;
    }
    #[cfg(not(feature = "ssr"))]
    {
        unreachable!()
    }
}