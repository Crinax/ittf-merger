use std::{path::Path, fs::File, io::Read};

use super::ParserError;
use serde::{Serialize, Deserialize};
use quick_xml::de::from_str;

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Players {
    #[serde(rename = "Player", default)]
    players: Vec<Player>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct Player {
    #[serde(rename = "@id")]
    id: String,
    name: String,
    shortname: String,
    country: String,
    region: String,
    location: String,
    birthday: Option<u32>,
    sex: String,
    rating: f64,

    #[serde(rename = "deltaR", default)]
    delta: f64,
    place: u64,
    stats: String,
    stats2: String,
}

pub fn parse_players(path: String) -> Result<Players, ParserError> {
    if !Path::new(&path).exists() {
        return Err(ParserError::FileNotFound);
    }

    let mut file = File::open(&path).unwrap();
    let mut file_data = String::new();

    let result = file.read_to_string(&mut file_data);

    if result.is_err() {
        return Err(ParserError::ErrorWhileReading);
    }

    let parsing_result = from_str::<Players>(&file_data);

    if parsing_result.is_err() {
        return Err(ParserError::CannotParseFile);
    }

    Ok(parsing_result.unwrap())
}
