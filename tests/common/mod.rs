use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use tokio;
use uuid::Uuid;
use zero2prod;

use zero2prod::configuration::{get_configuration, DbConfig};
use zero2prod::startup::run;
use zero2prod::telemetry::{get_subscriber, init_subscriber};

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

lazy_static::lazy_static! {
    static ref TRACING: () = {
        let filter = if std::env::var("TEST_LOG").is_ok() { "debug" } else { "" };
        let subscriber = get_subscriber("test".into(), filter.into());
        init_subscriber(subscriber);
    };
}

async fn config_db(db_config: &DbConfig) -> PgPool {
    let mut db_conn = PgConnection::connect(&db_config.connection_string_without_db())
        .await
        .expect("Failed to connect to Postgres.");

    db_conn
        .execute(&*format!(
            r#"CREATE DATABASE "{}";"#,
            db_config.database_name
        ))
        .await
        .expect("Failed to create database.");

    let db_pool = PgPool::connect(&db_config.connection_string())
        .await
        .expect("Failed to connect to Postgres.");
    sqlx::migrate!("./migrations")
        .run(&db_pool)
        .await
        .expect("Failed to migrate database");

    return db_pool;
}

// Launch our application in the background ~somehow~
pub async fn spawn_app() -> TestApp {
    lazy_static::initialize(&TRACING);

    // load app_config
    let mut app_config = get_configuration().expect("Failed to read configuration.");

    // randomize db_name
    app_config.database.database_name = format!(
        "{}-{}",
        &app_config.database.database_name,
        Uuid::new_v4().to_string(),
    );

    // setup tcp listener
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = format!("http://127.0.0.1:{}", port);

    // setup test database
    let db_pool = config_db(&app_config.database).await;

    let server = run(listener, db_pool.clone()).expect("Failed to bind address");
    let _ = tokio::spawn(server);

    TestApp { address, db_pool }
}
