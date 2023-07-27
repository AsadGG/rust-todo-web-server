use dotenv;

#[derive(Debug)]
pub struct EnvironmentVariables {
    pub database_url: String,
}

pub fn get_environment_variables() -> EnvironmentVariables {
    dotenv::dotenv().ok();
    let database_url = dotenv::var("DATABASE_URL").expect("Key DATABASE_URL is Required");
    return EnvironmentVariables {
        database_url: database_url,
    };
}
