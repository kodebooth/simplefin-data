//! SimpleFin data structures and utilities.
//!
//! This crate provides data structures for working with SimpleFin API responses,
//! including accounts, transactions, connections, and error handling.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Deserializer, de};

pub mod account;
pub mod connection;
pub mod error;
pub mod transaction;

/// Serializes a `DateTime<Utc>` as a Unix timestamp (seconds since epoch).
pub(crate) fn serialize_date<S>(date_time: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_i64(date_time.timestamp())
}

/// Serializes an `Option<DateTime<Utc>>` as a Unix timestamp.
///
/// # Panics
///
/// Panics if the option is `None`.
pub(crate) fn serialize_date_option<S>(
    date_time: &Option<DateTime<Utc>>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match date_time {
        Some(date_time) => Ok(serializer.serialize_i64(date_time.timestamp())?),
        None => panic!(),
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
