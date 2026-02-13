use leptos::prelude::{FromServerFnError, ServerFnErrorErr};
use leptos::server_fn::codec::JsonEncoding;
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum DatabaseError {
    #[error("Failed to initialize database: {0}")]
    InitializationError(String),

    #[error("Failed to connect to database at {path}: {message}")]
    ConnectionError { path: String, message: String },

    #[error("Database pool not initialized. Call init_database() first")]
    NotInitialized,

    #[error("SQL query failed: {query} - {message}")]
    QueryError { query: String, message: String },

    #[error("Failed to close database connection")]
    CloseError,

    #[error("Database error: {0}")]
    SqlxError(String),
}

#[cfg(feature = "ssr")]
impl From<sqlx::Error> for DatabaseError {
    fn from(err: sqlx::Error) -> Self {
        DatabaseError::SqlxError(err.to_string())
    }
}

#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum ModelError {
    #[error("Failed to parse JSON field '{field}': {message}")]
    JsonParseError { field: String, message: String },

    #[error("Failed to convert {from} to {to}: {reason}")]
    ConversionError {
        from: String,
        to: String,
        reason: String,
    },

    #[error("{entity} with {field} = {value} not found")]
    NotFound {
        entity: String,
        field: String,
        value: String,
    },

    #[error("Invalid {field}: {reason}")]
    InvalidData { field: String, reason: String },

    #[error("Foreign key constraint violated: {entity} references non-existent {referenced_entity} with id {id}")]
    ForeignKeyViolation {
        entity: String,
        referenced_entity: String,
        id: i64,
    },

    #[error("Database error: {0}")]
    DatabaseError(#[from] DatabaseError),
}

#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum ProjectError {
    #[error("Project with id {0} not found")]
    NotFound(i64),

    #[error("Invalid project status: {0}")]
    InvalidStatus(String),

    #[error("Invalid technology stack: {0}")]
    InvalidStack(String),

    #[error("Failed to parse stacks field for project {project_id}: {message}")]
    StacksParseError { project_id: i64, message: String },

    #[error("Technologies not found for project {project_id}: missing ids {missing_ids:?}")]
    TechnologiesNotFound {
        project_id: i64,
        missing_ids: Vec<i64>,
    },

    #[error(transparent)]
    ModelError(#[from] ModelError),
}

#[cfg(feature = "ssr")]
impl From<serde_json::Error> for ProjectError {
    fn from(err: serde_json::Error) -> Self {
        ProjectError::StacksParseError {
            project_id: 0,
            message: err.to_string(),
        }
    }
}

#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum TechnologyError {
    #[error("Technology with id {0} not found")]
    NotFound(i64),

    #[error("Invalid technology category: {0}")]
    InvalidCategory(String),

    #[error("Logo not found for technology {tech_id}")]
    LogoNotFound { tech_id: i64 },

    #[error(transparent)]
    ModelError(#[from] ModelError),
}

#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum CareerError {
    #[error("Career node with id {0} not found")]
    NotFound(i64),

    #[error("Invalid career structure: {0}")]
    InvalidStructure(String),

    #[error("Parent career node {parent_id} not found for node {node_id}")]
    ParentNotFound { node_id: i64, parent_id: i64 },

    #[error("Cycle detected in career tree at node {0}")]
    CycleDetected(i64),

    #[error(transparent)]
    ModelError(#[from] ModelError),
}

#[derive(Error, Debug, Clone, Serialize, Deserialize)]
pub enum LogoError {
    #[error("Logo with id {0} not found")]
    NotFound(i64),

    #[error("Invalid logo path: {0}")]
    InvalidPath(String),

    #[error("Logo file not found at path: {0}")]
    FileNotFound(String),

    #[error(transparent)]
    ModelError(#[from] ModelError),
}

pub type DbResult<T> = Result<T, DatabaseError>;

pub type ModelResult<T> = Result<T, ModelError>;

pub type ProjectResult<T> = Result<T, ProjectError>;

pub type TechnologyResult<T> = Result<T, TechnologyError>;

pub type CareerResult<T> = Result<T, CareerError>;

pub type LogoResult<T> = Result<T, LogoError>;

impl FromServerFnError for DatabaseError {
    type Encoder = JsonEncoding;

    fn from_server_fn_error(err: ServerFnErrorErr) -> Self {
        DatabaseError::SqlxError(err.to_string())
    }
}
impl FromServerFnError for ModelError {
    type Encoder = JsonEncoding;

    fn from_server_fn_error(err: ServerFnErrorErr) -> Self {
        ModelError::InvalidData {
            field: "unknown".to_string(),
            reason: err.to_string(),
        }
    }
}

impl FromServerFnError for ProjectError {
    type Encoder = JsonEncoding;
    fn from_server_fn_error(err: ServerFnErrorErr) -> Self {
        ProjectError::InvalidStack(err.to_string())
    }
}

impl FromServerFnError for TechnologyError {
    type Encoder = JsonEncoding;
    fn from_server_fn_error(err: ServerFnErrorErr) -> Self {
        TechnologyError::InvalidCategory(err.to_string())
    }
}

impl FromServerFnError for CareerError {
    type Encoder = JsonEncoding;
    fn from_server_fn_error(err: ServerFnErrorErr) -> Self {
        CareerError::InvalidStructure(err.to_string())
    }
}

impl FromServerFnError for LogoError {
    type Encoder = JsonEncoding;
    fn from_server_fn_error(err: ServerFnErrorErr) -> Self {
        LogoError::InvalidPath(err.to_string())
    }
}
