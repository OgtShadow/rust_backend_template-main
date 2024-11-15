use axum::{routing::get, Router};
use diesel::{
    r2d2::{ConnectionManager, Pool},
    PgConnection,
};
use dotenvy::dotenv;
use linuxiarnia::controllers;
use linuxiarnia::{models};
use models::app_state::{AppState, DbPool};
use std::env;
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let database_url: String = env::var("DATABASE_URL").expect("database url must be set");
    let manager: ConnectionManager<PgConnection> =
        ConnectionManager::<PgConnection>::new(database_url);
    let pool: DbPool = Pool::builder()
        .build(manager)
        .expect("Failed to create Pool");
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;

    let app_state: AppState = AppState { pools: pool };

    let app = Router::new()
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive())
        )
        .route("/", get(|| async { "Hello World" }))
        .nest("/users", controllers::users_controller::configure_paths())
        .with_state(app_state);

    let listener: TcpListener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
    Ok(())
}
