//! Connection-related data structures.

use derive_more::{AsRef, Deref, DerefMut};
use serde::{Deserialize, Serialize};
use url::{ParseError, Url};
use utoipa::ToSchema;

/// Unique identifier for a connection.
#[derive(Serialize, Deserialize, PartialEq, Debug, ToSchema, Deref, AsRef, DerefMut)]
#[as_ref(forward)]
pub struct ConnectionId(String);

impl ConnectionId {
    /// Creates a new connection ID from a string.
    pub fn new(id: impl AsRef<str>) -> Self {
        Self(id.as_ref().to_string())
    }
}

/// Human-readable name for a connection.
#[derive(Serialize, Deserialize, PartialEq, Debug, ToSchema, Deref, DerefMut, AsRef)]
#[as_ref(forward)]
pub struct ConnectionName(String);

impl ConnectionName {
    /// Creates a new connection name from a string.
    pub fn new(id: impl AsRef<str>) -> Self {
        Self(id.as_ref().to_string())
    }
}

/// Unique identifier for an organization.
#[derive(Serialize, Deserialize, PartialEq, Debug, ToSchema, Deref, DerefMut, AsRef)]
#[as_ref(forward)]
pub struct OrganizationId(String);

impl OrganizationId {
    /// Creates a new organization ID from a string.
    pub fn new(id: impl AsRef<str>) -> Self {
        Self(id.as_ref().to_string())
    }
}

/// URL for an organization's website.
#[derive(Serialize, Deserialize, PartialEq, Debug, ToSchema, Deref, DerefMut, AsRef)]
#[as_ref(forward)]
#[schema(value_type = String)]
pub struct OrganizationUrl(Url);

impl OrganizationUrl {
    /// Creates a new organization URL from a string.
    ///
    /// # Errors
    ///
    /// Returns an error if the string is not a valid URL.
    pub fn new(id: impl AsRef<str>) -> Result<Self, ParseError> {
        Ok(Self(Url::parse(id.as_ref())?))
    }
}

/// URL for a SimpleFin connection endpoint.
#[derive(Serialize, Deserialize, PartialEq, Debug, ToSchema, Deref, DerefMut, AsRef)]
#[as_ref(forward)]
#[schema(value_type = String)]
pub struct SimplefinUrl(Url);

impl SimplefinUrl {
    /// Creates a new SimpleFin URL from a string.
    ///
    /// # Errors
    ///
    /// Returns an error if the string is not a valid URL.
    pub fn new(id: impl AsRef<str>) -> Result<Self, ParseError> {
        Ok(Self(Url::parse(id.as_ref())?))
    }
}

/// Represents a SimpleFin connection to a financial institution.
///
/// See the [crate-level documentation](crate) for usage examples.
#[derive(Serialize, Deserialize, PartialEq, Debug, ToSchema)]
pub struct Connection {
    /// Unique identifier for this connection.
    #[serde(rename = "conn_id")]
    pub connection_id: ConnectionId,
    /// Human-readable connection name.
    pub name: ConnectionName,
    /// Organization (financial institution) ID.
    #[serde(rename = "org_id")]
    pub organization_id: OrganizationId,
    /// Optional URL for the organization's website.
    #[serde(rename = "org_url", skip_serializing_if = "Option::is_none")]
    pub organization_url: Option<OrganizationUrl>,
    /// SimpleFin API URL for this connection.
    #[serde(rename = "sfin_url")]
    pub simplefin_url: SimplefinUrl,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
    r#"{
  "conn_id": "CON-923049234-203940293409234",
  "name": "My Bank - Jill",
  "org_id": "ORG-8293948-230482398492834",
  "org_url": "https://mybank.com/",
  "sfin_url": "https://sfin.mybank.com/"
}"#,
    Connection {
            connection_id: ConnectionId::new("CON-923049234-203940293409234"),
            name: ConnectionName::new("My Bank - Jill"),
            organization_id: OrganizationId::new("ORG-8293948-230482398492834"),
            organization_url: Some(OrganizationUrl::new("https://mybank.com").unwrap()),
            simplefin_url: SimplefinUrl::new("https://sfin.mybank.com").unwrap(),
        }
    )]
    fn test_examples(#[case] input: &str, #[case] expected: Connection) {
        let deserialized: Connection = serde_json::from_str(input).unwrap();
        assert_eq!(deserialized, expected);

        let serialized = serde_json::to_string_pretty(&deserialized).unwrap();
        assert_eq!(serialized, input);
    }
}
