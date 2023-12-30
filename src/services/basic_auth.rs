use argon2::{Argon2, PasswordHash, PasswordVerifier};
use axum::{
    async_trait,
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use http_auth_basic::{AuthBasicError, Credentials};

use crate::{error::CustomError, model::AccessType};

use super::user_service::UserService;

pub struct BasicAuth;

#[async_trait]
impl<S> FromRequestParts<S> for BasicAuth
where
    UserService: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = CustomError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        // parse header
        let Credentials {
            user_id: username,
            password,
        } = parts
            .headers
            .get("Authorization")
            .and_then(|header| header.to_str().ok())
            .map_or_else(
                || Err(AuthBasicError::InvalidAuthorizationHeader),
                |auth_header| Credentials::from_header(auth_header.to_string()),
            )?;

        // get user
        let user = UserService::from_ref(state).get_user(username).await?;

        // verify password
        PasswordHash::new(&user.password).and_then(|parsed_hash| {
            Argon2::default().verify_password(password.as_bytes(), &parsed_hash)
        })?;

        // check access
        let (got_access, required_access) = (&user.access, &AccessType::ReadWrite);
        if got_access != required_access {
            return Err(CustomError::UserUnauthorized {
                message: format!(
                    "User doesn't have enough permissions: got {:?}, but requiered {:?}",
                    got_access, required_access
                ),
            });
        }

        Ok(Self)
    }
}
