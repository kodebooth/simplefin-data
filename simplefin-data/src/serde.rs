//! Internal serialization and deserialization helpers.
//!
//! This module contains helper functions for custom serde serialization/deserialization
//! of dates and numeric types used throughout the SimpleFIN data structures.

use chrono::{DateTime, Utc};
use serde::{
    Deserialize, Deserializer,
    de::{self},
};

/// Serializes a `DateTime<Utc>` as a Unix timestamp (seconds since epoch).
pub(crate) fn serialize_date<S>(date_time: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_i64(date_time.timestamp())
}

/// Serializes an `Option<DateTime<Utc>>` as a Unix timestamp.
pub(crate) fn serialize_date_option<S>(
    date_time: &Option<DateTime<Utc>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match date_time {
        Some(date_time) => Ok(serializer.serialize_i64(date_time.timestamp())?),
        None => serializer.serialize_none(),
    }
}

/// Deserializes a Unix timestamp (seconds since epoch) into a `DateTime<Utc>`.
///
/// # Errors
///
/// Returns an error if the timestamp is out of bounds.
pub(crate) fn deserialize_date<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let seconds = i64::deserialize(deserializer)?;

    DateTime::from_timestamp_secs(seconds).ok_or(de::Error::custom(format!(
        "out of bounds number of seconds: {}",
        seconds
    )))
}

/// Deserializes a Unix timestamp into an `Option<DateTime<Utc>>`.
///
/// # Errors
///
/// Returns an error if the timestamp is out of bounds.
pub(crate) fn deserialize_date_option<'de, D>(
    deserializer: D,
) -> Result<Option<DateTime<Utc>>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(Some(deserialize_date(deserializer)?))
}

/// Serializes an `Option<f32>` as a string.
pub(crate) fn serialize_f32_str_option<S>(
    value: &Option<f32>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match value {
        Some(value) => serialize_f32_str(value, serializer),
        None => serializer.serialize_none(),
    }
}

/// Serializes an `f32` as a string.
pub(crate) fn serialize_f32_str<S>(value: &f32, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&value.to_string())
}

/// Deserializes a string into an `f32`.
///
/// # Errors
///
/// Returns an error if the string cannot be parsed as an f32.
pub(crate) fn deserialize_f32_str<'de, D>(deserializer: D) -> Result<f32, D::Error>
where
    D: Deserializer<'de>,
{
    let value = String::deserialize(deserializer)?;
    let value = value
        .parse::<f32>()
        .map_err(|e| de::Error::custom(format!("failed conversion to f32: {}", e)))?;

    Ok(value)
}

/// Deserializes a string into an `Option<f32>`.
///
/// # Errors
///
/// Returns an error if the string cannot be parsed as an f32.
pub(crate) fn deserialize_f32_str_option<'de, D>(deserializer: D) -> Result<Option<f32>, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(Some(deserialize_f32_str(deserializer)?))
}
