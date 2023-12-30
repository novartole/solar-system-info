use mongodb::{
    bson::{doc, oid::ObjectId, Document},
    error::Result,
    Client, Collection,
};
use rust_embed::RustEmbed;
use tokio_stream::StreamExt;

use crate::{
    error::{CustomError, CustomResult},
    model::{Planet, PlanetType, User},
};

const DB_NAME: &str = "solar_system_info";
const PLANETS_COLLECTION_NAME: &str = "planets";
const USERS_COLLECTION_NAME: &str = "users";

#[derive(Clone)]
pub struct MongoDbClient {
    client: Client,
}

impl MongoDbClient {
    pub async fn new(uri: String) -> Result<Self> {
        Ok(Self {
            client: Client::with_uri_str(uri).await?,
        })
    }

    fn get_planets_collection(&self) -> Collection<Planet> {
        self.client
            .database(DB_NAME)
            .collection(PLANETS_COLLECTION_NAME)
    }

    fn get_users_collection(&self) -> Collection<User> {
        self.client
            .database(DB_NAME)
            .collection(USERS_COLLECTION_NAME)
    }

    pub async fn get_user(&self, username: String) -> CustomResult<User> {
        let filter = doc! { "username": username.clone() };

        self.get_users_collection()
            .find_one(filter, None)
            .await?
            .ok_or(CustomError::UserNotFound {
                message: format!("Can't find a user by username: {}", username),
            })
    }

    pub async fn get_planets(&self, planet_type: Option<PlanetType>) -> CustomResult<Vec<Planet>> {
        let filter = planet_type.map(|pt| doc! { "type": pt.to_string() });
        let mut planets = self.get_planets_collection().find(filter, None).await?;

        let mut result = Vec::new();

        while let Some(planet) = planets.next().await {
            result.push(planet?);
        }

        Ok(result)
    }

    pub async fn get_planet(&self, planet_id: ObjectId) -> CustomResult<Planet> {
        let filter = doc! { "_id": &planet_id };

        self.get_planets_collection()
            .find_one(filter, None)
            .await?
            .ok_or(CustomError::NotFound {
                message: format!("Can't find a planet by id: {}", planet_id),
            })
    }

    pub async fn create_planet(&self, planet: Planet) -> CustomResult<Planet> {
        let collection = self.get_planets_collection();

        let insert_result = collection.insert_one(planet, None).await?;

        let filter = doc! { "_id": &insert_result.inserted_id };
        collection
            .find_one(filter, None)
            .await?
            .ok_or(CustomError::NotFound {
                message: String::from("Can't find created planet"),
            })
    }

    pub async fn delete_planet(&self, planet_id: ObjectId) -> CustomResult<()> {
        let collection = self.get_planets_collection();

        let filter = doc! { "_id": &planet_id };

        collection
            .find_one_and_delete(filter, None)
            .await?
            .ok_or(CustomError::NotFound {
                message: format!("Can't find planet by id: {}", planet_id),
            })?;

        Ok(())
    }

    pub async fn update_planet(&self, planet_id: ObjectId, planet: Planet) -> CustomResult<Planet> {
        let collection = self.get_planets_collection();

        let query = doc! { "_id": &planet_id };
        let update = doc! { "&set": Document::from(&planet) };
        let _ = collection.update_one(query, update, None).await?;

        let filter = doc! { "_id": &planet_id };
        let planet = collection
            .find_one(filter, None)
            .await?
            .ok_or(CustomError::NotFound {
                message: format!("Can't find updated planet: {}", planet_id),
            })?;

        Ok(planet)
    }
}

#[derive(RustEmbed)]
#[folder = "images"]
struct Asset;

pub fn get_planet_image(planet_name: &str) -> Vec<u8> {
    let filename = format!("{}.jpg", planet_name.to_lowercase());

    let image = Asset::get(&filename).expect("Failed to open image");

    image.data.to_vec()
}
