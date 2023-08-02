use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{types::Uuid, FromRow};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;
#[derive(Clone, Debug, Deserialize)]
pub struct PathUuid {
    pub id: Uuid,
}

const fn limit_default() -> i64 {
    const LIMIT_DEFAULT: i64 = 10;
    return LIMIT_DEFAULT;
}
const fn offset_default() -> i64 {
    const OFFSET_DEFAULT: i64 = 0;
    return OFFSET_DEFAULT;
}
#[derive(Clone, Debug, Deserialize, IntoParams, Validate)]
pub struct GetTodosQueryParam {
    #[validate(range(min = 10))]
    #[serde(default = "limit_default")]
    pub limit: i64,
    #[validate(range(min = 0))]
    #[serde(default = "offset_default")]
    pub offset: i64,
}

#[derive(Clone, Debug, FromRow, Serialize)]
pub struct Count {
    pub count: Option<i64>,
}

#[derive(Clone, Debug, FromRow, Serialize)]
pub struct Todo {
    pub completed: bool,
    pub description: String,
    pub id: Uuid,
    #[serde(rename = "userId")]
    pub user_id: Uuid,
    pub title: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Serialize)]
pub struct GetTodosSuccess {
    pub todos: Vec<Todo>,
    pub total: i64,
    pub page: i64,
    #[serde(rename = "perPage")]
    pub per_page: i64,
    #[serde(rename = "totalPages")]
    pub total_pages: i64,
}

#[derive(Clone, Debug, Deserialize, ToSchema)]
pub struct CreateTodo {
    pub description: String,
    pub title: String,
}

#[derive(Clone, Debug, Deserialize, ToSchema)]
pub struct UpdateTodo {
    pub completed: Option<bool>,
    pub description: Option<String>,
    pub title: Option<String>,
}
