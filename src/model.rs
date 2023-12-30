use std::{fmt, str::FromStr};

use chrono::Utc;
use mongodb::bson::{self, oid::ObjectId, Document};
use serde::{Deserialize, Serialize};

use crate::dto::{PlanetDto, SatelliteDto};

#[derive(Debug, Deserialize)]
pub struct User {
    pub password: String,
    pub access: AccessType,
}

#[derive(Debug, Deserialize, PartialEq)]
pub enum AccessType {
    None,
    ReadOnly,
    ReadWrite,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Planet {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub r#type: PlanetType,
    pub mean_radius: f32,
    pub satellites: Option<Vec<Satellite>>,
}

impl From<PlanetDto> for Planet {
    fn from(
        PlanetDto {
            id,
            name,
            r#type,
            mean_radius,
            satellites,
        }: PlanetDto,
    ) -> Self {
        let id = id.map(|id| ObjectId::from_str(id.as_str()).expect("Can't convert to ObjectId"));

        let satellites = satellites.map(|vec| vec.into_iter().map(Satellite::from).collect());

        Self {
            id,
            name,
            r#type,
            mean_radius,
            satellites,
        }
    }
}

impl From<&Planet> for Document {
    fn from(planet: &Planet) -> Self {
        bson::to_document(planet).expect("Can't convert planet to Document")
    }
}

#[derive(Copy, Clone, PartialEq, Serialize, Deserialize, Debug)]
pub enum PlanetType {
    TerrestrialPlanet,
    GasGiant,
    IceGiant,
    DwarfPlanet,
}

impl fmt::Display for PlanetType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Satellite {
    pub name: String,
    pub first_spacecraft_landing_date: Option<mongodb::bson::DateTime>,
}

impl From<SatelliteDto> for Satellite {
    fn from(
        SatelliteDto {
            name,
            first_spacecraft_landing_date,
        }: SatelliteDto,
    ) -> Self {
        let first_spacecraft_landing_date = first_spacecraft_landing_date.map(|nd| {
            mongodb::bson::DateTime::from_millis(
                chrono::DateTime::<Utc>::from_naive_utc_and_offset(
                    nd.and_hms_opt(0, 0, 0).unwrap(),
                    Utc,
                )
                .timestamp(),
            )
        });

        Self {
            name,
            first_spacecraft_landing_date,
        }
    }
}
