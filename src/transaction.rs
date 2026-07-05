//! Transaction-related data structures.

use std::{collections::HashMap, ops::Deref};

use crate::{deserialize_date, deserialize_date_option, serialize_date, serialize_date_option};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Unique identifier for a transaction.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct TransactionId(String);

impl TransactionId {
    /// Creates a new transaction ID from a string.
    pub fn new(id: impl AsRef<str>) -> Self {
        Self(id.as_ref().to_string())
    }
}

impl Deref for TransactionId {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Represents a financial transaction.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Transaction<O = String> {
    /// Unique identifier for this transaction.
    #[serde(rename = "id")]
    pub transaction_id: TransactionId,
    /// Date when the transaction was posted to the account.
    #[serde(
        serialize_with = "serialize_date",
        deserialize_with = "deserialize_date"
    )]
    pub posted: DateTime<Utc>,
    /// Transaction amount (positive for credits, negative for debits).
    pub amount: f32,
    /// Human-readable transaction description.
    pub description: String,
    /// Date when the transaction actually occurred (may differ from posted date).
    #[serde(
        serialize_with = "serialize_date_option",
        deserialize_with = "deserialize_date_option"
    )]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transacted_at: Option<DateTime<Utc>>,
    /// Whether the transaction is still pending (not yet cleared).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending: Option<bool>,
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
        let transaction: Transaction<String> = Transaction {
            transaction_id: TransactionId::new("test_transaction_id"),
            posted: DateTime::from_timestamp_secs(1000).unwrap(),
            amount: 100.2,
            description: "test_description".to_string(),
            transacted_at: Some(DateTime::from_timestamp_secs(1002).unwrap()),
            pending: Some(true),
            extra: HashMap::new(),
        };

        assert_eq!(
            from_str::<Value>(&serde_json::to_string(&transaction).unwrap()).unwrap(),
            json!({
                "id": "test_transaction_id",
                "posted": 1000,
                "amount": 100.2,
                "description": "test_description",
                "transacted_at": 1002,
                "pending": true,
            })
        );
    }
}
