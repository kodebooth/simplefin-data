//! # SimpleFIN Server
//!
//! A Rust implementation of a SimpleFIN protocol server using [Axum](https://github.com/tokio-rs/axum).
//! This crate provides the web routing infrastructure and trait definitions needed to implement
//! a SimpleFIN v2 compliant server.
//!
//! ## Overview
//!
//! SimpleFIN is a protocol for accessing financial data from banks and other financial institutions.
//! This server library provides:
//!
//! - **[`Server`] trait**: Core trait for implementing SimpleFIN server logic
//! - **HTTP routes**: Pre-configured Axum routes for all SimpleFIN v2 endpoints
//! - **Type-safe handlers**: Route handlers with automatic JSON serialization/deserialization
//! - **State management**: [`ServerState`] wrapper for sharing server instances across routes
//!
//! ## Quick Start
//!
//! ### Implementing a SimpleFIN Server
//!
//! ```rust,no_run
//! use async_trait::async_trait;
//! use axum::http::Uri;
//! use simplefin_data::{
//!     accountset::{AccountSet, AccountsQuery},
//!     error::ServerError,
//!     token::Token,
//!     version::Version,
//! };
//! use simplefin_server::{Server, ServerState, router};
//! use url::Url;
//! use std::sync::Arc;
//!
//! // Implement the Server trait with your business logic
//! struct MySimpleFinServer;
//!
//! #[async_trait]
//! impl Server for MySimpleFinServer {
//!     async fn versions(&self) -> Vec<Version> {
//!         vec![Version::V1, Version::V2]
//!     }
//!
//!     async fn create_redirect(&self) -> Uri {
//!         // Return a redirect URI for the claim flow
//!         Uri::from_static("https://mybank.com/simplefin/authorize")
//!     }
//!
//!     async fn claim_token(&self, token: Token) -> Result<Url, ServerError> {
//!         // Validate the token and return a SimpleFIN access URL
//!         // Return Err(ServerError::Forbidden) for invalid tokens
//!         Ok(Url::parse("https://user:pass@api.mybank.com/simplefin/accounts").unwrap())
//!     }
//!
//!     async fn get_accounts(&self, query: AccountsQuery) -> Result<AccountSet, ServerError> {
//!         // Fetch account data based on query parameters
//!         // Return account set with connections, accounts, and transactions
//!         Ok(AccountSet::default())
//!     }
//! }
//!
//! #[tokio::main]
//! async fn main() {
//!     // Create server instance
//!     let server = Arc::new(MySimpleFinServer);
//!     let state = ServerState::new(server);
//!
//!     // Create router with all SimpleFIN routes
//!     let app = router().with_state(state);
//!
//!     // Start the server
//!     let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
//!         .await
//!         .unwrap();
//!     axum::serve(listener, app).await.unwrap();
//! }
//! ```
//!
//! ## Available Routes
//!
//! The [`router()`] function provides the following endpoints:
//!
//! - **`GET /info`**: Returns supported protocol versions
//! - **`GET /create`**: Initiates the claim flow, returns redirect URI
//! - **`POST /claim/{token}`**: Claims a token, returns SimpleFIN access URL
//! - **`GET /accounts`**: Returns account data with optional query parameters
//!
//! ## Query Parameters
//!
//! The `/accounts` endpoint supports the following query parameters:
//!
//! - `start_date`: Filter transactions on or after this date (Unix timestamp)
//! - `end_date`: Filter transactions on or before this date (Unix timestamp)
//! - `pending`: Include pending transactions (boolean)
//! - `account_id`: Return only specific account
//! - `balances_only`: Return only balances, omit transactions (boolean)
//! - `version`: Request specific protocol version
//!
//! ## Error Handling
//!
//! Server methods return [`ServerError`] which automatically maps to HTTP status codes:
//!
//! - `ServerError::PaymentRequired` → HTTP 402
//! - `ServerError::Forbidden` → HTTP 403

use std::{ops::Deref, sync::Arc};

use async_trait::async_trait;
use axum::{
    Router,
    http::Uri,
    routing::{get, post},
};
use simplefin_data::{
    accountset::{AccountSet, AccountsQuery},
    error::ServerError,
    token::Token,
    version::Version,
};
use url::Url;

pub mod routes;

/// Core trait for implementing SimpleFIN server functionality.
///
/// Implement this trait to provide the business logic for your SimpleFIN server.
/// Each method corresponds to a SimpleFIN protocol endpoint or operation.
///
/// # Examples
///
/// ```
/// use async_trait::async_trait;
/// use axum::http::Uri;
/// use simplefin_data::{
///     accountset::{AccountSet, AccountsQuery},
///     error::ServerError,
///     token::Token,
///     version::Version,
/// };
/// use simplefin_server::Server;
/// use url::Url;
///
/// struct MyServer;
///
/// #[async_trait]
/// impl Server for MyServer {
///     async fn versions(&self) -> Vec<Version> {
///         vec![Version::V2]
///     }
///
///     async fn create_redirect(&self) -> Uri {
///         Uri::from_static("https://example.com/authorize")
///     }
///
///     async fn claim_token(&self, token: Token) -> Result<Url, ServerError> {
///         // Validate and exchange token for access URL
///         Ok(Url::parse("https://user:pass@api.example.com/accounts").unwrap())
///     }
///
///     async fn get_accounts(&self, _query: AccountsQuery) -> Result<AccountSet, ServerError> {
///         Ok(AccountSet::default())
///     }
/// }
/// ```
#[async_trait]
pub trait Server: Send + Sync {
    /// Returns the list of SimpleFIN protocol versions this server supports.
    ///
    /// The default implementation returns `[Version::V2]`.
    async fn versions(&self) -> Vec<Version> {
        vec![Version::V2]
    }

    /// Creates a redirect URI for the SimpleFIN claim flow.
    ///
    /// This URI directs users to your authorization page where they can
    /// approve access and receive a claim token.
    async fn create_redirect(&self) -> Uri;

    /// Claims a token and returns a SimpleFIN access URL.
    ///
    /// Validates the provided token and returns a URL with embedded credentials
    /// pointing to your `/accounts` endpoint.
    ///
    /// Returns `Err(ServerError::Forbidden)` if the token is invalid or already claimed.
    async fn claim_token(&self, token: Token) -> Result<Url, ServerError>;

    /// Retrieves account data based on query parameters.
    ///
    /// Implements the core data retrieval for the SimpleFIN protocol, returning
    /// account information, connections, transactions, and any errors.
    async fn get_accounts(&self, query: AccountsQuery) -> Result<AccountSet, ServerError>;
}

/// Wrapper type for sharing [`Server`] implementations across Axum routes.
///
/// This type implements `Clone` and `Deref<Target = Arc<dyn Server>>`, making it
/// suitable for use as Axum state.
///
/// # Examples
///
/// ```
/// # use simplefin_server::{Server, ServerState};
/// # use std::sync::Arc;
/// # use async_trait::async_trait;
/// # struct MyServer;
/// # #[async_trait]
/// # impl Server for MyServer {
/// #     async fn versions(&self) -> Vec<simplefin_data::version::Version> { vec![] }
/// #     async fn create_redirect(&self) -> axum::http::Uri { axum::http::Uri::from_static("") }
/// #     async fn claim_token(&self, _: simplefin_data::token::Token) -> Result<url::Url, simplefin_data::error::ServerError> { unimplemented!() }
/// #     async fn get_accounts(&self, _: simplefin_data::accountset::AccountsQuery) -> Result<simplefin_data::accountset::AccountSet, simplefin_data::error::ServerError> { unimplemented!() }
/// # }
/// let server = Arc::new(MyServer);
/// let state = ServerState::new(server);
/// // state can now be cloned and used across routes
/// ```
#[derive(Clone)]
pub struct ServerState(Arc<dyn Server>);

impl ServerState {
    /// Creates a new `ServerState` wrapping the given server implementation.
    pub fn new(server: Arc<dyn Server>) -> Self {
        Self(server)
    }
}

impl Deref for ServerState {
    type Target = Arc<dyn Server>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Creates a router with all SimpleFIN protocol endpoints.
///
/// Returns an Axum [`Router`] configured with the following routes:
///
/// - `GET /info` - Returns supported protocol versions
/// - `GET /create` - Initiates claim flow, returns redirect URI
/// - `POST /claim/{token}` - Claims a token, returns access URL
/// - `GET /accounts` - Returns account data
///
/// # Examples
///
/// ```
/// # use simplefin_server::{Server, ServerState, router};
/// # use std::sync::Arc;
/// # use async_trait::async_trait;
/// # use axum::Router;
/// # struct MyServer;
/// # #[async_trait]
/// # impl Server for MyServer {
/// #     async fn versions(&self) -> Vec<simplefin_data::version::Version> { vec![] }
/// #     async fn create_redirect(&self) -> axum::http::Uri { axum::http::Uri::from_static("") }
/// #     async fn claim_token(&self, _: simplefin_data::token::Token) -> Result<url::Url, simplefin_data::error::ServerError> { unimplemented!() }
/// #     async fn get_accounts(&self, _: simplefin_data::accountset::AccountsQuery) -> Result<simplefin_data::accountset::AccountSet, simplefin_data::error::ServerError> { unimplemented!() }
/// # }
/// let server = Arc::new(MyServer);
/// let state = ServerState::new(server);
/// let _app: Router = router().with_state(state);
/// ```
pub fn router() -> Router<ServerState> {
    Router::new()
        .route("/info", get(routes::info::get_info))
        .route("/create", get(routes::create::get_create))
        .route("/claim/{token}", post(routes::claim::post_claim))
        .route("/accounts", get(routes::accounts::get_accounts))
}
