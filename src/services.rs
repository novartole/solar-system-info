pub mod basic_auth;
pub mod planet_service;
pub mod rate_limit_service;
pub mod user_service;

use std::sync::Arc;

use axum::extract::FromRef;
use planet_service::PlanetService;
use redis::{aio::ConnectionManager, Client};
use user_service::UserService;

use crate::db::MongoDbClient;

use rate_limit_service::RateLimitService;

pub struct AppState {
    pub planet_service: PlanetService,
    pub user_service: UserService,
    pub rate_limit_service: RateLimitService,
}

impl AppState {
    pub fn new(
        mongodb_client: MongoDbClient,
        redis_client: Client,
        redis_connection_manager: ConnectionManager,
    ) -> Self {
        let planet_service = PlanetService::new(
            mongodb_client.clone(),
            redis_client,
            redis_connection_manager.clone(),
        );

        let user_service = UserService::new(mongodb_client);

        let rate_limit_service = RateLimitService::new(redis_connection_manager);

        Self {
            planet_service,
            user_service,
            rate_limit_service,
        }
    }
}

impl FromRef<Arc<AppState>> for UserService {
    fn from_ref(input: &Arc<AppState>) -> Self {
        input.user_service.clone()
    }
}

impl FromRef<Arc<AppState>> for RateLimitService {
    fn from_ref(input: &Arc<AppState>) -> Self {
        input.rate_limit_service.clone()
    }
}
