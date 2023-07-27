use super::controllers::{create_todo, delete_todo, get_todo, get_todos, update_todo};
use actix_web::web;

pub fn scoped_config(service_config: &mut web::ServiceConfig) {
    let todo_scope = web::scope("/todos")
        .service(get_todos)
        .service(get_todo)
        .service(create_todo)
        .service(update_todo)
        .service(delete_todo);
    service_config.service(todo_scope);
}
