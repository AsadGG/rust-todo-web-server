use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{types::Uuid, FromRow};
use utoipa::ToSchema;
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PathUuid {
    pub id: Uuid,
}

#[derive(Clone, Debug, Deserialize, FromRow, Serialize, ToSchema)]
pub struct Todo {
    pub completed: bool,
    pub description: String,
    pub id: Uuid,
    pub title: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateTodo {
    pub description: String,
    pub title: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct UpdateTodo {
    pub completed: Option<bool>,
    pub description: Option<String>,
    pub title: Option<String>,
}