#![allow(clippy::needless_return)]

use actix_web::{post, web, Responder};
use sqlx::{Pool, Postgres};

use super::dtos::{LoginUser, RegisterUser};
use super::service;

#[utoipa::path(
    tag = "Users",
    path = "/api/users/sign-up",
    request_body = RegisterUser,
)]
#[post("/sign-up")]
pub async fn register_user(
    pool: web::Data<Pool<Postgres>>,
    register_user_dto: web::Json<RegisterUser>,
) -> impl Responder {
    return service::register_user(pool, register_user_dto).await;
}

#[utoipa::path(
    tag = "Users",
    path = "/api/users/sign-in",
    request_body = LoginUser,
)]
#[post("/sign-in")]
pub async fn login_user(
    pool: web::Data<Pool<Postgres>>,
    login_user_dto: web::Json<LoginUser>,
) -> impl Responder {
    return service::login_user(pool, login_user_dto).await;
}
