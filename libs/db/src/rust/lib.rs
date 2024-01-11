pub mod models;

use sea_orm::{DatabaseConnection,  Database, ConnectOptions};

pub async fn new() -> Result<DatabaseConnection, ()> {
    let db_url = makoto_config::db::Database::new().database_url.expect("cannot get database_url from env!");

    let mut connection_options = ConnectOptions::new(db_url);
    connection_options.max_connections(100).sqlx_logging(false);

    let db = Database::connect(connection_options).await.map_err(|err| {
        eprintln!("Failed to connect to database: {err}");
    })?;

    Ok(db)
}

pub async fn get_test_db() -> DatabaseConnection {
    let db_url = makoto_config::db::Database::new().database_test_url.unwrap_or("postgres://postgres:postgres@localhost/postgres".to_string());

    let connection_options = ConnectOptions::new(db_url);

    Database::connect(connection_options).await.unwrap()
}

pub mod utilities {
    use sea_orm::{ActiveValue, Value, sea_query::Nullable};

    pub fn nullable<T: Into<Value> + Nullable>(value: T) -> ActiveValue<Option<T>> {
    ActiveValue::Set(Some(value))
    }

    pub fn not_null<T: Into<Value>>(value: T) -> ActiveValue<T> {
    ActiveValue::Set(value)
    }
}
