use std::fmt;
use serde::{Deserialize, Serialize};
use crate::models::technology::TechnologyWithLogo;

#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectDb {
    id: i64,
    status_id: i64,
    description: Option<String>,
    title: String,
    stacks: String,
    url_to_project: Option<String>,
}

#[cfg_attr(feature = "ssr", derive(sqlx::FromRow))]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectRow {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub status_name: String,
    pub status_id: i64,
    pub url_to_project: Option<String>,
    pub stacks: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub status: ProjectStatus,
    pub status_id: i64,
    pub url_to_project: Option<String>,
    pub technologies: Vec<TechnologyWithLogo>,
}
#[derive(Clone, Debug,Serialize, Deserialize)]
pub enum ProjectStatus {
    Idea,
    InProgress,
    Cancelled,
    Completed,
    Pending,
    Archived,
}
impl ProjectStatus {
    pub fn from_id(id: i64) -> Self {
        match id {
            1 => ProjectStatus::Idea,
            2 => ProjectStatus::InProgress,
            3 => ProjectStatus::Cancelled,
            4 => ProjectStatus::Completed,
            5 => ProjectStatus::Pending,
            6 => ProjectStatus::Archived,
            _ => unreachable!(),
        }
    }
}

impl fmt::Display for ProjectStatus {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let label = match self {
            ProjectStatus::Idea => "Idée",
            ProjectStatus::InProgress => "En cours",
            ProjectStatus::Cancelled => "Annulé",
            ProjectStatus::Completed => "Terminé",
            ProjectStatus::Pending => "En attente",
            ProjectStatus::Archived => "Archivé",
        };
        write!(formatter, "{}", label)
    }
}

