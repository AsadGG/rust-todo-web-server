use actix_web::{web, HttpResponse, Responder};
use serde_json::json;
use sqlx::{Pool, Postgres};
use validator::Validate;

use crate::{config::argon2::Argon2PasswordHash, users::dtos::User};

use super::dtos::{LoginUser, RegisterUser};

pub async fn register_user(
    pool: web::Data<Pool<Postgres>>,
    register_user_dto: web::Json<RegisterUser>,
) -> impl Responder {
    let validation = register_user_dto.validate();
    if let Err(error) = validation {
        let json_user = json!({
            "errors":error,
            "statusCode": 400,
        });
        return HttpResponse::BadRequest().json(json_user);
    }

    let pool = pool.as_ref();
    let hashed_password = Argon2PasswordHash::hash_password(register_user_dto.password.to_owned());

    if hashed_password.is_err() {
        let json_user = json!({
            "message":"internal server error",
            "statusCode": 500,
        });
        return HttpResponse::InternalServerError().json(json_user);
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
                "data":user,
                "message":"user registered successfully",
                "statusCode": 201,
            });
            return HttpResponse::Created().json(json_user);
        }
        Err(error) => {
            let database_error = error.as_database_error();
            if let Some(database_error) = database_error {
                if database_error.is_unique_violation() {
                    let json_user = json!({
                        "message":"email already exist",
                        "statusCode": 409,
                    });
                    return HttpResponse::Conflict().json(json_user);
                }
            }
            let json_user = json!({
                "message":"internal server error",
                "statusCode": 500,
            });
            return HttpResponse::InternalServerError().json(json_user);
        }
    }
}

pub async fn login_user(
    pool: web::Data<Pool<Postgres>>,
    login_user_dto: web::Json<LoginUser>,
) -> impl Responder {
    let validation = login_user_dto.validate();
    if let Err(error) = validation {
        let json_user = json!({
            "errors": error,
            "statusCode": 400,
        });
        return HttpResponse::BadRequest().json(json_user);
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
                let json_user = json!({
                    "data":user,
                    "message":"user logged in successfully",
                    "statusCode": 200,
                });
                return HttpResponse::Ok().json(json_user);
            }
            let json_user = json!({
                "message":"invalid credentials",
                "statusCode": 401,
            });
            return HttpResponse::Unauthorized().json(json_user);
        }
        Err(_) => {
            let json_user = json!({
                "message":"invalid credentials",
                "statusCode": 401,
            });
            return HttpResponse::Unauthorized().json(json_user);
        }
    }
}
