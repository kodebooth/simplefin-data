//! Transaction-related data structures.

use crate::serde::{
    deserialize_date, deserialize_date_option, deserialize_f32_str, serialize_date,
    serialize_date_option, serialize_f32_str,
};

use chrono::{DateTime, Utc};
use derive_more::{AsRef, Deref, DerefMut};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Unique identifier for a transaction.
#[derive(Serialize, Deserialize, PartialEq, Debug, ToSchema, Deref, AsRef, DerefMut)]
#[as_ref(forward)]
pub struct TransactionId(String);

impl TransactionId {
    /// Creates a new transaction ID from a string.
    pub fn new(id: impl AsRef<str>) -> Self {
        Self(id.as_ref().to_string())
    }
}

/// Represents a financial transaction.
///
/// Transactions represent financial activity on an account, with amounts
/// serialized as strings to maintain precision.
/// See the [crate-level documentation](crate) for usage examples.
#[derive(Serialize, Deserialize, PartialEq, Debug, ToSchema)]
pub struct Transaction<ExtraT = ()>
where
    ExtraT: ToSchema,
{
    /// Unique identifier for this transaction.
    #[serde(rename = "id")]
    pub transaction_id: TransactionId,
    /// Date when the transaction was posted to the account.
    #[serde(
        serialize_with = "serialize_date",
        deserialize_with = "deserialize_date"
    )]
    #[schema(value_type = i64)]
    pub posted: DateTime<Utc>,
    /// Transaction amount (positive for credits, negative for debits).
    #[serde(
        serialize_with = "serialize_f32_str",
        deserialize_with = "deserialize_f32_str"
    )]
    pub amount: f32,
    /// Human-readable transaction description.
    pub description: String,
    /// Date when the transaction actually occurred (may differ from posted date).
    #[serde(
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_date_option",
        deserialize_with = "deserialize_date_option",
        default
    )]
    #[schema(value_type = i64)]
    pub transacted_at: Option<DateTime<Utc>>,
    /// Whether the transaction is still pending (not yet cleared).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending: Option<bool>,
    /// Additional custom fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<ExtraT>,
}

#[cfg(test)]
mod tests {

    use super::*;
    use rstest::rstest;

    #[derive(Serialize, Deserialize, Debug, PartialEq, ToSchema)]
    struct Extra {
        pub category: String,
    }

    #[rstest]
    #[case(
        r#"{
  "id": "12394832938403",
  "posted": 793090572,
  "amount": "-33293.43",
  "description": "Uncle Frank's Bait Shop",
  "pending": true,
  "extra": {
    "category": "food"
  }
}"#,
        Transaction::<_> {
            transaction_id: TransactionId::new("12394832938403"),
            posted: DateTime::from_timestamp_secs(793090572).unwrap(),
            amount: -33293.43,
            description: "Uncle Frank's Bait Shop".to_string(),
            transacted_at: None,
            pending: Some(true),
            extra: Some(Extra {
                category: "food".to_string()
            })
        }
    )]
    fn test_examples(#[case] input: &str, #[case] expected: Transaction<Extra>) {
        let deserialized: Transaction<_> = serde_json::from_str(input).unwrap();
        assert_eq!(deserialized, expected);

        let serialized = serde_json::to_string_pretty(&deserialized).unwrap();
        assert_eq!(serialized, input);
    }
}
