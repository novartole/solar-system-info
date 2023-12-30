pub mod basic_auth;
pub mod planet_service;
pub mod user_service;

use std::sync::Arc;

use axum::extract::FromRef;
use planet_service::PlanetService;
use redis::{aio::ConnectionManager, Client};
use user_service::UserService;

use crate::db::MongoDbClient;

pub struct AppState {
    pub planet_service: PlanetService,
    pub user_service: UserService,
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
            redis_connection_manager,
        );

        let user_service = UserService::new(mongodb_client);

        Self {
            planet_service,
            user_service,
        }
    }
}

impl FromRef<Arc<AppState>> for UserService {
    fn from_ref(input: &Arc<AppState>) -> Self {
        input.user_service.clone()
    }
}
