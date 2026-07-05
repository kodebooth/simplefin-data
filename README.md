# SimpleFIN Data Structures

A Rust library providing type-safe data structures for the [SimpleFIN v2 protocol](https://www.simplefin.org/). This crate includes models for accounts, transactions, connections, and error handling, with full serialization/deserialization support via `serde`.

[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]
[![Build Status][actions-badge]][actions-url]

[crates-badge]: https://img.shields.io/crates/v/simplefin-data
[crates-url]: https://crates.io/crates/simplefin-data
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/kodebooth/simplefin/blob/main/LICENSE
[actions-badge]: https://github.com/kodebooth/simplefin/workflows/CI/badge.svg
[actions-url]: https://github.com/kodebooth/simplefin/actions?query=workflow%3ACI+branch%3Amain

## Overview

SimpleFIN is a protocol for accessing financial data from banks and other financial institutions. This library provides:

- **Type-safe wrappers** for identifiers (AccountId, ConnectionId, TransactionId, etc.)
- **Data structures** for accounts, transactions, and connections
- **Automatic serialization/deserialization** between Rust types and JSON
- **Date/time handling** with Unix timestamp conversion
- **Currency support** for both official codes (USD, EUR) and custom currencies

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
simplefin-data = "0.1.0"
```

## Quick Start

### Creating a Transaction

```rust
use simplefin_data::transaction::{Transaction, TransactionId};
use chrono::DateTime;
use std::collections::HashMap;

let transaction = Transaction {
    transaction_id: TransactionId::new("txn_12345"),
    posted: DateTime::from_timestamp_secs(1704067200).unwrap(), // 2024-01-01
    amount: -42.50, // Negative for debits
    description: "Coffee Shop Purchase".to_string(),
    transacted_at: Some(DateTime::from_timestamp_secs(1704067200).unwrap()),
    pending: Some(false),
    extra: HashMap::new(),
};

// Serialize to JSON
let json = serde_json::to_string(&transaction).unwrap();
println!("{}", json);
```

### Creating an Account

```rust
use simplefin_data::account::{Account, AccountId, AccountName, Currency};
use simplefin_data::connection::ConnectionId;
use simplefin_data::transaction::Transaction;
use chrono::DateTime;
use std::collections::HashMap;

let account: Account<String> = Account {
    account_id: AccountId::new("acc_67890"),
    name: AccountName::new("Checking Account"),
    connection_id: ConnectionId::new("conn_123"),
    currency: Currency::Official("USD".to_string()),
    balance: 1234.56,
    available_balance: Some(1234.56),
    balance_date: DateTime::from_timestamp_secs(1704067200).unwrap(),
    transactions: vec![],
    extra: HashMap::new(),
};

// Serialize to JSON
let json = serde_json::to_string_pretty(&account).unwrap();
println!("{}", json);
```

### Creating a Connection

```rust
use simplefin_data::connection::{
    Connection, ConnectionId, ConnectionName,
    OrganizationId, OrganizationUrl, SimplefinUrl
};

let connection = Connection {
    connection_id: ConnectionId::new("conn_bank_123"),
    name: ConnectionName::new("My Bank Account"),
    organization_id: OrganizationId::new("org_mybank"),
    organization_url: Some(OrganizationUrl::new("https://mybank.com").unwrap()),
    simplefin_url: SimplefinUrl::new("https://api.simplefin.org/accounts").unwrap(),
};

// Serialize to JSON
let json = serde_json::to_string_pretty(&connection).unwrap();
println!("{}", json);
```

### Deserializing from JSON

```rust
use simplefin_data::account::Account;

let json = r#"{
    "id": "acc_12345",
    "name": "Savings Account",
    "conn_id": "conn_123",
    "currency": "USD",
    "balance": 5000.00,
    "balance-date": 1704067200,
    "transactions": []
}"#;

let account: Account<String> = serde_json::from_str(json).unwrap();
println!("Account: {} has balance: {}", account.name.as_ref(), account.balance);
```

## Features

- **Type Safety**: Strong typing prevents mixing up different types of IDs
- **Deref Implementations**: Easy access to underlying string values via `Deref`
- **Serde Integration**: Seamless JSON serialization/deserialization
- **Optional Fields**: Proper handling of optional data with `skip_serializing_if`
- **Custom Currency**: Support for both standard currency codes and custom URL-based currencies
- **Extensible**: `extra` HashMap fields for custom data
