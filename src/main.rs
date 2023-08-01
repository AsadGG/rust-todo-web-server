#![allow(clippy::needless_return)]

mod config;
mod todos;
mod users;
use actix_web::{web, App, HttpServer};
use config::{database::get_pool, environment::EnvironmentVariables, open_api::APIDocumentation};

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub fn config(service_config: &mut web::ServiceConfig) {
    service_config.service(
        web::scope("/api")
            .configure(todos::routes::scoped_config)
            .configure(users::routes::scoped_config),
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = get_pool().await;
    let pool = web::Data::new(pool);

    let openapi = APIDocumentation::openapi();

    let environment_variables = EnvironmentVariables::initialize();
    let web_server_protocol = environment_variables.web_server_protocol.as_str();
    let web_server_host = environment_variables.web_server_host.as_str();
    let web_server_port = environment_variables.web_server_port.as_str();

    let http_server = HttpServer::new(move || {
        App::new()
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
            .app_data(pool.clone())
            .configure(config)
    })
    .bind(format!("{}:{}", web_server_host, web_server_port))?;
    let web_url = format!(
        "{}://{}:{}/",
        web_server_protocol, web_server_host, web_server_port
    );
    println!("Server Listening On {}", web_url);
    return http_server.run().await;
}
