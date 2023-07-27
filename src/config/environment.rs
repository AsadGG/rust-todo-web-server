use dotenv;

#[derive(Debug)]
pub struct EnvironmentVariables {
    pub database_url: String,
}

impl EnvironmentVariables {
    pub fn initialize() -> EnvironmentVariables {
        let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
        return EnvironmentVariables { database_url };
    }
}
