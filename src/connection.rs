//! Connection-related data structures.

use std::ops::Deref;

use serde::{Deserialize, Serialize};
use url::{ParseError, Url};

/// Unique identifier for a connection.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ConnectionId(String);

impl ConnectionId {
    /// Creates a new connection ID from a string.
    pub fn new(id: impl AsRef<str>) -> Self {
        Self(id.as_ref().to_string())
    }
}

impl Deref for ConnectionId {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Human-readable name for a connection.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct ConnectionName(String);

impl ConnectionName {
    /// Creates a new connection name from a string.
    pub fn new(id: impl AsRef<str>) -> Self {
        Self(id.as_ref().to_string())
    }
}

impl Deref for ConnectionName {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Unique identifier for an organization.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct OrganizationId(String);

impl OrganizationId {
    /// Creates a new organization ID from a string.
    pub fn new(id: impl AsRef<str>) -> Self {
        Self(id.as_ref().to_string())
    }
}

impl Deref for OrganizationId {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// URL for an organization's website.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
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

impl Deref for OrganizationUrl {
    type Target = Url;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// URL for a SimpleFin connection endpoint.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
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

impl Deref for SimplefinUrl {
    type Target = Url;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Represents a SimpleFin connection to a financial institution.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
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
    use serde_json::{Value, from_str, json};

    #[test]
    fn test() {
        let connection = Connection {
            connection_id: ConnectionId::new("test_connection_id"),
            name: ConnectionName::new("test_name"),
            organization_id: OrganizationId::new("test_organization_id"),
            organization_url: Some(OrganizationUrl::new("https://example.org").unwrap()),
            simplefin_url: SimplefinUrl::new("https://sfin.example.org").unwrap(),
        };

        assert_eq!(
            from_str::<Value>(&serde_json::to_string(&connection).unwrap()).unwrap(),
            json!({
                "conn_id": "test_connection_id",
                "name": "test_name",
                "org_id": "test_organization_id",
                "org_url": "https://example.org/",
                "sfin_url": "https://sfin.example.org/",
            })
        );
    }
}
