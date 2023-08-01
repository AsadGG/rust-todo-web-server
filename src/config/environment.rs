use dotenv;

#[derive(Debug)]
pub struct EnvironmentVariables {
    pub web_server_protocol: String,
    pub web_server_host: String,
    pub web_server_port: String,
    pub database_url: String,
    pub jwt_secret: String,
}

impl EnvironmentVariables {
    pub fn initialize() -> EnvironmentVariables {
        let web_server_protocol =
            dotenv::var("WEB_SERVER_PROTOCOL").expect("WEB_SERVER_PROTOCOL must be set");
        let web_server_host = dotenv::var("WEB_SERVER_HOST").expect("WEB_SERVER_HOST must be set");
        let web_server_port = dotenv::var("WEB_SERVER_PORT").expect("WEB_SERVER_PORT must be set");
        let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
        let jwt_secret = dotenv::var("JWT_SECRET").expect("JWT_SECRET must be set");
        return EnvironmentVariables {
            web_server_protocol,
            web_server_host,
            web_server_port,
            database_url,
            jwt_secret,
        };
    }
}
