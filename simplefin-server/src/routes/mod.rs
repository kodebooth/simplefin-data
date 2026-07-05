//! Route handlers for SimpleFIN protocol endpoints.
//!
//! This module contains the HTTP route handlers that implement the SimpleFIN v2 protocol.
//! Each submodule corresponds to one endpoint:
//!
//! - [`info`]: `GET /info` - Returns supported protocol versions
//! - [`create`]: `GET /create` - Initiates claim flow
//! - [`claim`]: `POST /claim/{token}` - Claims a token
//! - [`accounts`]: `GET /accounts` - Returns account data

pub mod accounts;
pub mod claim;
pub mod create;
pub mod info;
