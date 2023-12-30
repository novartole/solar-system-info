use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};

use crate::model::{Planet, PlanetType, Satellite};

#[derive(Serialize, Deserialize)]
pub struct PlanetDto {
    pub id: Option<String>,
    pub name: String,
    pub r#type: PlanetType,
    pub mean_radius: f32,
    pub satellites: Option<Vec<SatelliteDto>>,
}

impl From<Planet> for PlanetDto {
    fn from(
        Planet {
            id,
            name,
            r#type,
            mean_radius,
            satellites,
        }: Planet,
    ) -> Self {
        let id = id.map(|id| id.to_string());

        let satellites = satellites.map(|v| v.into_iter().map(|s| s.into()).collect());

        Self {
            id,
            name,
            r#type,
            mean_radius,
            satellites,
        }
    }
}

#[derive(Serialize)]
pub struct PlanetMessage {
    pub id: String,
    pub name: String,
    pub r#type: PlanetType,
}

impl From<&Planet> for PlanetMessage {
    fn from(
        Planet {
            id, name, r#type, ..
        }: &Planet,
    ) -> Self {
        PlanetMessage {
            id: id
                .map(|id| id.to_string())
                .expect("Planet.id is not specified"),
            name: name.clone(),
            r#type: *r#type,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SatelliteDto {
    pub name: String,
    pub first_spacecraft_landing_date: Option<NaiveDate>,
}

impl From<Satellite> for SatelliteDto {
    fn from(
        Satellite {
            name,
            first_spacecraft_landing_date,
        }: Satellite,
    ) -> Self {
        let first_spacecraft_landing_date = first_spacecraft_landing_date.map(|dt| {
            NaiveDateTime::from_timestamp_millis(dt.timestamp_millis())
                .unwrap()
                .date()
        });

        Self {
            name,
            first_spacecraft_landing_date,
        }
    }
}
