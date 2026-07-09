use std::str::FromStr;
use chrono::NaiveDateTime;
use serde::{ Deserialize, Deserializer };

pub(crate) fn parse_date<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: Deserializer<'de>,
{
    let string = String::deserialize(deserializer)?;
    let string = string.trim();

    NaiveDateTime::parse_from_str(string, "%m/%d/%Y %I:%M %p")
        .or_else(|_| NaiveDateTime::parse_from_str(string, "%m/%d/%Y %H:%M:%S"))
        .map_err(|_| serde::de::Error::custom(format!("could not parse date: {string:?}")))
}

pub(crate) fn parse_optional_date<'de, D>(deserializer: D) -> Result<Option<NaiveDateTime>, D::Error>
where
    D: Deserializer<'de>,
{
    let string = String::deserialize(deserializer)?;   
    if string.is_empty() || string.trim() == "Unknown" {
        return Ok(None);
    }

    NaiveDateTime::parse_from_str(&string, "%m/%d/%Y %I:%M %p")
        .map(Some)
        .map_err(serde::de::Error::custom)
}

pub(crate) fn parse_optional_number<'de, D, T>(deserializer: D) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr,
    T::Err: std::fmt::Display,
{
    let string = String::deserialize(deserializer)?;
    if string.is_empty() || string.trim() == "Unknown" {
        return Ok(None);
    }

    string.trim()
        .parse::<T>()
        .map(Some)
        .map_err(serde::de::Error::custom)
}

pub(crate) fn parse_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let string = String::deserialize(deserializer)?;
    string.trim().to_lowercase().parse::<bool>().map_err(serde::de::Error::custom)
}
