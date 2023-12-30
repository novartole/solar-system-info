use std::str::FromStr;

use mongodb::bson::oid::ObjectId;
use redis::{aio::ConnectionManager, AsyncCommands, Client, Value};

use crate::{
    db::MongoDbClient,
    dto::PlanetMessage,
    error::{CustomError, CustomResult},
    model::{Planet, PlanetType},
};

const PLANET_KEY_PREFIX: &str = "planet";
const PLANET_IMAGE_KEY_PREFIX: &str = "image";
const NEW_PLANETS_CHANNEL_NAME: &str = "new_planets";

pub struct PlanetService {
    mongodb_client: MongoDbClient,
    redis_client: Client,
    redis_connection_manager: ConnectionManager,
}

impl PlanetService {
    pub fn new(
        mongodb_client: MongoDbClient,
        redis_client: Client,
        redis_connection_manager: ConnectionManager,
    ) -> Self {
        Self {
            mongodb_client,
            redis_client,
            redis_connection_manager,
        }
    }

    fn get_planet_cache_key(&self, planet_id: &str) -> String {
        format!("{}{}", PLANET_KEY_PREFIX, planet_id)
    }

    fn get_planet_image_cache_key(&self, planet_id: &str) -> String {
        format!(
            "{}{}{}",
            PLANET_KEY_PREFIX, planet_id, PLANET_IMAGE_KEY_PREFIX
        )
    }

    pub async fn get_planets(&self, planet_type: Option<PlanetType>) -> CustomResult<Vec<Planet>> {
        self.mongodb_client.get_planets(planet_type).await
    }

    pub async fn get_planet(&self, planet_id: &str) -> CustomResult<Planet> {
        let cache_key = self.get_planet_cache_key(planet_id);

        let mut con = self.redis_client.get_async_connection().await?;

        match con.get(&cache_key).await? {
            Value::Nil => {
                log::debug!("No cached value - getting planet from db");

                let planet = self
                    .mongodb_client
                    .get_planet(ObjectId::from_str(planet_id)?)
                    .await?;

                let _ = redis::pipe()
                    .atomic()
                    .set(&cache_key, &planet)
                    .expire(&cache_key, 60)
                    .query_async(&mut con)
                    .await?;

                Ok(planet)
            }
            Value::Data(data) => {
                log::debug!("Return cached planet");

                let planet = serde_json::from_slice(&data)?;

                Ok(planet)
            }
            res => Err(CustomError::RedisError {
                message: format!("Unexpected response from Redis: {:?}", res),
            }),
        }
    }

    pub async fn create_planet(&self, planet: Planet) -> CustomResult<Planet> {
        let planet = self.mongodb_client.create_planet(planet).await?;

        let planet_message = PlanetMessage::from(&planet);
        self.redis_connection_manager
            .clone()
            .publish(
                NEW_PLANETS_CHANNEL_NAME,
                serde_json::to_string(&planet_message)?,
            )
            .await?;

        Ok(planet)
    }

    pub async fn update_planet(&self, planet_id: &str, planet: Planet) -> CustomResult<Planet> {
        let planet = {
            let planet_id = ObjectId::from_str(planet_id)?;
            self.mongodb_client.update_planet(planet_id, planet).await?
        };

        let cache_key = self.get_planet_cache_key(planet_id);
        self.redis_connection_manager.clone().del(cache_key).await?;

        Ok(planet)
    }

    pub async fn delete_planet(&self, planet_id: &str) -> CustomResult<()> {
        {
            let planet_id = ObjectId::from_str(planet_id)?;
            self.mongodb_client.delete_planet(planet_id).await?;
        }

        let cache_key = self.get_planet_cache_key(planet_id);
        self.redis_connection_manager.clone().del(cache_key).await?;

        Ok(())
    }

    pub async fn get_planet_image(&self, planet_id: &str) -> CustomResult<Vec<u8>> {
        let cache_key = self.get_planet_image_cache_key(planet_id);

        let mut con = self.redis_client.get_async_connection().await?;

        match con.get(&cache_key).await? {
            Value::Nil => {
                let planet_id = ObjectId::from_str(planet_id)?;
                let planet = self.mongodb_client.get_planet(planet_id).await?;
                let result = crate::db::get_planet_image(&planet.name);

                let _ = redis::pipe()
                    .set(&cache_key, result.clone())
                    .expire(&cache_key, 60)
                    .query_async(&mut con)
                    .await?;

                return Ok(result);
            }
            Value::Data(value) => Ok(value),
            res => Err(CustomError::RedisError {
                message: format!("Unexpected response from Redis: {:?}", res),
            }),
        }
    }
}
