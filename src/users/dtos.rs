use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{types::Uuid, FromRow};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PathUuid {
    pub id: Uuid,
}

#[derive(Clone, Debug, Deserialize, FromRow, Serialize, ToSchema)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    #[serde(skip)]
    pub password: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Validate, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct RegisterUser {
    #[validate(email(message = "invalid email address"))]
    #[schema(value_type = String, format = "email")]
    pub email: String,
    #[validate(length(min = 8, message = "password must have at least 8 characters"))]
    #[schema(value_type = String, min_length = 8)]
    pub password: String,
}

#[derive(Clone, Debug, Validate, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct LoginUser {
    #[validate(email(message = "invalid email address"))]
    #[schema(value_type = String, format = "email")]
    pub email: String,
    #[validate(length(min = 8, message = "password must have at least 8 characters"))]
    #[schema(value_type = String, min_length = 8)]
    pub password: String,
}
