# SimpleFIN Server

[![Crates.io][crates-badge]][crates-url]
[![MIT licensed][mit-badge]][mit-url]
[![Build Status][actions-badge]][actions-url]

[crates-badge]: https://img.shields.io/crates/v/simplefin-server
[crates-url]: https://crates.io/crates/simplefin-server
[mit-badge]: https://img.shields.io/badge/license-MIT-blue.svg
[mit-url]: https://github.com/kodebooth/simplefin/blob/main/LICENSE
[actions-badge]: https://github.com/kodebooth/simplefin/workflows/CI/badge.svg
[actions-url]: https://github.com/kodebooth/simplefin/actions?query=workflow%3ACI+branch%3Amain

## SimpleFIN Server

A Rust implementation of a SimpleFIN protocol server using [Axum](https://github.com/tokio-rs/axum).
This crate provides the web routing infrastructure and trait definitions needed to implement
a SimpleFIN v2 compliant server.

### Overview

SimpleFIN is a protocol for accessing financial data from banks and other financial institutions.
This server library provides:

- **[`Server`] trait**: Core trait for implementing SimpleFIN server logic
- **HTTP routes**: Pre-configured Axum routes for all SimpleFIN v2 endpoints
- **Type-safe handlers**: Route handlers with automatic JSON serialization/deserialization
- **State management**: [`ServerState`] wrapper for sharing server instances across routes

### Quick Start

#### Implementing a SimpleFIN Server

```rust
use async_trait::async_trait;
use axum::http::Uri;
use simplefin_data::{
    accountset::{AccountSet, AccountsQuery},
    error::ServerError,
    token::Token,
    version::Version,
};
use simplefin_server::{Server, ServerState, router};
use url::Url;
use std::sync::Arc;

// Implement the Server trait with your business logic
struct MySimpleFinServer;

#[async_trait]
impl Server for MySimpleFinServer {
    async fn versions(&self) -> Vec<Version> {
        vec![Version::V1, Version::V2]
    }

    async fn create_redirect(&self) -> Uri {
        // Return a redirect URI for the claim flow
        Uri::from_static("https://mybank.com/simplefin/authorize")
    }

    async fn claim_token(&self, token: Token) -> Result<Url, ServerError> {
        // Validate the token and return a SimpleFIN access URL
        // Return Err(ServerError::Forbidden) for invalid tokens
        Ok(Url::parse("https://user:pass@api.mybank.com/simplefin/accounts").unwrap())
    }

    async fn get_accounts(&self, query: AccountsQuery) -> Result<AccountSet, ServerError> {
        // Fetch account data based on query parameters
        // Return account set with connections, accounts, and transactions
        Ok(AccountSet::default())
    }
}

#[tokio::main]
async fn main() {
    // Create server instance
    let server = Arc::new(MySimpleFinServer);
    let state = ServerState::new(server);

    // Create router with all SimpleFIN routes
    let app = router().with_state(state);

    // Start the server
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
```

### Available Routes

The [`router()`] function provides the following endpoints:

- **`GET /info`**: Returns supported protocol versions
- **`GET /create`**: Initiates the claim flow, returns redirect URI
- **`POST /claim/{token}`**: Claims a token, returns SimpleFIN access URL
- **`GET /accounts`**: Returns account data with optional query parameters

### Query Parameters

The `/accounts` endpoint supports the following query parameters:

- `start_date`: Filter transactions on or after this date (Unix timestamp)
- `end_date`: Filter transactions on or before this date (Unix timestamp)
- `pending`: Include pending transactions (boolean)
- `account_id`: Return only specific account
- `balances_only`: Return only balances, omit transactions (boolean)
- `version`: Request specific protocol version

### Error Handling

Server methods return [`ServerError`] which automatically maps to HTTP status codes:

- `ServerError::PaymentRequired` → HTTP 402
- `ServerError::Forbidden` → HTTP 403
