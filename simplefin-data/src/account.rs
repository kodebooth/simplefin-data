//! Account-related data structures.

use crate::serde::{
    deserialize_date, deserialize_f32_str, deserialize_f32_str_option, serialize_date,
    serialize_f32_str, serialize_f32_str_option,
};

use chrono::{DateTime, Utc};
use derive_more::{AsRef, Deref, DerefMut};
use serde::{Deserialize, Serialize};
use url::Url;
use utoipa::ToSchema;

use crate::{connection::ConnectionId, transaction::Transaction};

/// Unique identifier for an account.
#[derive(Serialize, Deserialize, PartialEq, Debug, ToSchema, Deref, AsRef, DerefMut)]
#[as_ref(forward)]
pub struct AccountId(String);

impl AccountId {
    /// Creates a new account ID from a string.
    pub fn new(id: impl AsRef<str>) -> Self {
        Self(id.as_ref().to_string())
    }
}

/// Human-readable name for an account.
#[derive(Serialize, Deserialize, PartialEq, Debug, ToSchema, Deref, DerefMut, AsRef)]
#[as_ref(forward)]
pub struct AccountName(String);

impl AccountName {
    /// Creates a new account name from a string.
    pub fn new(id: impl AsRef<str>) -> Self {
        Self(id.as_ref().to_string())
    }
}

/// Currency type, either an official currency code or a custom URL.
///
/// Use [`Currency::new`] to automatically detect whether the input is a URL or currency code.
///
/// # Example
///
/// ```
/// use simplefin_data::account::Currency;
///
/// let usd = Currency::new("USD");
/// let custom = Currency::new("https://example.com/currency");
/// ```
#[derive(Serialize, Deserialize, PartialEq, Debug, ToSchema)]
#[serde(untagged)]
pub enum Currency {
    /// Custom currency identified by a URL.
    #[schema(value_type = String)]
    Custom(Url),
    /// Official currency code (e.g., "USD", "EUR").
    Official(String),
}

impl Currency {
    /// Creates a new Currency from a string.
    ///
    /// Automatically determines whether the input is a URL (Custom) or
    /// a currency code (Official).
    pub fn new(code: impl AsRef<str>) -> Self {
        match Url::parse(code.as_ref()) {
            Ok(url) => Currency::Custom(url),
            Err(_) => Currency::Official(code.as_ref().to_string()),
        }
    }
}

/// Represents a financial account with balance and transaction history.
///
/// Supports generic type parameters for custom extra fields on both the account
/// and its transactions. See the [crate-level documentation](crate) for usage examples.
///
/// # Example: Basic Account Construction
///
/// ```
/// use simplefin_data::account::{Account, AccountId, AccountName, Currency};
/// use simplefin_data::connection::ConnectionId;
/// use chrono::DateTime;
///
/// let account: Account = Account {
///     account_id: AccountId::new("acc_67890"),
///     name: AccountName::new("Checking Account"),
///     connection_id: ConnectionId::new("conn_123"),
///     currency: Currency::new("USD"),
///     balance: 1234.56,
///     available_balance: Some(1234.56),
///     balance_date: DateTime::from_timestamp_secs(1704067200).unwrap(),
///     transactions: vec![],
///     extra: None,
/// };
/// ```
#[derive(Serialize, Deserialize, PartialEq, Debug, ToSchema)]
pub struct Account<ExtraT = (), TransactionExtraT = ()>
where
    ExtraT: ToSchema,
    TransactionExtraT: ToSchema,
{
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
    #[serde(
        serialize_with = "serialize_f32_str",
        deserialize_with = "deserialize_f32_str"
    )]
    pub balance: f32,
    /// Available balance (if different from current balance).
    #[serde(
        rename = "available-balance",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_f32_str_option",
        deserialize_with = "deserialize_f32_str_option"
    )]
    pub available_balance: Option<f32>,
    /// Date when the balance was last updated.
    #[serde(
        serialize_with = "serialize_date",
        deserialize_with = "deserialize_date"
    )]
    #[serde(rename = "balance-date")]
    #[schema(value_type = i64)]
    pub balance_date: DateTime<Utc>,
    /// List of transactions for this account.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub transactions: Vec<Transaction<TransactionExtraT>>,
    /// Additional custom fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extra: Option<ExtraT>,
}

#[cfg(test)]
mod tests {

    use crate::transaction::TransactionId;

    use super::*;
    use rstest::rstest;

    #[derive(Debug, Serialize, Deserialize, PartialEq, ToSchema)]
    struct Extra {
        #[serde(
            serialize_with = "serialize_date",
            deserialize_with = "deserialize_date",
            rename = "account-open-date"
        )]
        #[schema(value_type = i64)]
        pub account_open_date: DateTime<Utc>,
    }

    #[rstest]
    #[case(
        r#"{
  "id": "2930002",
  "name": "Savings",
  "conn_id": "1238239482348382932",
  "currency": "USD",
  "balance": "100.23",
  "available-balance": "75.23",
  "balance-date": 978366153,
  "transactions": [
    {
      "id": "12394832938403",
      "posted": 793090572,
      "amount": "-33293.43",
      "description": "Uncle Frank's Bait Shop"
    }
  ],
  "extra": {
    "account-open-date": 978360153
  }
}"#,
        Account::<_> {
            account_id: AccountId::new("2930002"),
            name: AccountName::new("Savings"),
            connection_id: ConnectionId::new("1238239482348382932"),
            currency: Currency::new("USD"),
            balance: 100.23,
            available_balance: Some(75.23),
            balance_date: DateTime::from_timestamp_secs(978366153).unwrap(),
            transactions: vec![
                Transaction {
                    transaction_id: TransactionId::new("12394832938403"),
                    posted: DateTime::from_timestamp_secs(793090572).unwrap(),
                    amount: -33293.43,
                    description: "Uncle Frank's Bait Shop".to_string(),
                    transacted_at: None,
                    pending: None,
                    extra: None,
                }
            ],
            extra: Some(Extra {
                account_open_date: DateTime::from_timestamp_secs(978360153).unwrap(),
            })
        }
    )]
    fn test_examples(#[case] input: &str, #[case] expected: Account<Extra>) {
        let deserialized: Account<_> = serde_json::from_str(input).unwrap();
        assert_eq!(deserialized, expected);

        let serialized = serde_json::to_string_pretty(&deserialized).unwrap();
        assert_eq!(serialized, input);
    }
}
