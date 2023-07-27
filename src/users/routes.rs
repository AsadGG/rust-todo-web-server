use super::controllers::{login_user, register_user};
use actix_web::web;

pub fn scoped_config(service_config: &mut web::ServiceConfig) {
    let user_scope = web::scope("/users")
        .service(register_user)
        .service(login_user);
    service_config.service(user_scope);
}
