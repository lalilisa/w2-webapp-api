pub mod api;

pub mod repo;
mod function;
pub mod common;
pub mod model;
mod middleware;
mod consumer;
mod job;

use axum::routing::{delete, get, post, put};
use axum::{Extension, Router};
use once_cell::sync::Lazy;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};
use sqlx_template::multi_query;
use std::env;
use std::sync::Arc;
use axum::handler::Handler;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing::info;
use tracing_appender::rolling;
use tracing_appender::rolling::Rotation;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{fmt, EnvFilter};
use crate::common::kafka::consumer::KafkaConsumer;
use crate::consumer::notification_handler::NotificationConsumerHandler;

#[multi_query(file = "sql/init.sql", 0)]
async fn migrate() {}
pub async fn start_server() -> anyhow::Result<()> {

    init_app().await?;

    let test_route = Router::new()
        .route("/", get(api::exam::list_tests::process).post(api::exam::create_test::process))
        .route("/:id", put(api::exam::update_test::process).delete(api::exam::delete_test::process));

    let user_route = Router::new()
        .route("/userInfo", get(api::user::user_info::process))
        .route("/doTest", post(api::exam::test_result::user_test::process))
        .route("/submitTest", post(api::exam::test_result::submit_test::process))
        .layer(axum::middleware::from_fn(middleware::authentication::authentication_middleware));

    let template_route = Router::new()
        .route("/", get(api::template::get_all_templates::process).post(api::template::create_template::process))
        .route("/:id", get(api::template::update_template::process).put(api::template::update_template::process).delete(api::template::delete_template::process))
        .layer(axum::middleware::from_fn(middleware::authorization::authorized_middleware))
        .layer(axum::middleware::from_fn(middleware::authentication::authentication_middleware))
        ;

    let question_route = Router::new()
        .route("/", get(api::exam::question::get_question_part::process))
        .route("/",
            post(api::exam::question::create_question::process)
                .layer(axum::middleware::from_fn(middleware::authorization::authorized_middleware))
                .layer(axum::middleware::from_fn(middleware::authentication::authentication_middleware))

        )
        .route("/:id", delete(api::exam::question::delete_question::process).put(api::exam::question::update_question::process)
            .layer(axum::middleware::from_fn(middleware::authorization::authorized_middleware))
            .layer(axum::middleware::from_fn(middleware::authentication::authentication_middleware))
        )
        ;

    let app = Router::new()
        .route("/healCheck", get(|| async { "Hello, World!" }))
        .route("/api/v1/login", post(api::auth::login::process))
        .route("/api/v1/register", post(api::auth::register::process))
        .nest("/api/v1/test", test_route)
        .nest("/api/v1/user", user_route)
        .nest("/api/v1/questions", question_route)
        .nest("/api/v1/template", template_route)
        .layer(TraceLayer::new_for_http());

    let server_port = env::var("SERVER_PORT").expect("SERVER_PORT NOT FOUND");

    let server_addr = format!("0.0.0.0:{}", server_port);
    let listener = TcpListener::bind(&server_addr).await?;

    info!("Server listening on {:?}", server_addr);

    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}

fn init_pool_db() -> anyhow::Result<Pool<Postgres>> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL NOT FOUND");
    let pool = PgPoolOptions::new().max_connections(5);
    let pool = pool.connect_lazy(&database_url)?;
    Ok(pool)
}
fn init_tracer() {
    let appender = rolling::RollingFileAppender::builder()
        .max_log_files(1000)
        .filename_prefix("application")
        .filename_suffix("log")
        .rotation(Rotation::DAILY)
        .build("logs")
        .expect("Failed to build RollingFileAppender");

    let (non_blocking, _guard) = tracing_appender::non_blocking(appender);
    let file_layer = fmt::layer().with_writer(non_blocking).with_ansi(false);

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::fmt::layer()
                .with_thread_names(true)
                .with_thread_ids(true),
        )
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .with(file_layer)
        .init();
}

static DB_POOL: Lazy<Pool<Postgres>> = Lazy::new(|| {
    info!("init pool");
    match init_pool_db() {
        Ok(p) => p,
        Err(e) => {
            panic!("Invalid database config: {}", e);
        }
    }
});

pub fn get_db_conn() -> anyhow::Result<&'static Pool<Postgres>> {
    Ok(&DB_POOL)
}

async fn init_kafka() -> anyhow::Result<()> {
    info!("init kafka");
    let kafka_urls = env::var("KAFKA_URLS").expect("KAFKA_URL NOT FOUND");

    // common::kafka::producer::init_kafka_producer(&kafka_urls);
    let consumer = KafkaConsumer::new(&kafka_urls,"notification","notification");

    let handler =Arc::new(NotificationConsumerHandler{});
    consumer.start_consuming(handler).await;
    Ok(())
}

async fn  init_app() -> anyhow::Result<()> {
    init_tracer();
    info!("init app");
    init_kafka().await?;
    migrate(get_db_conn()?).await?;
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL NOT FOUND");
    repo::redis::init_redis(&redis_url).await?;
    job::init_jobs().await;
    Ok(())
}
