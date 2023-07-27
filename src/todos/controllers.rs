#![allow(clippy::needless_return)]
use super::dtos::{CreateTodo, PathUuid, UpdateTodo};
use super::service;
use actix_web::{delete, get, patch, post, web, Responder};
use sqlx::{Pool, Postgres};

#[utoipa::path(tag = "Todos", path = "/api/todos")]
#[get("")]
pub async fn get_todos(pool: web::Data<Pool<Postgres>>) -> impl Responder {
    return service::get_todos(pool).await;
}

#[utoipa::path(
    tag = "Todos", path = "/api/todos/{id}",
params(
    ("id", description = "Unique storage id of Todo")
),)]
#[get("/{id}")]
pub async fn get_todo(
    pool: web::Data<Pool<Postgres>>,
    path: web::Path<PathUuid>,
) -> impl Responder {
    return service::get_todo(pool, path).await;
}

#[utoipa::path(
    tag = "Todos",
    path = "/api/todos",
    request_body = CreateTodo,
)]
#[post("")]
pub async fn create_todo(
    pool: web::Data<Pool<Postgres>>,
    create_todo_dto: web::Json<CreateTodo>,
) -> impl Responder {
    return service::create_todo(pool, create_todo_dto).await;
}

#[utoipa::path(
    tag = "Todos",
    path = "/api/todos/{id}",
    params(
        ("id", description = "Unique storage id of Todo")
    ),
    request_body = UpdateTodo,
)]
#[patch("/{id}")]
pub async fn update_todo(
    pool: web::Data<Pool<Postgres>>,
    path: web::Path<PathUuid>,
    update_todo_dto: web::Json<UpdateTodo>,
) -> impl Responder {
    return service::update_todo(pool, path, update_todo_dto).await;
}

#[utoipa::path(
    tag = "Todos",
    path = "/api/todos/{id}",
    params(
        ("id", description = "Unique storage id of Todo")
    )
)]
#[delete("/{id}")]
pub async fn delete_todo(
    pool: web::Data<Pool<Postgres>>,
    path: web::Path<PathUuid>,
) -> impl Responder {
    return service::delete_todo(pool, path).await;
}