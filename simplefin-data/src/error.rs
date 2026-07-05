//! Error handling and error code types.

use std::{fmt::Display, str::FromStr};

use serde::{Deserialize, Serialize};
use serde::{Deserializer, de};
use utoipa::ToSchema;

#[cfg(feature = "axum")]
use axum::{http::StatusCode, response::IntoResponse};

use crate::{account::AccountId, connection::ConnectionId};

/// Error returned when parsing an error code fails.
pub struct CodeParseError;

/// General error types that apply to the entire API.
#[derive(PartialEq, Debug, ToSchema)]
pub enum General {
    /// Generic API error.
    Api,
    /// Authentication error.
    Authentication,
}

impl Display for General {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            General::Api => write!(f, "api"),
            General::Authentication => write!(f, "auth"),
        }
    }
}

impl FromStr for General {
    type Err = CodeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "api" => Ok(General::Api),
            "auth" => Ok(General::Authentication),
            _ => Err(CodeParseError),
        }
    }
}

/// Connection-specific error types.
#[derive(PartialEq, Debug, ToSchema)]
pub enum Connection {
    /// Authentication error for a specific connection.
    Authentication,
}

impl Display for Connection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Connection::Authentication => write!(f, "auth"),
        }
    }
}

impl FromStr for Connection {
    type Err = CodeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "auth" => Ok(Connection::Authentication),
            _ => Err(CodeParseError),
        }
    }
}

/// Account-specific error types.
#[derive(PartialEq, Debug, ToSchema)]
pub enum Account {
    /// Account operation failed.
    Failed,
    /// Required account data is missing.
    MissingData,
}

impl Display for Account {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Account::Failed => write!(f, "failed"),
            Account::MissingData => write!(f, "missingdata"),
        }
    }
}

impl FromStr for Account {
    type Err = CodeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "failed" => Ok(Account::Failed),
            "missingdata" => Ok(Account::MissingData),
            _ => Err(CodeParseError),
        }
    }
}

/// Hierarchical error code indicating the category and specific type of error.
///
/// Error codes follow the format `prefix.subcode`, where the prefix indicates
/// the error category (gen, con, act) and the optional subcode provides specificity.
///
/// See [`Error`] for usage examples.
#[derive(PartialEq, Debug, ToSchema)]
pub enum Code {
    /// General error affecting the entire API.
    General(Option<General>),
    /// Connection-specific error.
    Connection(Option<Connection>),
    /// Account-specific error.
    Account(Option<Account>),
}

impl<'de> Deserialize<'de> for Code {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let code = String::deserialize(deserializer)?;
        let mut parts = code.split('.');

        let prefix = parts
            .next()
            .ok_or(de::Error::custom(format!("no prefix in: {}", code)))?;

        let subcode = parts.next();

        // There should be no more parts left. Only a single '.' is allowed.
        if subcode.is_some() && parts.next().is_some() {
            return Err(de::Error::custom(format!("invalid format: {}", code)));
        }

        let code = match prefix {
            "gen" => Code::General(match subcode {
                None => None,
                Some(subcode) => Some(
                    <General as FromStr>::from_str(subcode)
                        .map_err(|_| de::Error::custom(format!("unknown subcode: {}", subcode)))?,
                ),
            }),
            "con" => Code::Connection(match subcode {
                None => None,
                Some(subcode) => Some(
                    <Connection as FromStr>::from_str(subcode)
                        .map_err(|_| de::Error::custom(format!("unknown subcode: {}", subcode)))?,
                ),
            }),
            "act" => Code::Account(match subcode {
                None => None,
                Some(subcode) => Some(
                    <Account as FromStr>::from_str(subcode)
                        .map_err(|_| de::Error::custom(format!("unknown subcode: {}", subcode)))?,
                ),
            }),
            _ => Err(de::Error::custom(format!("unknown prefix: {}", prefix)))?,
        };

        Ok(code)
    }
}

impl Serialize for Code {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Code::General(Some(subcode)) => serializer.serialize_str(&format!("gen.{}", subcode)),
            Code::General(None) => serializer.serialize_str("gen."),
            Code::Connection(None) => serializer.serialize_str("con."),
            Code::Connection(Some(subcode)) => {
                serializer.serialize_str(&format!("con.{}", subcode))
            }
            Code::Account(None) => serializer.serialize_str("act."),
            Code::Account(Some(subcode)) => serializer.serialize_str(&format!("act.{}", subcode)),
        }
    }
}

/// Represents an error returned by the SimpleFin API.
///
/// Errors follow a hierarchical code structure with prefixes indicating the category:
/// - `gen`: General errors affecting the entire API
/// - `con`: Connection-specific errors
/// - `act`: Account-specific errors
///
/// See the [crate-level documentation](crate) for usage examples.
#[derive(Serialize, Deserialize, PartialEq, Debug, ToSchema)]
pub struct Error {
    /// Hierarchical error code indicating the type of error.
    pub code: Code,
    /// Human-readable error message.
    #[serde(rename = "msg")]
    pub message: String,
    /// Connection ID associated with this error (if applicable).
    #[serde(skip_serializing_if = "Option::is_none", rename = "conn_id")]
    pub connection_id: Option<ConnectionId>,
    /// Account ID associated with this error (if applicable).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<AccountId>,
}

/// Server-side errors for SimpleFIN operations.
///
/// These errors map to HTTP status codes when using the `axum` feature.
/// Note: This is a different Error type from the API `Error` above.
pub enum ServerError {
    /// Payment is required to access this resource (HTTP 402)
    PaymentRequired,
    /// Access is forbidden, typically due to invalid credentials (HTTP 403)
    Forbidden,
}

#[cfg(feature = "axum")]
impl IntoResponse for ServerError {
    fn into_response(self) -> axum::response::Response {
        match self {
            ServerError::PaymentRequired => StatusCode::PAYMENT_REQUIRED.into_response(),
            ServerError::Forbidden => StatusCode::FORBIDDEN.into_response(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        r#"{
  "code": "gen.auth",
  "msg": "No credentials provided"
}"#,
        Error {
            code: Code::General(Some(General::Authentication)),
            message: "No credentials provided".to_string(),
            connection_id: None,
            account_id: None,
        }
    )]
    #[case(
        r#"{
  "code": "con.auth",
  "msg": "Authentication failed for My Bank - Jim",
  "conn_id": "CON-21983498-29349823984293842"
}"#,
        Error {
            code: Code::Connection(Some(Connection::Authentication)),
            message: "Authentication failed for My Bank - Jim".to_string(),
            connection_id: Some(ConnectionId::new("CON-21983498-29349823984293842")),
            account_id: None,
        }
    )]
    #[case(
        r#"{
  "code": "act.failed",
  "msg": "Failed to get all transactions. Try again later.",
  "account_id": "ACT-1982398-12398192839182398123"
}"#,
        Error {
            code: Code::Account(Some(Account::Failed)),
            message: "Failed to get all transactions. Try again later.".to_string(),
            connection_id: None,
            account_id: Some(AccountId::new("ACT-1982398-12398192839182398123")),
        }

    )]
    fn test_examples(#[case] input: &str, #[case] expected: Error) {
        let deserialized: Error = serde_json::from_str(input).unwrap();
        assert_eq!(deserialized, expected);

        let serialized = serde_json::to_string_pretty(&deserialized).unwrap();
        assert_eq!(serialized, input);
    }
}
