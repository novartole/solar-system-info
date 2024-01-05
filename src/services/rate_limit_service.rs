use std::net::SocketAddr;

use axum::{extract::{FromRequestParts, FromRef, ConnectInfo}, http::request::Parts, async_trait};
use chrono::{Utc, Timelike};
use redis::aio::ConnectionManager;

use crate::error::{CustomResult, CustomError};

const RATE_LIMIT_KEY_PREFIX: &str = "rate_limit";
const MAX_REQUEST_PER_MINUTE: u64 = 10;

#[derive(Clone)]
pub struct RateLimitService {
    redis_connection_manager: ConnectionManager,
}

impl RateLimitService {
    pub fn new(redis_connection_manager: ConnectionManager) -> Self {
        Self {
            redis_connection_manager
        }
    }

    pub async fn assert_rate_limit_not_exceeded(&self, client_addr: &SocketAddr) -> CustomResult<()> {
        let current_minute = Utc::now().minute();
        let rate_minute_key = format!("{}:{}:{}", RATE_LIMIT_KEY_PREFIX, client_addr.ip(), current_minute);

        let (count, _): (u64, u64) = redis::pipe()
            .atomic()
            .incr(&rate_minute_key, 1)
            .expire(&rate_minute_key, 60)
            .query_async(&mut self.redis_connection_manager.clone())
            .await?;

        if count > MAX_REQUEST_PER_MINUTE {
            return Err(CustomError::TooManyRequests {
                actual_count: count,
                permission_count: MAX_REQUEST_PER_MINUTE,
            });
        }

        Ok(())
    }
}

pub struct RateLimit;

#[async_trait]
impl<S> FromRequestParts<S> for RateLimit
where
    RateLimitService: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = CustomError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let ConnectInfo(addr) = parts.extensions.get().unwrap();

        RateLimitService::from_ref(state)
            .assert_rate_limit_not_exceeded(addr)
            .await?;

        Ok(Self)
    }
}
