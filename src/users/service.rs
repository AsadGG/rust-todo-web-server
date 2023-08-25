use actix_web::{http::StatusCode, web, HttpResponse, Responder};
use serde_json::json;
use sqlx::{Pool, Postgres};
use tracing::{error, info};
use validator::Validate;

use crate::{
    config::{argon2::Argon2PasswordHash, jwt::JWT},
    users::dtos::User,
};

use super::dtos::{LoginUser, RegisterUser};

pub async fn register_user(
    pool: web::Data<Pool<Postgres>>,
    register_user_dto: web::Json<RegisterUser>,
) -> impl Responder {
    let validation = register_user_dto.validate();
    if let Err(error) = validation {
        let json_error = json!({
            "errors": error,
            "statusCode": StatusCode::BAD_REQUEST.as_u16(),
        });
        error!("{}", serde_json::to_string(&json_error).unwrap());
        error!("{}", error);
        return HttpResponse::BadRequest().json(json_error);
    }

    let pool = pool.as_ref();
    let hashed_password = Argon2PasswordHash::hash_password(register_user_dto.password.to_owned());

    if let Err(error) = hashed_password {
        let json_error = json!({
            "message": "internal server error",
            "statusCode": StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
        });
        error!("{}", serde_json::to_string(&json_error).unwrap());
        error!("{}", error);
        return HttpResponse::InternalServerError().json(json_error);
    }
    let hashed_password = hashed_password.unwrap();

    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (email, password)
        VALUES ($1, $2)
        RETURNING *;
        "#,
        register_user_dto.email,
        hashed_password
    )
    .fetch_one(pool)
    .await;

    match user {
        Ok(user) => {
            let json_user = json!({
                "data": user.to_register_success(),
                "message": "user registered successfully",
                "statusCode": StatusCode::CREATED.as_u16(),
            });
            info!("{}", serde_json::to_string(&json_user).unwrap());
            return HttpResponse::Created().json(json_user);
        }
        Err(error) => {
            let database_error = error.as_database_error();
            if let Some(database_error) = database_error {
                if database_error.is_unique_violation() {
                    let json_error = json!({
                        "message": "email already exist",
                        "statusCode": StatusCode::CONFLICT.as_u16(),
                    });
                    error!("{}", serde_json::to_string(&json_error).unwrap());
                    error!("{}", error);
                    return HttpResponse::Conflict().json(json_error);
                }
            }
            let json_error = json!({
                "message": "internal server error",
                "statusCode": StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            });
            error!("{}", serde_json::to_string(&json_error).unwrap());
            error!("{}", error);
            return HttpResponse::InternalServerError().json(json_error);
        }
    }
}

pub async fn login_user(
    pool: web::Data<Pool<Postgres>>,
    login_user_dto: web::Json<LoginUser>,
) -> impl Responder {
    let validation = login_user_dto.validate();
    if let Err(error) = validation {
        let json_error = json!({
            "errors": error,
            "statusCode": StatusCode::BAD_REQUEST.as_u16(),
        });
        error!("{}", serde_json::to_string(&json_error).unwrap());
        error!("{}", error);
        return HttpResponse::BadRequest().json(json_error);
    }
    let pool = pool.as_ref();
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT * FROM users
        WHERE email = $1
        "#,
        login_user_dto.email
    )
    .fetch_one(pool)
    .await;

    match user {
        Ok(user) => {
            if Argon2PasswordHash::verify_password(
                login_user_dto.password.clone(),
                user.password.clone(),
            ) {
                let token = JWT::jwt_encode(user.id.to_string());

                match token {
                    Ok(token) => {
                        let json_user = json!({
                            "data": user.to_login_success(token),
                            "message": "user logged in successfully",
                            "statusCode": StatusCode::OK.as_u16(),
                        });
                        info!("{}", serde_json::to_string(&json_user).unwrap());
                        return HttpResponse::Ok().json(json_user);
                    }
                    Err(error) => {
                        let json_error = json!({
                            "message": "internal server error",
                            "statusCode": StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                        });
                        error!("{}", serde_json::to_string(&json_error).unwrap());
                        error!("{}", error);
                        return HttpResponse::InternalServerError().json(json_error);
                    }
                }
            }
            let json_error = json!({
                "message": "invalid credentials",
                "statusCode": StatusCode::UNAUTHORIZED.as_u16(),
            });
            error!("{}", serde_json::to_string(&json_error).unwrap());
            return HttpResponse::Unauthorized().json(json_error);
        }
        Err(error) => {
            let json_error = json!({
                "message": "invalid credentials",
                "statusCode": StatusCode::UNAUTHORIZED.as_u16(),
            });
            error!("{}", serde_json::to_string(&json_error).unwrap());
            error!("{}", error);
            return HttpResponse::Unauthorized().json(json_error);
        }
    }
}
