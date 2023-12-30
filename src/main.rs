mod db;
mod dto;
mod error;
mod handlers;
mod model;
mod redis;
mod services;

use std::{
    env,
    net::{Ipv4Addr, SocketAddr},
    sync::Arc,
};

use axum::{routing::get, Router};
use tokio::net::TcpListener;

use crate::services::AppState;

const MONGODB_URI: &str = "MONGODB_URI";
const REDIS_URI: &str = "REDIS_URI";

#[tokio::main]
async fn main() {
    if let Ok(config) = env::var("CONFIG_FILE") {
        dotenv::from_filename(config).ok();
    }

    env_logger::init();

    log::info!("Starting Solar system info server...");

    let mongodb_uri = env::var(MONGODB_URI).expect(&format!("{} should be specified", MONGODB_URI));
    let mongodb_client = db::MongoDbClient::new(mongodb_uri)
        .await
        .expect("Failed to create MongoDB client");

    let redis_uri = env::var(REDIS_URI).expect(&format!("{} should be specified", REDIS_URI));
    let redis_client =
        redis::create_redis_client(redis_uri).expect("Failed to create Redis client");
    let redis_connection_manager = redis_client
        .get_connection_manager()
        .await
        .expect("Failed to create Redis connection manager");

    let app_state = Arc::new(AppState::new(
        mongodb_client,
        redis_client,
        redis_connection_manager,
    ));

    let router = Router::new()
        .route("/", get(handlers::index))
        .route(
            "/planets",
            get(handlers::get_planets).post(handlers::create_planet),
        )
        .route(
            "/planets/:planet_id",
            get(handlers::get_planet)
                .delete(handlers::delete_planet)
                .put(handlers::update_planet),
        )
        .route("/planets/:planet/image", get(handlers::get_image_of_planet))
        .with_state(app_state);

    let ip = env::var("IP")
        .map_or_else(
            |_| {
                let ip = Ipv4Addr::LOCALHOST;

                log::warn!("IP env is not provided. Let's use a default one: {}", ip);

                ip
            },
            |s| {
                s.parse()
                    .expect(&format!("Failed to parse {} value to Ipv4Addr", s))
            },
        )
        .into();
    let port = env::var("PORT").map_or_else(
        |_| {
            let port = 9000;

            log::warn!(
                "PORT env is not provided. Let's use a default one: {}",
                port
            );

            port
        },
        |p| {
            p.parse()
                .expect(&format!("Failed to parse {} value into u16", p))
        },
    );

    let addr = SocketAddr::new(ip, port);
    let listener = TcpListener::bind(addr)
        .await
        .expect("Failed to create TCP listener");

    axum::serve(listener, router).await.unwrap();
}
