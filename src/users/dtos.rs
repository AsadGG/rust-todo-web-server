use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{types::Uuid, FromRow};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;
#[derive(Clone, Debug, Serialize)]
pub struct PathUuid {
    pub id: Uuid,
}

#[derive(Clone, Debug, FromRow, ToSchema)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn to_login_success(&self, token: String) -> LoginUserSuccess {
        return LoginUserSuccess {
            id: self.id,
            email: self.email.clone(),
            created_at: self.created_at,
            updated_at: self.updated_at,
            token,
        };
    }

    pub fn to_register_success(&self) -> RegisterUserSuccess {
        return RegisterUserSuccess {
            id: self.id,
            email: self.email.clone(),
            created_at: self.created_at,
            updated_at: self.updated_at,
        };
    }
}

#[derive(Clone, Debug, Serialize)]
pub struct LoginUserSuccess {
    pub id: Uuid,
    pub email: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
    pub token: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct RegisterUserSuccess {
    pub id: Uuid,
    pub email: String,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Validate, Deserialize, ToSchema, IntoParams)]
pub struct RegisterUser {
    #[validate(email(message = "invalid email address"))]
    #[schema(value_type = String, format = "email")]
    pub email: String,
    #[validate(length(min = 8, message = "password must have at least 8 characters"))]
    #[schema(value_type = String, min_length = 8)]
    pub password: String,
}

#[derive(Clone, Debug, Validate, Deserialize, ToSchema, IntoParams)]
pub struct LoginUser {
    #[validate(email(message = "invalid email address"))]
    #[schema(value_type = String, format = "email")]
    pub email: String,
    #[validate(length(min = 8, message = "password must have at least 8 characters"))]
    #[schema(value_type = String, min_length = 8)]
    pub password: String,
}
