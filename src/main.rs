#![allow(clippy::needless_return)]

mod config;
mod todos;
mod users;
use config::{database::get_pool, open_api::APIDocumentation};

use actix_web::{web, App, HttpServer};

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

    let http_server = HttpServer::new(move || {
        App::new()
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-docs/openapi.json", openapi.clone()),
            )
            .app_data(pool.clone())
            .configure(config)
    })
    .bind("localhost:8080")?;
    println!("Server Listening On http://localhost:8080/");
    return http_server.run().await;
}
