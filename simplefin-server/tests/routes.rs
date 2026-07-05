use async_trait::async_trait;
use axum::{
    Router,
    body::Body,
    http::{Request, StatusCode, Uri},
    routing::post,
};
use simplefin_data::{
    account::{Account, AccountId, AccountName, Currency},
    accountset::{AccountSet, AccountsQuery},
    connection::{Connection, ConnectionId, ConnectionName, OrganizationId, SimplefinUrl},
    error::ServerError,
    token::Token,
    transaction::{Transaction, TransactionId},
    version::Version,
};
use simplefin_server::{Server, ServerState, router, routes};
use std::sync::{Arc, Mutex};
use tower::ServiceExt;
use url::Url;

/// A dummy server implementation for testing
pub struct DummyServer {
    claimed_tokens: Arc<Mutex<Vec<String>>>,
}

impl DummyServer {
    pub fn new() -> Self {
        Self {
            claimed_tokens: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

impl Default for DummyServer {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Server for DummyServer {
    async fn versions(&self) -> Vec<Version> {
        vec![Version::V1, Version::V2]
    }

    async fn create_redirect(&self) -> Uri {
        Uri::from_static("https://example.com/redirect?token=test_redirect_token")
    }

    async fn claim_token(&self, token: Token) -> Result<Url, ServerError> {
        let token_str = format!("{:?}", token);

        let mut claimed = self.claimed_tokens.lock().unwrap();

        // Check if token was already claimed
        if claimed.contains(&token_str) {
            return Err(ServerError::Forbidden);
        }

        // Special test case: reject tokens containing "invalid"
        if token_str.contains("invalid") {
            return Err(ServerError::Forbidden);
        }

        // Mark token as claimed
        claimed.push(token_str.clone());

        // Return a SimpleFIN URL with credentials
        Ok(Url::parse("https://username:password@example.com/accounts").unwrap())
    }

    async fn get_accounts(&self, _query: AccountsQuery) -> Result<AccountSet, ServerError> {
        let connection = Connection {
            connection_id: ConnectionId::new("CON-TEST-001"),
            name: ConnectionName::new("Test Bank"),
            organization_id: OrganizationId::new("ORG-TEST-123"),
            organization_url: None,
            simplefin_url: SimplefinUrl::new("https://test.example.com").unwrap(),
        };

        let account = Account {
            account_id: AccountId::new("ACC-TEST-001"),
            name: AccountName::new("Test Account"),
            connection_id: ConnectionId::new("CON-TEST-001"),
            currency: Currency::new("USD"),
            balance: 1000.00,
            available_balance: Some(1000.00),
            balance_date: chrono::DateTime::parse_from_rfc3339("2024-01-01T00:00:00Z")
                .unwrap()
                .into(),
            transactions: vec![Transaction {
                transaction_id: TransactionId::new("TXN-TEST-001"),
                posted: chrono::DateTime::parse_from_rfc3339("2024-01-01T12:00:00Z")
                    .unwrap()
                    .into(),
                amount: 100.00,
                description: "Test Transaction".to_string(),
                transacted_at: None,
                pending: Some(false),
                extra: None,
            }],
            extra: None,
        };

        Ok(AccountSet {
            errlist: vec![],
            connections: vec![connection],
            accounts: vec![account],
            ..Default::default()
        })
    }
}

/// Helper function to create a test router with a DummyServer
pub fn create_test_router() -> Router {
    let server = Arc::new(DummyServer::new());
    let state = ServerState::new(server);

    router().with_state(state)
}

#[tokio::test]
async fn test_get_info() {
    let app = create_test_router();

    let response = app
        .oneshot(Request::builder().uri("/info").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(
        json,
        serde_json::json!({
            "versions": ["1", "2"]
        })
    );
}

#[tokio::test]
async fn test_get_create() {
    let app = create_test_router();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/create")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::SEE_OTHER);

    let location = response
        .headers()
        .get("location")
        .expect("Location header should be present")
        .to_str()
        .unwrap();

    assert_eq!(
        location,
        "https://example.com/redirect?token=test_redirect_token"
    );
}

#[tokio::test]
async fn test_post_claim_success() {
    let app = create_test_router();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/claim/valid_token_123")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let url = String::from_utf8(body.to_vec()).unwrap();

    assert_eq!(url, "https://username:password@example.com/accounts");
}

#[tokio::test]
async fn test_post_claim_invalid_token() {
    let app = create_test_router();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/claim/invalid_token")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_post_claim_duplicate_token() {
    let server = Arc::new(DummyServer::new());
    let state = ServerState::new(server);

    let app = Router::new()
        .route("/claim/{token}", post(routes::claim::post_claim))
        .with_state(state);

    // First claim should succeed
    let response1 = app
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/claim/duplicate_token")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response1.status(), StatusCode::OK);

    // Second claim of the same token should fail
    let response2 = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/claim/duplicate_token")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response2.status(), StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_get_accounts() {
    let app = create_test_router();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/accounts")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let account_set: AccountSet = serde_json::from_slice(&body).unwrap();

    assert_eq!(account_set.connections.len(), 1);
    assert_eq!(account_set.accounts.len(), 1);
    assert_eq!(&*account_set.accounts[0].account_id, "ACC-TEST-001");
}

#[tokio::test]
async fn test_route_not_found() {
    let app = create_test_router();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/nonexistent")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}
