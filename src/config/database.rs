use sqlx::{Pool, Postgres};

use super::environment::get_environment_variables;

pub async fn get_pool() -> Pool<Postgres> {
    let environment_variables = get_environment_variables();
    let url = environment_variables.database_url.as_str();
    return sqlx::postgres::PgPool::connect(url)
        .await
        .expect("Could Not Connect To Database");
}
