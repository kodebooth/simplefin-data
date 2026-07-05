//! Protocol version types for SimpleFIN.

use serde::{Deserialize, Deserializer, Serialize, de};
use std::{fmt::Display, str::FromStr};
use utoipa::ToSchema;

/// Error parsing a version string.
#[derive(Debug)]
pub struct VersionParseError;

/// SimpleFIN protocol version.
///
/// SimpleFIN supports multiple protocol versions to maintain backward compatibility
/// while introducing new features. Clients and servers negotiate the version to use.
///
/// # Examples
///
/// ```
/// use simplefin_data::version::Version;
/// use std::str::FromStr;
///
/// let v1 = Version::from_str("1").unwrap();
/// assert_eq!(v1, Version::V1);
/// assert_eq!(v1.to_string(), "1");
/// ```
#[derive(PartialEq, Debug, Clone, Copy, ToSchema)]
pub enum Version {
    /// Protocol version 1
    V1,
    /// Protocol version 2
    V2,
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Version::V1 => write!(f, "1"),
            Version::V2 => write!(f, "2"),
        }
    }
}

impl FromStr for Version {
    type Err = VersionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Self::V1),
            "2" => Ok(Self::V2),
            _ => Err(VersionParseError),
        }
    }
}

impl Serialize for Version {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for Version {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let version = String::deserialize(deserializer)?;

        match version.as_str() {
            "1" => Ok(Version::V1),
            "2" => Ok(Version::V2),
            _ => Err(de::Error::custom(format!("unknown version: {}", version)))?,
        }
    }
}

/// List of supported protocol versions.
///
/// Used in the `/info` endpoint to advertise which protocol versions
/// a SimpleFIN server supports.
#[derive(Serialize, Deserialize, PartialEq, Debug, ToSchema)]
pub struct Versions {
    pub versions: Vec<Version>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
r#"{
  "versions": [
    "1",
    "2"
  ]
}"#,
    Versions {
        versions: vec![Version::V1, Version::V2]
    }
    )]
    fn test_versions(#[case] input: &str, #[case] expected: Versions) {
        let deserialized: Versions = serde_json::from_str(input).unwrap();
        assert_eq!(deserialized, expected);

        let serialized = serde_json::to_string_pretty(&deserialized).unwrap();
        assert_eq!(serialized, input);
    }
}
