use std::sync::Arc;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::{IntoResponse, Redirect, Response},
};
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, CsrfToken, PkceCodeChallenge, PkceCodeVerifier,
    RedirectUrl, Scope, TokenResponse, TokenUrl, basic::BasicClient,
};
use serde::Deserialize;
use tower_sessions::Session;

use super::{OAuthConfig, jwks::JwksCache};

const SESSION_KEY_VERIFIER: &str = "pkce_verifier";
const SESSION_KEY_CSRF: &str = "pkce_state";
pub const SESSION_KEY_USER_ID: &str = "user_id";

#[derive(Clone)]
pub struct WebAuthState {
    pub oauth: Arc<OAuthConfig>,
    pub jwks: Arc<JwksCache>,
}

/// Thin async HTTP client adapter bridging oauth2 crate to reqwest 0.13
async fn async_http_client(
    request: oauth2::HttpRequest,
) -> Result<oauth2::HttpResponse, oauth2::HttpClientError<reqwest::Error>> {
    let client = reqwest::ClientBuilder::new()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .map_err(|e| oauth2::HttpClientError::Reqwest(Box::new(e)))?;

    let method = reqwest::Method::from_bytes(request.method().as_str().as_bytes())
        .map_err(|e| oauth2::HttpClientError::Other(e.to_string()))?;

    let mut req = client.request(method, request.uri().to_string());
    for (name, value) in request.headers() {
        req = req.header(name.as_str(), value.as_bytes());
    }
    req = req.body(request.body().to_vec());

    let resp = req
        .send()
        .await
        .map_err(|e| oauth2::HttpClientError::Reqwest(Box::new(e)))?;

    let status = resp.status().as_u16();
    let headers = resp.headers().clone();
    let body = resp
        .bytes()
        .await
        .map_err(|e| oauth2::HttpClientError::Reqwest(Box::new(e)))?;

    let mut builder = oauth2::http::Response::builder().status(status);
    for (name, value) in &headers {
        builder = builder.header(name.as_str(), value.as_bytes());
    }
    builder
        .body(body.to_vec())
        .map_err(oauth2::HttpClientError::Http)
}

#[tracing::instrument(skip(state, session))]
pub async fn login_handler(
    State(state): State<WebAuthState>,
    session: Session,
) -> Result<Response, StatusCode> {
    let (client_id, redirect_uri) = state
        .oauth
        .client_id
        .as_deref()
        .zip(state.oauth.redirect_uri.as_deref())
        .ok_or_else(|| {
            tracing::error!("OAUTH_CLIENT_ID is not set; web login is not configured");
            StatusCode::NOT_IMPLEMENTED
        })?;

    let auth_url = AuthUrl::new(state.oauth.authorization_endpoint.clone()).map_err(|e| {
        tracing::error!("invalid auth URL: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    let token_url = TokenUrl::new(state.oauth.token_endpoint.clone()).map_err(|e| {
        tracing::error!("invalid token URL: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    let redirect_url = RedirectUrl::new(redirect_uri.to_string()).map_err(|e| {
        tracing::error!("invalid redirect URI: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let client = BasicClient::new(ClientId::new(client_id.to_string()))
        .set_auth_uri(auth_url)
        .set_token_uri(token_url)
        .set_redirect_uri(redirect_url);

    let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

    let (auth_url, csrf_token) = client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("openid".to_string()))
        .add_scope(Scope::new("profile".to_string()))
        .add_scope(Scope::new("email".to_string()))
        .set_pkce_challenge(pkce_challenge)
        .url();

    session
        .insert(SESSION_KEY_VERIFIER, pkce_verifier.secret())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    session
        .insert(SESSION_KEY_CSRF, csrf_token.secret())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    tracing::debug!("redirecting to {auth_url}");
    Ok(Redirect::to(auth_url.as_str()).into_response())
}

#[derive(Deserialize, Debug)]
pub struct CallbackParams {
    code: String,
    state: String,
}

#[tracing::instrument(skip(state, session))]
pub async fn callback_handler(
    State(state): State<WebAuthState>,
    Query(params): Query<CallbackParams>,
    session: Session,
) -> Result<Response, StatusCode> {
    let (client_id, redirect_uri) = state
        .oauth
        .client_id
        .as_deref()
        .zip(state.oauth.redirect_uri.as_deref())
        .ok_or_else(|| {
            tracing::error!("OAUTH_CLIENT_ID is not set; web login is not configured");
            StatusCode::NOT_IMPLEMENTED
        })?;

    // Validate CSRF
    let saved_csrf: Option<String> = session
        .get(SESSION_KEY_CSRF)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    if saved_csrf.as_deref() != Some(params.state.as_str()) {
        tracing::warn!("CSRF state mismatch");
        return Err(StatusCode::BAD_REQUEST);
    }

    let verifier_secret: Option<String> = session
        .get(SESSION_KEY_VERIFIER)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let verifier_secret = verifier_secret.ok_or(StatusCode::BAD_REQUEST)?;

    session
        .remove::<String>(SESSION_KEY_VERIFIER)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    session
        .remove::<String>(SESSION_KEY_CSRF)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let auth_url = AuthUrl::new(state.oauth.authorization_endpoint.clone()).map_err(|e| {
        tracing::error!("invalid auth URL: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    let token_url = TokenUrl::new(state.oauth.token_endpoint.clone()).map_err(|e| {
        tracing::error!("invalid token URL: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    let redirect_url = RedirectUrl::new(redirect_uri.to_string()).map_err(|e| {
        tracing::error!("invalid redirect URI: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let client = BasicClient::new(ClientId::new(client_id.to_string()))
        .set_auth_uri(auth_url)
        .set_token_uri(token_url)
        .set_redirect_uri(redirect_url);

    let token_response = client
        .exchange_code(AuthorizationCode::new(params.code))
        .set_pkce_verifier(PkceCodeVerifier::new(verifier_secret))
        .request_async(&async_http_client)
        .await
        .map_err(|e| {
            tracing::error!("token exchange failed: {e}");
            StatusCode::BAD_GATEWAY
        })?;

    let access_token = token_response.access_token().secret();

    let sub = state.jwks.validate_token(access_token).await.map_err(|e| {
        tracing::warn!("token validation failed: {e}");
        StatusCode::UNAUTHORIZED
    })?;

    session
        .insert(SESSION_KEY_USER_ID, &sub)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    tracing::info!("user {sub} logged in");
    Ok(Redirect::to("/").into_response())
}

#[tracing::instrument(skip(session))]
pub async fn logout_handler(session: Session) -> Result<Response, StatusCode> {
    session
        .flush()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Redirect::to("/").into_response())
}
