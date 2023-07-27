use crate::todos;
use crate::users;

use utoipa::{Modify, OpenApi};
struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, _openapi: &mut utoipa::openapi::OpenApi) {}
}

#[derive(OpenApi)]
#[openapi(
    paths(
        todos::controllers::get_todos,
        todos::controllers::get_todo,
        todos::controllers::create_todo,
        todos::controllers::update_todo,
        todos::controllers::delete_todo,
        users::controllers::register_user,
        users::controllers::login_user,
    ),
    components(
        schemas(todos::dtos::CreateTodo,todos::dtos::UpdateTodo,users::dtos::RegisterUser,users::dtos::LoginUser)
    ),
    modifiers(&SecurityAddon)
)]
pub struct APIDocumentation;
