use actix_web::{
    http::StatusCode,
    web::{self, ReqData},
    HttpResponse, Responder,
};
use chrono::Utc;
use serde_json::json;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::todos::dtos::Todo;

use super::dtos::{CreateTodo, PathUuid, UpdateTodo};

pub async fn get_todo(
    pool: web::Data<Pool<Postgres>>,
    req_data: Option<ReqData<String>>,
    path: web::Path<PathUuid>,
) -> impl Responder {
    let user_id = Uuid::parse_str(req_data.unwrap().into_inner().as_str()).unwrap();
    let pool = pool.as_ref();
    let todo_id = path.into_inner().id;
    let todo = sqlx::query_as!(
        Todo,
        r#"
        SELECT * FROM todos
        WHERE id = $1 AND user_id = $2
        "#,
        todo_id,
        user_id
    )
    .fetch_one(pool)
    .await;

    match todo {
        Ok(todo) => {
            let json_todo = json!({
                "data":todo,
                "message":"todo fetched successfully",
                "statusCode": StatusCode::OK.as_u16(),
            });
            return HttpResponse::Ok().json(json_todo);
        }
        Err(_) => {
            let json_todo = json!({
                "message":format!("todo with ID: {} not found", todo_id),
                "statusCode": StatusCode::NOT_FOUND.as_u16(),
            });
            return HttpResponse::NotFound().json(json_todo);
        }
    }
}

pub async fn get_todos(
    pool: web::Data<Pool<Postgres>>,
    req_data: Option<ReqData<String>>,
) -> impl Responder {
    let user_id = Uuid::parse_str(req_data.unwrap().into_inner().as_str()).unwrap();
    let pool = pool.as_ref();
    let todos = sqlx::query_as!(
        Todo,
        r#"
        SELECT * FROM todos
        WHERE user_id = $1
        "#,
        user_id
    )
    .fetch_all(pool)
    .await;

    match todos {
        Ok(todos) => {
            if !todos.is_empty() {
                let json_todo = json!({
                    "data":todos,
                    "message":"todos fetched successfully",
                    "statusCode": StatusCode::OK.as_u16(),
                });
                return HttpResponse::Ok().json(json_todo);
            }
            let json_todo = json!({
                "message":"todos does not exist",
                "statusCode": StatusCode::NOT_FOUND.as_u16(),
            });
            return HttpResponse::NotFound().json(json_todo);
        }
        Err(_) => {
            let json_todo = json!({
                "message":"internal server error",
                "statusCode": StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            });
            return HttpResponse::InternalServerError().json(json_todo);
        }
    }
}

pub async fn create_todo(
    pool: web::Data<Pool<Postgres>>,
    req_data: Option<ReqData<String>>,
    create_todo_dto: web::Json<CreateTodo>,
) -> impl Responder {
    let user_id = Uuid::parse_str(req_data.unwrap().into_inner().as_str()).unwrap();
    let pool = pool.as_ref();

    let todo = sqlx::query_as!(
        Todo,
        r#"
        INSERT INTO todos (title, description, user_id)
        VALUES ($1, $2, $3)
        RETURNING *;
        "#,
        create_todo_dto.title,
        create_todo_dto.description,
        user_id
    )
    .fetch_one(pool)
    .await;

    match todo {
        Ok(todo) => {
            let json_todo = json!({
                "data":todo,
                "message":"todo created successfully",
                "statusCode": StatusCode::CREATED.as_u16(),
            });
            return HttpResponse::Created().json(json_todo);
        }
        Err(_) => {
            let json_todo = json!({
                "message":"internal server error",
                "statusCode": StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            });
            return HttpResponse::InternalServerError().json(json_todo);
        }
    }
}

pub async fn update_todo(
    pool: web::Data<Pool<Postgres>>,
    req_data: Option<ReqData<String>>,
    path: web::Path<PathUuid>,
    update_todo_dto: web::Json<UpdateTodo>,
) -> impl Responder {
    let user_id = Uuid::parse_str(req_data.unwrap().into_inner().as_str()).unwrap();
    let pool = pool.as_ref();
    let todo_id = path.into_inner().id;
    let query_result = sqlx::query_as!(
        Todo,
        r#"
        SELECT * FROM todos
        WHERE id = $1 AND user_id = $2
        "#,
        todo_id,
        user_id
    )
    .fetch_one(pool)
    .await;

    if query_result.is_err() {
        let json_todo = json!({
            "message":format!("todo with ID: {} not found", todo_id),
            "statusCode": StatusCode::NOT_FOUND.as_u16(),
        });
        return HttpResponse::NotFound().json(json_todo);
    }

    let now = Utc::now();
    let todo = query_result.unwrap();

    let query_result = sqlx::query_as!(
        Todo,
        r#"
        UPDATE todos
        SET title = $1, description = $2, completed = $3, updated_at = $4
        WHERE id = $5
        RETURNING *
        "#,
        update_todo_dto.title.to_owned().unwrap_or(todo.title),
        update_todo_dto
            .description
            .to_owned()
            .unwrap_or(todo.description),
        update_todo_dto
            .completed
            .to_owned()
            .unwrap_or(todo.completed),
        now,
        todo_id
    )
    .fetch_one(pool)
    .await;

    match query_result {
        Ok(todo) => {
            let json_todo = json!({
                "message":"todo updated successfully",
                "statusCode": StatusCode::OK.as_u16(),
                "data": todo
            });
            return HttpResponse::Ok().json(json_todo);
        }
        Err(_) => {
            let json_todo = json!({
                "message":"todo updating failed",
                "statusCode": StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
            });
            return HttpResponse::InternalServerError().json(json_todo);
        }
    }
}

pub async fn delete_todo(
    pool: web::Data<Pool<Postgres>>,
    req_data: Option<ReqData<String>>,
    path: web::Path<PathUuid>,
) -> impl Responder {
    let user_id = Uuid::parse_str(req_data.unwrap().into_inner().as_str()).unwrap();
    let pool = pool.as_ref();
    let todo_id = path.into_inner().id;
    let rows_affected = sqlx::query!(
        r#"
        DELETE FROM todos
        WHERE id = $1 AND user_id = $2
        "#,
        todo_id,
        user_id
    )
    .execute(pool)
    .await
    .unwrap()
    .rows_affected();

    if rows_affected == 0 {
        let json_todo = json!({
            "message":format!("todo with ID: {} not found", todo_id),
            "statusCode": StatusCode::NOT_FOUND.as_u16(),
        });
        return HttpResponse::NotFound().json(json_todo);
    }
    let json_todo = json!({
        "message":"todo deleted successfully",
        "statusCode": StatusCode::OK.as_u16(),
    });
    return HttpResponse::Ok().json(json_todo);
}
