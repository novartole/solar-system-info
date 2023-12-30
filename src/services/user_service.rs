use crate::{db::MongoDbClient, error::CustomResult, model::User};

#[derive(Clone)]
pub struct UserService {
    mongodb_client: MongoDbClient,
}

impl UserService {
    pub fn new(mongodb_client: MongoDbClient) -> Self {
        Self { mongodb_client }
    }

    pub async fn get_user(&self, username: String) -> CustomResult<User> {
        self.mongodb_client.get_user(username).await
    }
}
