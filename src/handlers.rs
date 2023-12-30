use std::sync::Arc;

use askama::Template;
use axum::{
    extract::{Path, Query, State},
    http::header,
    response::{Html, IntoResponse},
    Json,
};
use serde::Deserialize;

use crate::{
    dto::PlanetDto,
    error::CustomResult,
    model::{Planet, PlanetType},
    services::{AppState, basic_auth::BasicAuth},
};

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;

pub async fn index() -> CustomResult<Html<String>> {
    let template = IndexTemplate;
    let result = template.render()?;

    Ok(Html(result))
}

#[derive(Deserialize)]
pub struct PlanetTypeQueryParam {
    r#type: Option<PlanetType>,
}

pub async fn get_planets(
    Query(param): Query<PlanetTypeQueryParam>,
    State(state): State<Arc<AppState>>,
) -> CustomResult<Json<Vec<PlanetDto>>> {
    let planets = state.planet_service.get_planets(param.r#type).await?;

    let result = planets.into_iter().map(PlanetDto::from).collect::<Vec<_>>();

    Ok(Json(result))
}

pub async fn get_planet(
    Path(planet_id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> CustomResult<Json<PlanetDto>> {
    let result = state.planet_service.get_planet(&planet_id).await?.into();

    Ok(Json(result))
}

pub async fn create_planet(
    State(state): State<Arc<AppState>>,
    _auth: BasicAuth,
    Json(planet_dto): Json<PlanetDto>,
) -> CustomResult<Json<PlanetDto>> {
    let planet = Planet::from(planet_dto);

    let result = state.planet_service.create_planet(planet).await?.into();

    Ok(Json(result))
}

pub async fn update_planet(
    State(state): State<Arc<AppState>>,
    Path(planet_id): Path<String>,
    _auth: BasicAuth,
    Json(planet_dto): Json<PlanetDto>,
) -> CustomResult<Json<PlanetDto>> {
    let planet = Planet::from(planet_dto);

    let result = state
        .planet_service
        .update_planet(&planet_id, planet)
        .await?
        .into();

    Ok(Json(result))
}

pub async fn delete_planet(
    Path(planet_id): Path<String>,
    State(state): State<Arc<AppState>>,
    _auth: BasicAuth,
) -> CustomResult<()> {
    state.planet_service.delete_planet(&planet_id).await?;

    Ok(())
}

pub async fn get_image_of_planet(
    Path(planet_id): Path<String>,
    State(state): State<Arc<AppState>>,
) -> CustomResult<impl IntoResponse> {
    let result = (
        [(header::CONTENT_TYPE, mime::IMAGE_JPEG.as_ref())],
        state.planet_service.get_planet_image(&planet_id).await?,
    );

    Ok(result)
}
