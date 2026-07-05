use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    account::{Account, AccountId},
    connection::Connection,
    error::Error,
    serde::{deserialize_date_option, serialize_date_option},
    version::Version,
};

/// Represents a complete SimpleFIN API response containing accounts, connections, and errors.
///
/// This is the primary data structure returned by SimpleFIN v2 endpoints.
/// See the [crate-level documentation](crate) for usage examples.
#[derive(Serialize, Deserialize, PartialEq, Debug, ToSchema)]
#[serde(bound(
    serialize = "AccountExtraT: Serialize, TransactionExtraT: Serialize",
    deserialize = "AccountExtraT: Deserialize<'de>, TransactionExtraT: Deserialize<'de>"
))]
pub struct AccountSet<AccountExtraT = (), TransactionExtraT = ()>
where
    AccountExtraT: ToSchema,
    TransactionExtraT: ToSchema,
{
    pub errlist: Vec<Error>,
    #[deprecated = "Use errlist"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<String>,
    #[serde(default)]
    pub connections: Vec<Connection>,
    #[serde(default)]
    pub accounts: Vec<Account<AccountExtraT, TransactionExtraT>>,
}

impl<AccountExtraT, TransactionExtraT> Default for AccountSet<AccountExtraT, TransactionExtraT>
where
    AccountExtraT: ToSchema,
    TransactionExtraT: ToSchema,
{
    fn default() -> Self {
        Self {
            errlist: Vec::new(),
            #[allow(deprecated)]
            errors: None,
            connections: Vec::new(),
            accounts: Vec::new(),
        }
    }
}

/// Query parameters for the `/accounts` endpoint.
///
/// These parameters allow clients to filter and customize the account data
/// returned by the server. All fields are optional.
///
/// # Examples
///
/// ```
/// use simplefin_data::accountset::AccountsQuery;
/// use simplefin_data::account::AccountId;
/// use chrono::{DateTime, Utc};
///
/// // Request only balances without transactions
/// let query = AccountsQuery {
///     start_date: None,
///     end_date: None,
///     pending: None,
///     account_id: None,
///     balances_only: Some(()),
///     version: None,
/// };
///
/// // Request specific account with date range
/// let query = AccountsQuery {
///     start_date: Some(DateTime::from_timestamp_secs(1704067200).unwrap()),
///     end_date: Some(DateTime::from_timestamp_secs(1706745600).unwrap()),
///     pending: Some(()),
///     account_id: Some(AccountId::new("ACC-123")),
///     balances_only: Some(()),
///     version: None,
/// };
/// ```
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct AccountsQuery {
    /// Filter transactions to those on or after this date
    #[serde(
        rename = "start-date",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_date_option",
        deserialize_with = "deserialize_date_option",
        default
    )]
    pub start_date: Option<DateTime<Utc>>,
    /// Filter transactions to those on or before this date
    #[serde(
        rename = "end-date",
        skip_serializing_if = "Option::is_none",
        serialize_with = "serialize_date_option",
        deserialize_with = "deserialize_date_option",
        default
    )]
    pub end_date: Option<DateTime<Utc>>,
    /// Include pending transactions when true
    #[serde(skip_serializing_if = "Option::is_none", default)]
    //TODO: This needs to actually parse pending=1
    pub pending: Option<()>,
    /// Return only data for this specific account
    #[serde(
        rename = "account-id",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub account_id: Option<AccountId>,
    /// Return only balance information, omit all transactions
    #[serde(
        rename = "balances-only",
        skip_serializing_if = "Option::is_none",
        default
    )]
    //TODO: This needs to actually parse balances-only=1
    pub balances_only: Option<()>,
    /// Request a specific protocol version
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub version: Option<Version>,
}

#[cfg(test)]
mod tests {

    use crate::{
        account::{Account, AccountId, AccountName, Currency},
        connection::{ConnectionId, ConnectionName, OrganizationId, OrganizationUrl, SimplefinUrl},
        serde::{deserialize_date, serialize_date},
        transaction::{Transaction, TransactionId},
    };

    use super::*;
    use chrono::{DateTime, Utc};
    use rstest::rstest;

    #[derive(Debug, Serialize, Deserialize, PartialEq, ToSchema)]
    struct AccountExtra {
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
  "errlist": [],
  "connections": [
    {
      "conn_id": "CON-1122121298398234234",
      "name": "My Bank - Jill",
      "org_id": "INST-1298391823-129381928391823",
      "org_url": "https://mybank.com/",
      "sfin_url": "https://sfin.mybank.com/"
    }
  ],
  "accounts": [
    {
      "id": "2930002",
      "name": "Savings",
      "conn_id": "CON-1122121298398234234",
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
    }
  ]
}"#,
        AccountSet::<_, _> {
            errlist: vec![],
            connections: vec![
                Connection {
                    connection_id: ConnectionId::new("CON-1122121298398234234"),
                    name: ConnectionName::new("My Bank - Jill"),
                    organization_id: OrganizationId::new("INST-1298391823-129381928391823"),
                    organization_url: Some(OrganizationUrl::new("https://mybank.com").unwrap()),
                    simplefin_url: SimplefinUrl::new("https://sfin.mybank.com").unwrap(),
                },
            ],
            accounts: vec![
                Account {
                    account_id: AccountId::new("2930002"),
                    name: AccountName::new("Savings"),
                    connection_id: ConnectionId::new("CON-1122121298398234234"),
                    currency: Currency::new("USD"),
                    balance: 100.23,
                    available_balance: Some(75.23),
                    balance_date: DateTime::from_timestamp_secs(978366153).unwrap(),
                    transactions: vec![
                        Transaction {
                            transaction_id:TransactionId::new("12394832938403"),
                            posted: DateTime::from_timestamp_secs(793090572).unwrap(),
                            amount: -33293.43,
                            description: "Uncle Frank's Bait Shop".to_string(),
                            transacted_at: None,
                            pending: None,
                            extra: None,
                        }
                    ],
                    extra: Some(AccountExtra {
                        account_open_date: DateTime::from_timestamp_secs(978360153).unwrap(),
                    })
                }
            ],
            ..Default::default()
        }
    )]
    fn test_examples(#[case] input: &str, #[case] expected: AccountSet<AccountExtra>) {
        let deserialized: AccountSet<_, _> = serde_json::from_str(input).unwrap();
        assert_eq!(deserialized, expected);

        let serialized = serde_json::to_string_pretty(&deserialized).unwrap();
        assert_eq!(serialized, input);
    }
}
