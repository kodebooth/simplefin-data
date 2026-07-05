//! Account-related data structures.

use std::{collections::HashMap, ops::Deref};

use crate::{deserialize_date, serialize_date};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use url::Url;

use crate::{connection::ConnectionId, transaction::Transaction};

/// Unique identifier for an account.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct AccountId(String);

impl AccountId {
    /// Creates a new account ID from a string.
    pub fn new(id: impl AsRef<str>) -> Self {
        Self(id.as_ref().to_string())
    }
}

impl Deref for AccountId {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Human-readable name for an account.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct AccountName(String);

impl AccountName {
    /// Creates a new account name from a string.
    pub fn new(id: impl AsRef<str>) -> Self {
        Self(id.as_ref().to_string())
    }
}

impl Deref for AccountName {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Currency type, either an official currency code or a custom URL.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(untagged)]
pub enum Currency {
    /// Custom currency identified by a URL.
    Custom(Url),
    /// Official currency code (e.g., "USD", "EUR").
    Official(String),
}

/// Represents a financial account with balance and transaction history.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Account<O = String> {
    /// Unique identifier for this account.
    #[serde(rename = "id")]
    pub account_id: AccountId,
    /// Human-readable account name.
    pub name: AccountName,
    /// Connection ID this account belongs to.
    #[serde(rename = "conn_id")]
    pub connection_id: ConnectionId,
    /// Currency type for this account.
    pub currency: Currency,
    /// Current account balance.
    pub balance: f32,
    /// Available balance (if different from current balance).
    #[serde(rename = "available-balance", skip_serializing_if = "Option::is_none")]
    pub available_balance: Option<f32>,
    /// Date when the balance was last updated.
    #[serde(
        serialize_with = "serialize_date",
        deserialize_with = "deserialize_date"
    )]
    #[serde(rename = "balance-date")]
    pub balance_date: DateTime<Utc>,
    /// List of transactions for this account.
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub transactions: Vec<Transaction>,
    /// Additional custom fields.
    #[serde(skip_serializing_if = "HashMap::is_empty", default)]
    pub extra: HashMap<String, O>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{Value, from_str, json};

    #[test]
    fn test() {
        let account: Account<String> = Account {
            account_id: AccountId::new("test_account_id"),
            name: AccountName::new("test_account_name"),
            connection_id: ConnectionId::new("test_connection_id"),
            currency: Currency::Custom(Url::parse("http://example.org").unwrap()),
            balance: 100.0,
            available_balance: Some(200.2),
            balance_date: DateTime::from_timestamp_secs(1000).unwrap(),
            transactions: vec![],
            extra: HashMap::new(),
        };

        assert_eq!(
            from_str::<Value>(&serde_json::to_string(&account).unwrap()).unwrap(),
            json!({
                "id": "test_account_id",
                "name": "test_account_name",
                "conn_id": "test_connection_id",
                "currency": "http://example.org/",
                "balance": 100.0,
                "available-balance": Some(200.2),
                "balance-date": 1000,
            })
        );
    }
}
