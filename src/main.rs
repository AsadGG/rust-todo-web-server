#![allow(clippy::needless_return)]

mod config;
mod todos;
mod users;

use actix_web::{get, http::StatusCode, web, App, HttpResponse, HttpServer, Responder};
use config::{database::get_pool, environment::EnvironmentVariables, open_api::APIDocumentation};

use serde_json::json;
use tracing::info;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use tracing_actix_web::TracingLogger;

use crate::config::logger::initialize_logger;

pub fn config(service_config: &mut web::ServiceConfig) {
    service_config.service(
        web::scope("/api")
            .configure(todos::routes::scoped_config)
            .configure(users::routes::scoped_config),
    );
}

#[get("/health-check")]
async fn health_check() -> impl Responder {
    let json_response = json!({
        "message": "server is running",
        "statusCode": StatusCode::OK.as_u16(),
    });
    return HttpResponse::Ok().json(json_response);
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

    initialize_logger();

    let http_server = HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .service(health_check)
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
    info!("Server Listening On {}", web_url);
    return http_server.run().await;
}
