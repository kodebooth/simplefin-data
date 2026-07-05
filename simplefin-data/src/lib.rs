//! # SimpleFIN Data Structures
//!
//! A Rust library providing type-safe data structures for the [SimpleFIN v2 protocol](https://www.simplefin.org/).
//! This crate includes models for accounts, transactions, connections, and error handling,
//! with full serialization/deserialization support via `serde`.
//!
//! ## Overview
//!
//! SimpleFIN is a protocol for accessing financial data from banks and other financial institutions.
//! This library provides:
//!
//! - **Type-safe wrappers** for identifiers (AccountId, ConnectionId, TransactionId, etc.)
//! - **Data structures** for accounts, transactions, connections, and complete account sets
//! - **Automatic serialization/deserialization** between Rust types and JSON, matching SimpleFIN v2 protocol format
//! - **Date/time handling** with Unix timestamp conversion
//! - **Currency support** for both official codes (USD, EUR) and custom currencies
//! - **Error handling** with hierarchical error codes (`gen`, `con`, `act`)
//! - **Generic extra fields** support for custom data on accounts and transactions
//!
//! ## Quick Start
//!
//! ### Deserializing a Complete AccountSet
//!
//! The most common use case is deserializing a complete API response containing accounts,
//! connections, and any errors:
//!
//! ```
//! use simplefin_data::accountset::AccountSet;
//!
//! let json = r#"{
//!   "errlist": [],
//!   "connections": [
//!     {
//!       "conn_id": "CON-1122121298398234234",
//!       "name": "My Bank - Jill",
//!       "org_id": "INST-1298391823-129381928391823",
//!       "org_url": "https://mybank.com/",
//!       "sfin_url": "https://sfin.mybank.com/"
//!     }
//!   ],
//!   "accounts": [
//!     {
//!       "id": "2930002",
//!       "name": "Savings",
//!       "conn_id": "CON-1122121298398234234",
//!       "currency": "USD",
//!       "balance": "100.23",
//!       "available-balance": "75.23",
//!       "balance-date": 978366153,
//!       "transactions": [
//!         {
//!           "id": "12394832938403",
//!           "posted": 793090572,
//!           "amount": "-33293.43",
//!           "description": "Uncle Frank's Bait Shop"
//!         }
//!       ]
//!     }
//!   ]
//! }"#;
//!
//! let account_set: AccountSet = serde_json::from_str(json).unwrap();
//! println!("Loaded {} accounts", account_set.accounts.len());
//!
//! // Process accounts
//! for account in &account_set.accounts {
//!     println!("Account: {}, Balance: {}", &*account.name, account.balance);
//! }
//!
//! // Check for errors
//! if !account_set.errlist.is_empty() {
//!     for error in &account_set.errlist {
//!         eprintln!("Error: {}", error.message);
//!     }
//! }
//! ```
//!
//! ### Using Generic Extra Fields
//!
//! Accounts and transactions support custom fields through generic type parameters:
//!
//! ```
//! use simplefin_data::account::{Account, AccountId, AccountName, Currency};
//! use simplefin_data::connection::ConnectionId;
//! use chrono::DateTime;
//! use serde::{Deserialize, Serialize};
//! use utoipa::ToSchema;
//!
//! #[derive(Serialize, Deserialize, Debug, ToSchema)]
//! struct AccountExtra {
//!     #[serde(rename = "account-open-date")]
//!     pub account_open_date: i64,
//!     pub branch_id: String,
//! }
//!
//! #[derive(Serialize, Deserialize, Debug, ToSchema)]
//! struct TransactionExtra {
//!     pub category: String,
//!     pub tags: Vec<String>,
//! }
//!
//! // Use the generic types
//! let account: Account<AccountExtra, TransactionExtra> = Account {
//!     account_id: AccountId::new("acc_67890"),
//!     name: AccountName::new("Checking Account"),
//!     connection_id: ConnectionId::new("conn_123"),
//!     currency: Currency::new("USD"),
//!     balance: 1234.56,
//!     available_balance: Some(1234.56),
//!     balance_date: DateTime::from_timestamp_secs(1704067200).unwrap(),
//!     transactions: vec![],
//!     extra: Some(AccountExtra {
//!         account_open_date: 1640000000,
//!         branch_id: "BR-001".to_string(),
//!     }),
//! };
//! ```
//!
//! ### Creating Individual Types
//!
//! #### Transaction
//!
//! ```
//! use simplefin_data::transaction::{Transaction, TransactionId};
//! use chrono::DateTime;
//!
//! let transaction: Transaction = Transaction {
//!     transaction_id: TransactionId::new("txn_12345"),
//!     posted: DateTime::from_timestamp_secs(1704067200).unwrap(),
//!     amount: -42.50, // Negative for debits
//!     description: "Coffee Shop Purchase".to_string(),
//!     transacted_at: Some(DateTime::from_timestamp_secs(1704067200).unwrap()),
//!     pending: Some(false),
//!     extra: None,
//! };
//!
//! // Serialize to JSON (amounts are encoded as strings)
//! let json = serde_json::to_string_pretty(&transaction).unwrap();
//! // Output: {"id":"txn_12345","posted":1704067200,"amount":"-42.5", ...}
//! ```
//!
//! #### Account
//!
//! ```
//! use simplefin_data::account::{Account, AccountId, AccountName, Currency};
//! use simplefin_data::connection::ConnectionId;
//! use chrono::DateTime;
//!
//! let account: Account = Account {
//!     account_id: AccountId::new("acc_67890"),
//!     name: AccountName::new("Checking Account"),
//!     connection_id: ConnectionId::new("conn_123"),
//!     currency: Currency::new("USD"),
//!     balance: 1234.56,
//!     available_balance: Some(1234.56),
//!     balance_date: DateTime::from_timestamp_secs(1704067200).unwrap(),
//!     transactions: vec![],
//!     extra: None,
//! };
//! ```
//!
//! #### Connection
//!
//! ```
//! use simplefin_data::connection::{
//!     Connection, ConnectionId, ConnectionName,
//!     OrganizationId, OrganizationUrl, SimplefinUrl
//! };
//!
//! let connection = Connection {
//!     connection_id: ConnectionId::new("conn_bank_123"),
//!     name: ConnectionName::new("My Bank Account"),
//!     organization_id: OrganizationId::new("org_mybank"),
//!     organization_url: Some(OrganizationUrl::new("https://mybank.com").unwrap()),
//!     simplefin_url: SimplefinUrl::new("https://api.simplefin.org/accounts").unwrap(),
//! };
//! ```
//!
//! ### Error Handling
//!
//! SimpleFIN errors follow a hierarchical code structure:
//!
//! ```
//! use simplefin_data::error::{Error, Code, Connection as ConnError};
//! use simplefin_data::connection::ConnectionId;
//!
//! let error = Error {
//!     code: Code::Connection(Some(ConnError::Authentication)),
//!     message: "Authentication failed for My Bank - Jim".to_string(),
//!     connection_id: Some(ConnectionId::new("CON-21983498-29349823984293842")),
//!     account_id: None,
//! };
//!
//! // Serializes to:
//! // {
//! //   "code": "con.auth",
//! //   "msg": "Authentication failed for My Bank - Jim",
//! //   "conn_id": "CON-21983498-29349823984293842"
//! // }
//! ```
//!
//! Error codes include:
//! - **General errors**: `gen.api`, `gen.auth`
//! - **Connection errors**: `con.auth`
//! - **Account errors**: `act.failed`, `act.missingdata`

pub mod account;
pub mod accountset;
pub mod connection;
pub mod error;
pub mod token;
pub mod transaction;
pub mod version;

mod serde;
