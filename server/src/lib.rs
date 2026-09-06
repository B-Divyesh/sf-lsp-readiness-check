use std::{
    collections::HashMap,
    net::{IpAddr, SocketAddr},
    path::{Path, PathBuf},
    sync::{
        Arc, Mutex,
        atomic::{AtomicU64, Ordering},
    },
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

use axum::{
    Json, Router,
    extract::{ConnectInfo, Extension, Path as AxumPath, Query, Request, State},
    http::{HeaderMap, HeaderValue, Method, StatusCode, header},
    middleware::{self, Next},
    response::{IntoResponse, Redirect, Response},
    routing::{delete, get, post},
};
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, decode_header, encode,
    jwk::JwkSet,
};
use lsp_readiness_check::{SCHEMA, SignedPacket, verify};
use rand::{RngCore, rngs::OsRng};
use reqwest::Client;
use rusqlite::{Connection, OpenFlags, OptionalExtension, params};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use tower_http::{
    cors::{AllowOrigin, CorsLayer},
    limit::RequestBodyLimitLayer,
};
use tracing::{error, info};
use url::Url;

const MIGRATION_UP: &str = include_str!("../migrations/001_init.sql");
const MIGRATION_DOWN: &str = include_str!("../migrations/001_down.sql");
const MAX_BODY_BYTES: usize = 64 * 1024;
const SUBSCRIPTION_PRICE_MINOR: u32 = 4_900;

#[derive(Clone)]
pub struct AppState {
    db: Database,
    auth: Authenticator,
    github: Option<GithubClient>,
    public: PublicConfig,
    limiter: RateLimiter,
    metrics: Metrics,
}

#[derive(Clone)]
pub struct Database {
    path: Arc<PathBuf>,
}

#[derive(Debug, Clone)]
pub struct ServiceConfig {
    pub database_path: PathBuf,
    pub public_origin: String,
    pub api_origin: String,
    pub ciam: Option<CiamConfig>,
    pub github: Option<GithubConfig>,
    pub requests_per_minute: u32,
    pub allow_test_auth: bool,
}

#[derive(Debug, Clone)]
pub struct CiamConfig {
    pub issuer: String,
    pub audience: String,
    pub client_id: String,
    pub authorize_url: String,
    pub token_url: String,
    pub jwks_url: String,
    pub scope: String,
}

#[derive(Debug, Clone)]
pub struct GithubConfig {
    pub app_id: String,
    pub app_slug: String,
    pub private_key_pem: String,
    pub api_base: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct PublicConfig {
    pub identity_configured: bool,
    pub github_app_configured: bool,
    pub subscription_configured: bool,
    pub client_id: Option<String>,
    pub authorize_url: Option<String>,
    pub token_url: Option<String>,
    pub scope: Option<String>,
    pub redirect_url: String,
    pub api_origin: String,
}

impl ServiceConfig {
    pub fn from_env() -> anyhow::Result<Self> {
        let database_path = std::env::var("DATABASE_PATH")
            .map(PathBuf::from)
            .unwrap_or_else(|_| PathBuf::from("/data/lsp-readiness.db"));
        let public_origin = std::env::var("PUBLIC_ORIGIN")
            .unwrap_or_else(|_| "https://lsp-readiness-check.sociobot.in".into());
        let api_origin = std::env::var("API_ORIGIN")
            .unwrap_or_else(|_| "https://lsp-readiness-check-api.sociobot.in".into());

        let ciam_values = [
            "CIAM_ISSUER",
            "CIAM_AUDIENCE",
            "CIAM_CLIENT_ID",
            "CIAM_AUTHORIZE_URL",
            "CIAM_TOKEN_URL",
            "CIAM_JWKS_URL",
            "CIAM_SCOPE",
        ]
        .map(|name| {
            std::env::var(name)
                .ok()
                .filter(|value| !value.trim().is_empty())
        });
        let ciam = if ciam_values.iter().all(Option::is_none) {
            None
        } else if ciam_values.iter().all(Option::is_some) {
            Some(CiamConfig {
                issuer: ciam_values[0].clone().unwrap(),
                audience: ciam_values[1].clone().unwrap(),
                client_id: ciam_values[2].clone().unwrap(),
                authorize_url: validate_https_url(
                    ciam_values[3].as_deref().unwrap(),
                    "CIAM_AUTHORIZE_URL",
                )?,
                token_url: validate_https_url(
                    ciam_values[4].as_deref().unwrap(),
                    "CIAM_TOKEN_URL",
                )?,
                jwks_url: validate_https_url(ciam_values[5].as_deref().unwrap(), "CIAM_JWKS_URL")?,
                scope: ciam_values[6].clone().unwrap(),
            })
        } else {
            anyhow::bail!("CIAM configuration is incomplete; set all seven CIAM_* values or none")
        };

        let github_values = [
            "GITHUB_APP_ID",
            "GITHUB_APP_SLUG",
            "GITHUB_APP_PRIVATE_KEY_PEM",
        ]
        .map(|name| {
            std::env::var(name)
                .ok()
                .filter(|value| !value.trim().is_empty())
        });
        let github = if github_values.iter().all(Option::is_none) {
            None
        } else if github_values.iter().all(Option::is_some) {
            Some(GithubConfig {
                app_id: github_values[0].clone().unwrap(),
                app_slug: github_values[1].clone().unwrap(),
                private_key_pem: github_values[2].clone().unwrap(),
                api_base: "https://api.github.com".into(),
            })
        } else {
            anyhow::bail!(
                "GitHub App configuration is incomplete; set all three GITHUB_APP_* values or none"
            )
        };

        let requests_per_minute = std::env::var("REQUESTS_PER_MINUTE")
            .ok()
            .map(|value| value.parse())
            .transpose()?
            .unwrap_or(60);
        if !(10..=10_000).contains(&requests_per_minute) {
            anyhow::bail!("REQUESTS_PER_MINUTE must be between 10 and 10000")
        }
        let allow_test_auth = cfg!(debug_assertions)
            && std::env::var("LSP_READINESS_TEST_AUTH").as_deref() == Ok("1");

        Ok(Self {
            database_path,
            public_origin,
            api_origin,
            ciam,
            github,
            requests_per_minute,
            allow_test_auth,
        })
    }
}

fn validate_https_url(value: &str, name: &str) -> anyhow::Result<String> {
    let parsed = Url::parse(value)?;
    if parsed.scheme() != "https" || parsed.host_str().is_none() {
        anyhow::bail!("{name} must be an https URL")
    }
    Ok(value.trim_end_matches('/').to_string())
}

impl Database {
    fn open_connection(path: &Path) -> rusqlite::Result<Connection> {
        // The production database lives on the fleet's Azure Files mount.
        // SQLite's default fcntl locks are not reliable through SMB, while
        // the dot-file VFS coordinates the single allowed replica with a
        // lock file on that same durable mount.
        Connection::open_with_flags_and_vfs(
            path,
            OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE,
            "unix-dotfile",
        )
    }

    pub fn open(path: impl AsRef<Path>) -> anyhow::Result<Self> {
        let path = path.as_ref().to_path_buf();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let db = Self {
            path: Arc::new(path),
        };
        {
            let connection = Self::open_connection(db.path.as_ref())?;
            connection.pragma_update(None, "busy_timeout", 10_000)?;
            connection.pragma_update(None, "journal_mode", "DELETE")?;
            connection.pragma_update(None, "synchronous", "FULL")?;
        }
        db.migrate()?;
        Ok(db)
    }

    fn connection(&self) -> anyhow::Result<Connection> {
        let connection = Self::open_connection(self.path.as_ref())?;
        connection.pragma_update(None, "foreign_keys", "ON")?;
        connection.pragma_update(None, "busy_timeout", 10_000)?;
        Ok(connection)
    }

    pub fn migrate(&self) -> anyhow::Result<()> {
        let connection = self.connection()?;
        connection.execute_batch(MIGRATION_UP)?;
        Ok(())
    }

    pub fn health(&self) -> anyhow::Result<i64> {
        let connection = self.connection()?;
        Ok(connection.query_row(
            "SELECT COALESCE(MAX(version), 0) FROM schema_migrations",
            [],
            |row| row.get(0),
        )?)
    }

    pub fn backup(&self, destination: &Path) -> anyhow::Result<()> {
        if let Some(parent) = destination.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let source = self.connection()?;
        let mut target = Self::open_connection(destination)?;
        let backup = rusqlite::backup::Backup::new(&source, &mut target)?;
        backup.run_to_completion(128, Duration::from_millis(10), None)?;
        drop(backup);
        target.query_row("PRAGMA integrity_check", [], |row| row.get::<_, String>(0))?;
        Ok(())
    }

    pub fn restore(source: &Path, destination: &Path) -> anyhow::Result<()> {
        let source_connection = Self::open_connection(source)?;
        let integrity: String =
            source_connection.query_row("PRAGMA integrity_check", [], |row| row.get(0))?;
        if integrity != "ok" {
            anyhow::bail!("backup failed SQLite integrity check")
        }
        if let Some(parent) = destination.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let temporary = destination.with_extension("restore.tmp");
        std::fs::copy(source, &temporary)?;
        std::fs::rename(temporary, destination)?;
        Ok(())
    }
}

#[derive(Clone)]
enum Authenticator {
    Ciam(Arc<CiamAuthenticator>),
    Debug,
    Missing,
}

struct CiamAuthenticator {
    config: CiamConfig,
    client: Client,
    jwks: Mutex<Option<JwkSet>>,
}

#[derive(Debug, Deserialize)]
struct TokenClaims {
    sub: String,
    iss: String,
    exp: usize,
    name: Option<String>,
    email: Option<String>,
    emails: Option<Vec<String>>,
}

#[derive(Clone, Debug)]
struct Identity {
    subject: String,
    name: Option<String>,
    email: Option<String>,
}

impl Authenticator {
    async fn authenticate(&self, token: &str) -> Result<Identity, ApiError> {
        match self {
            Self::Missing => Err(ApiError::dependency(
                "identity_not_configured",
                "Sign-in needs the product CIAM registration before this API can accept accounts.",
            )),
            Self::Debug => {
                let subject = token
                    .strip_prefix("test-")
                    .filter(|value| !value.is_empty() && value.len() <= 80)
                    .ok_or_else(|| ApiError::unauthorized("invalid test identity"))?;
                Ok(Identity {
                    subject: subject.into(),
                    name: Some(format!("Test {subject}")),
                    email: Some(format!("{subject}@example.test")),
                })
            }
            Self::Ciam(authenticator) => authenticator.authenticate(token).await,
        }
    }
}

impl CiamAuthenticator {
    async fn authenticate(&self, token: &str) -> Result<Identity, ApiError> {
        let header = decode_header(token)
            .map_err(|_| ApiError::unauthorized("invalid access token header"))?;
        if header.alg != Algorithm::RS256 {
            return Err(ApiError::unauthorized("access token must use RS256"));
        }
        let kid = header
            .kid
            .ok_or_else(|| ApiError::unauthorized("access token has no key id"))?;
        let jwks = self.jwks().await?;
        let jwk = jwks
            .find(&kid)
            .ok_or_else(|| ApiError::unauthorized("access token key is unknown"))?;
        let key = DecodingKey::from_jwk(jwk)
            .map_err(|_| ApiError::unauthorized("access token key is invalid"))?;
        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_audience(&[&self.config.audience]);
        validation.set_issuer(&[&self.config.issuer]);
        validation.validate_exp = true;
        let claims = decode::<TokenClaims>(token, &key, &validation)
            .map_err(|_| ApiError::unauthorized("access token is invalid or expired"))?
            .claims;
        if claims.sub.trim().is_empty() || claims.iss != self.config.issuer || claims.exp == 0 {
            return Err(ApiError::unauthorized("access token claims are incomplete"));
        }
        Ok(Identity {
            subject: claims.sub,
            name: claims.name,
            email: claims
                .email
                .or_else(|| claims.emails.and_then(|items| items.into_iter().next())),
        })
    }

    async fn jwks(&self) -> Result<JwkSet, ApiError> {
        if let Some(cached) = self.jwks.lock().expect("jwks mutex").clone() {
            return Ok(cached);
        }
        let response = self
            .client
            .get(&self.config.jwks_url)
            .timeout(Duration::from_secs(5))
            .send()
            .await
            .map_err(|_| {
                ApiError::dependency(
                    "identity_unavailable",
                    "The identity keys could not be reached. Try again.",
                )
            })?;
        if !response.status().is_success() {
            return Err(ApiError::dependency(
                "identity_unavailable",
                "The identity keys could not be loaded. Try again.",
            ));
        }
        let jwks: JwkSet = response.json().await.map_err(|_| {
            ApiError::dependency(
                "identity_unavailable",
                "The identity keys were not valid JSON.",
            )
        })?;
        *self.jwks.lock().expect("jwks mutex") = Some(jwks.clone());
        Ok(jwks)
    }
}

#[derive(Clone, Debug)]
struct AuthUser {
    user_id: String,
    organization_id: String,
    role: String,
}

#[derive(Clone)]
struct GithubClient {
    config: GithubConfig,
    client: Client,
}

#[derive(Serialize)]
struct GithubAppClaims {
    iat: u64,
    exp: u64,
    iss: String,
}

#[derive(Deserialize)]
struct GithubTokenResponse {
    token: String,
}

#[derive(Deserialize)]
struct GithubRepositoriesResponse {
    repositories: Vec<GithubRepository>,
}

#[derive(Deserialize)]
struct GithubRepository {
    id: i64,
    name: String,
    private: bool,
    owner: GithubOwner,
}

#[derive(Deserialize)]
struct GithubOwner {
    login: String,
}

impl GithubClient {
    async fn repositories(&self, installation_id: i64) -> Result<Vec<GithubRepository>, ApiError> {
        let now = now() as u64;
        let claims = GithubAppClaims {
            iat: now.saturating_sub(30),
            exp: now + 540,
            iss: self.config.app_id.clone(),
        };
        let key =
            EncodingKey::from_rsa_pem(self.config.private_key_pem.as_bytes()).map_err(|_| {
                ApiError::dependency(
                    "github_app_invalid",
                    "The GitHub App signing key is not valid.",
                )
            })?;
        let app_token = encode(&Header::new(Algorithm::RS256), &claims, &key).map_err(|_| {
            ApiError::dependency(
                "github_app_invalid",
                "The GitHub App token could not be signed.",
            )
        })?;
        let base = self.config.api_base.trim_end_matches('/');
        let token_response = self
            .client
            .post(format!(
                "{base}/app/installations/{installation_id}/access_tokens"
            ))
            .bearer_auth(app_token)
            .header(header::ACCEPT, "application/vnd.github+json")
            .header(header::USER_AGENT, "lsp-readiness-check/0.1")
            .send()
            .await
            .map_err(|_| {
                ApiError::dependency(
                    "github_unavailable",
                    "GitHub could not be reached. Try the connection again.",
                )
            })?;
        if !token_response.status().is_success() {
            return Err(ApiError::dependency(
                "github_installation_rejected",
                "GitHub did not authorize this installation.",
            ));
        }
        let token = token_response
            .json::<GithubTokenResponse>()
            .await
            .map_err(|_| {
                ApiError::dependency(
                    "github_response_invalid",
                    "GitHub returned an invalid installation token.",
                )
            })?
            .token;
        let repositories = self
            .client
            .get(format!("{base}/installation/repositories?per_page=100"))
            .bearer_auth(token)
            .header(header::ACCEPT, "application/vnd.github+json")
            .header(header::USER_AGENT, "lsp-readiness-check/0.1")
            .send()
            .await
            .map_err(|_| {
                ApiError::dependency(
                    "github_unavailable",
                    "GitHub could not list this installation.",
                )
            })?;
        if !repositories.status().is_success() {
            return Err(ApiError::dependency(
                "github_installation_rejected",
                "GitHub did not return repositories for this installation.",
            ));
        }
        Ok(repositories
            .json::<GithubRepositoriesResponse>()
            .await
            .map_err(|_| {
                ApiError::dependency(
                    "github_response_invalid",
                    "GitHub returned an invalid repository list.",
                )
            })?
            .repositories)
    }
}

#[derive(Clone)]
struct RateLimiter {
    limit: u32,
    windows: Arc<Mutex<HashMap<String, RateWindow>>>,
}

struct RateWindow {
    started: Instant,
    count: u32,
}

#[derive(Clone, Default)]
struct Metrics {
    requests: Arc<AtomicU64>,
    server_errors: Arc<AtomicU64>,
}

impl RateLimiter {
    fn check(&self, key: &str) -> Result<(), u64> {
        let mut windows = self.windows.lock().expect("rate limiter mutex");
        let window = windows.entry(key.into()).or_insert(RateWindow {
            started: Instant::now(),
            count: 0,
        });
        if window.started.elapsed() >= Duration::from_secs(60) {
            window.started = Instant::now();
            window.count = 0;
        }
        if window.count >= self.limit {
            return Err(60_u64
                .saturating_sub(window.started.elapsed().as_secs())
                .max(1));
        }
        window.count += 1;
        Ok(())
    }
}

pub fn build_state(config: ServiceConfig) -> anyhow::Result<AppState> {
    let db = Database::open(&config.database_path)?;
    let auth = if config.allow_test_auth {
        Authenticator::Debug
    } else if let Some(ciam) = config.ciam.clone() {
        Authenticator::Ciam(Arc::new(CiamAuthenticator {
            config: ciam,
            client: Client::builder().build()?,
            jwks: Mutex::new(None),
        }))
    } else {
        Authenticator::Missing
    };
    let github = config.github.clone().map(|github| GithubClient {
        config: github,
        client: Client::new(),
    });
    let public = PublicConfig {
        identity_configured: config.ciam.is_some(),
        github_app_configured: config.github.is_some(),
        subscription_configured: false,
        client_id: config.ciam.as_ref().map(|ciam| ciam.client_id.clone()),
        authorize_url: config.ciam.as_ref().map(|ciam| ciam.authorize_url.clone()),
        token_url: config.ciam.as_ref().map(|ciam| ciam.token_url.clone()),
        scope: config.ciam.as_ref().map(|ciam| ciam.scope.clone()),
        redirect_url: format!("{}/sign-in", config.public_origin.trim_end_matches('/')),
        api_origin: config.api_origin,
    };
    Ok(AppState {
        db,
        auth,
        github,
        public,
        limiter: RateLimiter {
            limit: config.requests_per_minute,
            windows: Arc::new(Mutex::new(HashMap::new())),
        },
        metrics: Metrics::default(),
    })
}

pub fn router(state: AppState, public_origin: &str) -> anyhow::Result<Router> {
    let allowed_origin = HeaderValue::from_str(public_origin)?;
    let protected = Router::new()
        .route("/session", get(session))
        .route("/repositories", get(list_repositories))
        .route("/repositories/{id}", get(get_repository))
        .route("/repositories/{id}/policy", get(get_policy).put(put_policy))
        .route("/repositories/{id}/report-token", post(rotate_report_token))
        .route("/github/connect", post(begin_github_connect))
        .route("/billing", get(billing_status))
        .route("/account/export", get(export_account))
        .route("/account", delete(delete_account))
        .layer(middleware::from_fn_with_state(
            state.clone(),
            authenticate_request,
        ));

    let mut api = Router::new()
        .route("/config", get(public_config))
        .route("/repositories/{id}/runs", post(upload_run))
        .route("/github/callback", get(github_callback))
        .merge(protected);
    if matches!(state.auth, Authenticator::Debug) {
        api = api.route("/test/repositories", post(seed_test_repository));
    }

    Ok(Router::new()
        .route("/healthz", get(health))
        .route("/metrics", get(metrics))
        .nest("/api/v1", api)
        .layer(RequestBodyLimitLayer::new(MAX_BODY_BYTES))
        .layer(
            CorsLayer::new()
                .allow_origin(AllowOrigin::exact(allowed_origin))
                .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
                .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE]),
        )
        .layer(middleware::from_fn_with_state(state.clone(), request_log))
        .with_state(state))
}

async fn request_log(State(state): State<AppState>, request: Request, next: Next) -> Response {
    state.metrics.requests.fetch_add(1, Ordering::Relaxed);
    let request_id = random_id("req");
    let method = request.method().clone();
    let path = request.uri().path().to_string();
    let started = Instant::now();
    let mut response = next.run(request).await;
    if response.status().is_server_error() {
        state.metrics.server_errors.fetch_add(1, Ordering::Relaxed);
    }
    if let Ok(value) = HeaderValue::from_str(&request_id) {
        response.headers_mut().insert("x-request-id", value);
    }
    response.headers_mut().insert(
        header::X_CONTENT_TYPE_OPTIONS,
        HeaderValue::from_static("nosniff"),
    );
    response.headers_mut().insert(
        header::REFERRER_POLICY,
        HeaderValue::from_static("no-referrer"),
    );
    response
        .headers_mut()
        .insert(header::CACHE_CONTROL, HeaderValue::from_static("no-store"));
    response.headers_mut().insert(
        header::CONTENT_SECURITY_POLICY,
        HeaderValue::from_static("default-src 'none'; frame-ancestors 'none'"),
    );
    info!(
        request_id,
        method = %method,
        path,
        status = response.status().as_u16(),
        elapsed_ms = started.elapsed().as_millis() as u64,
        "request_complete"
    );
    response
}

async fn metrics(State(state): State<AppState>) -> impl IntoResponse {
    let body = format!(
        "# TYPE lsp_readiness_http_requests_total counter\nlsp_readiness_http_requests_total {}\n# TYPE lsp_readiness_http_server_errors_total counter\nlsp_readiness_http_server_errors_total {}\n",
        state.metrics.requests.load(Ordering::Relaxed),
        state.metrics.server_errors.load(Ordering::Relaxed),
    );
    ([(header::CONTENT_TYPE, "text/plain; version=0.0.4")], body)
}

async fn authenticate_request(
    State(state): State<AppState>,
    ConnectInfo(address): ConnectInfo<SocketAddr>,
    mut request: Request,
    next: Next,
) -> Response {
    if let Err(retry) = state
        .limiter
        .check(&format!("ip:{}", request_ip(request.headers(), address)))
    {
        return rate_limited(retry);
    }
    let token = match bearer_token(request.headers()) {
        Ok(token) => token,
        Err(error) => return error.into_response(),
    };
    let identity = match state.auth.authenticate(token).await {
        Ok(identity) => identity,
        Err(error) => return error.into_response(),
    };
    let user = match upsert_identity(&state.db, &identity) {
        Ok(user) => user,
        Err(error) => {
            error!(error = %error, "identity_persistence_failed");
            return ApiError::internal().into_response();
        }
    };
    if let Err(retry) = state
        .limiter
        .check(&format!("org:{}", user.organization_id))
    {
        return rate_limited(retry);
    }
    request.extensions_mut().insert(user);
    next.run(request).await
}

fn request_ip(headers: &HeaderMap, address: SocketAddr) -> String {
    ["x-envoy-external-address", "x-forwarded-for"]
        .iter()
        .find_map(|name| {
            headers
                .get(*name)
                .and_then(|value| value.to_str().ok())
                .and_then(|value| value.split(',').next())
                .map(str::trim)
                .and_then(|value| value.parse::<IpAddr>().ok())
        })
        .unwrap_or_else(|| address.ip())
        .to_string()
}

fn rate_limited(retry: u64) -> Response {
    let mut response = ApiError::new(
        StatusCode::TOO_MANY_REQUESTS,
        "rate_limit",
        "Too many requests. Wait before trying again.",
    )
    .into_response();
    response.headers_mut().insert(
        header::RETRY_AFTER,
        HeaderValue::from_str(&retry.to_string()).unwrap_or(HeaderValue::from_static("1")),
    );
    response
}

fn bearer_token(headers: &HeaderMap) -> Result<&str, ApiError> {
    headers
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .filter(|value| !value.is_empty())
        .ok_or_else(|| ApiError::unauthorized("A bearer access token is required."))
}

fn upsert_identity(db: &Database, identity: &Identity) -> anyhow::Result<AuthUser> {
    let mut connection = db.connection()?;
    let transaction = connection.transaction()?;
    let timestamp = now();
    let existing: Option<(String, String, String)> = transaction
        .query_row(
            "SELECT u.id, m.organization_id, m.role FROM users u JOIN memberships m ON m.user_id = u.id WHERE u.ciam_subject = ?1 LIMIT 1",
            [&identity.subject],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
        )
        .optional()?;
    let (user_id, organization_id, role) = if let Some(existing) = existing {
        transaction.execute(
            "UPDATE users SET display_name = ?1, email = ?2, updated_at = ?3 WHERE id = ?4",
            params![identity.name, identity.email, timestamp, existing.0],
        )?;
        existing
    } else {
        let user_id = stable_id("usr", &identity.subject);
        let organization_id = stable_id("org", &identity.subject);
        let organization_name = identity
            .name
            .as_deref()
            .filter(|name| !name.trim().is_empty())
            .map(|name| format!("{name}'s team"))
            .unwrap_or_else(|| "My team".into());
        transaction.execute(
            "INSERT INTO users(id, ciam_subject, display_name, email, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?5)",
            params![user_id, identity.subject, identity.name, identity.email, timestamp],
        )?;
        transaction.execute(
            "INSERT INTO organizations(id, name, created_at) VALUES (?1, ?2, ?3)",
            params![organization_id, organization_name, timestamp],
        )?;
        transaction.execute(
            "INSERT INTO memberships(organization_id, user_id, role, created_at) VALUES (?1, ?2, 'owner', ?3)",
            params![organization_id, user_id, timestamp],
        )?;
        (user_id, organization_id, "owner".into())
    };
    transaction.commit()?;
    Ok(AuthUser {
        user_id,
        organization_id,
        role,
    })
}

async fn health(State(state): State<AppState>) -> Result<Json<Value>, ApiError> {
    let version = state.db.health().map_err(|_| ApiError::internal())?;
    Ok(Json(
        json!({ "status": "ok", "database": "ok", "schema_version": version }),
    ))
}

async fn public_config(
    State(state): State<AppState>,
    ConnectInfo(address): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
) -> Result<Json<PublicConfig>, ApiError> {
    if let Err(retry) = state
        .limiter
        .check(&format!("public:{}", request_ip(&headers, address)))
    {
        return Err(ApiError::rate_limited(retry));
    }
    Ok(Json(state.public))
}

async fn session(
    State(state): State<AppState>,
    Extension(user): Extension<AuthUser>,
) -> Result<Json<Value>, ApiError> {
    let connection = state.db.connection().map_err(|_| ApiError::internal())?;
    let result = connection
        .query_row(
            "SELECT u.display_name, u.email, o.name FROM users u JOIN memberships m ON m.user_id = u.id JOIN organizations o ON o.id = m.organization_id WHERE u.id = ?1 AND o.id = ?2",
            params![user.user_id, user.organization_id],
            |row| Ok(json!({
                "user": { "id": user.user_id, "display_name": row.get::<_, Option<String>>(0)?, "email": row.get::<_, Option<String>>(1)? },
                "organization": { "id": user.organization_id, "name": row.get::<_, String>(2)?, "role": user.role }
            })),
        )
        .map_err(|_| ApiError::internal())?;
    Ok(Json(result))
}

#[derive(Serialize)]
struct RepositorySummary {
    id: String,
    owner: String,
    name: String,
    private: bool,
    created_at: i64,
}

async fn list_repositories(
    State(state): State<AppState>,
    Extension(user): Extension<AuthUser>,
) -> Result<Json<Value>, ApiError> {
    let connection = state.db.connection().map_err(|_| ApiError::internal())?;
    let mut statement = connection
        .prepare("SELECT id, owner, name, private, created_at FROM repositories WHERE organization_id = ?1 ORDER BY owner, name")
        .map_err(|_| ApiError::internal())?;
    let repositories = statement
        .query_map([user.organization_id], |row| {
            Ok(RepositorySummary {
                id: row.get(0)?,
                owner: row.get(1)?,
                name: row.get(2)?,
                private: row.get::<_, i64>(3)? == 1,
                created_at: row.get(4)?,
            })
        })
        .map_err(|_| ApiError::internal())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| ApiError::internal())?;
    Ok(Json(json!({ "repositories": repositories })))
}

async fn get_repository(
    State(state): State<AppState>,
    Extension(user): Extension<AuthUser>,
    AxumPath(id): AxumPath<String>,
) -> Result<Json<Value>, ApiError> {
    let connection = state.db.connection().map_err(|_| ApiError::internal())?;
    let repository = connection
        .query_row(
            "SELECT id, owner, name, private, created_at FROM repositories WHERE organization_id = ?1 AND id = ?2",
            params![user.organization_id, id],
            |row| Ok(json!({ "id": row.get::<_, String>(0)?, "owner": row.get::<_, String>(1)?, "name": row.get::<_, String>(2)?, "private": row.get::<_, i64>(3)? == 1, "created_at": row.get::<_, i64>(4)? })),
        )
        .optional()
        .map_err(|_| ApiError::internal())?
        .ok_or_else(ApiError::not_found)?;
    Ok(Json(repository))
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct PolicyInput {
    required_lsp: bool,
    required_formatters: bool,
    required_tests: bool,
}

async fn get_policy(
    State(state): State<AppState>,
    Extension(user): Extension<AuthUser>,
    AxumPath(id): AxumPath<String>,
) -> Result<Json<Value>, ApiError> {
    ensure_repository(&state.db, &user.organization_id, &id)?;
    let connection = state.db.connection().map_err(|_| ApiError::internal())?;
    let policy = connection
        .query_row(
            "SELECT required_lsp, required_formatters, required_tests, version, updated_at FROM policies WHERE organization_id = ?1 AND repository_id = ?2",
            params![user.organization_id, id],
            |row| Ok(json!({ "required_lsp": row.get::<_, i64>(0)? == 1, "required_formatters": row.get::<_, i64>(1)? == 1, "required_tests": row.get::<_, i64>(2)? == 1, "version": row.get::<_, i64>(3)?, "updated_at": row.get::<_, i64>(4)? })),
        )
        .optional()
        .map_err(|_| ApiError::internal())?
        .unwrap_or_else(|| json!({ "required_lsp": true, "required_formatters": true, "required_tests": true, "version": 0, "updated_at": null }));
    Ok(Json(policy))
}

async fn put_policy(
    State(state): State<AppState>,
    Extension(user): Extension<AuthUser>,
    AxumPath(id): AxumPath<String>,
    Json(input): Json<PolicyInput>,
) -> Result<Json<Value>, ApiError> {
    ensure_owner(&user)?;
    ensure_repository(&state.db, &user.organization_id, &id)?;
    let timestamp = now();
    let connection = state.db.connection().map_err(|_| ApiError::internal())?;
    connection
        .execute(
            "INSERT INTO policies(id, organization_id, repository_id, required_lsp, required_formatters, required_tests, version, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, 1, ?7, ?7) ON CONFLICT(organization_id, repository_id) DO UPDATE SET required_lsp = excluded.required_lsp, required_formatters = excluded.required_formatters, required_tests = excluded.required_tests, version = policies.version + 1, updated_at = excluded.updated_at",
            params![random_id("pol"), user.organization_id, id, input.required_lsp as i64, input.required_formatters as i64, input.required_tests as i64, timestamp],
        )
        .map_err(|_| ApiError::internal())?;
    get_policy(State(state), Extension(user), AxumPath(id)).await
}

async fn rotate_report_token(
    State(state): State<AppState>,
    Extension(user): Extension<AuthUser>,
    AxumPath(id): AxumPath<String>,
) -> Result<Json<Value>, ApiError> {
    ensure_owner(&user)?;
    ensure_repository(&state.db, &user.organization_id, &id)?;
    let token = random_token();
    let connection = state.db.connection().map_err(|_| ApiError::internal())?;
    let changed = connection
        .execute(
            "UPDATE repositories SET report_token_hash = ?1, updated_at = ?2 WHERE organization_id = ?3 AND id = ?4",
            params![hash_token(&token), now(), user.organization_id, id],
        )
        .map_err(|_| ApiError::internal())?;
    if changed != 1 {
        return Err(ApiError::not_found());
    }
    Ok(Json(json!({ "token": token, "shown_once": true })))
}

#[derive(Deserialize)]
struct GithubCallback {
    installation_id: i64,
    state: String,
}

async fn begin_github_connect(
    State(state): State<AppState>,
    Extension(user): Extension<AuthUser>,
) -> Result<Json<Value>, ApiError> {
    ensure_owner(&user)?;
    let github = state.github.as_ref().ok_or_else(|| {
        ApiError::dependency(
            "github_app_not_configured",
            "Connecting a repository needs the product GitHub App registration.",
        )
    })?;
    let token = random_token();
    let connection = state.db.connection().map_err(|_| ApiError::internal())?;
    connection
        .execute(
            "INSERT INTO github_connect_states(state_hash, organization_id, expires_at) VALUES (?1, ?2, ?3)",
            params![hash_token(&token), user.organization_id, now() + 600],
        )
        .map_err(|_| ApiError::internal())?;
    let mut url = Url::parse(&format!(
        "https://github.com/apps/{}/installations/new",
        github.config.app_slug
    ))
    .map_err(|_| ApiError::internal())?;
    url.query_pairs_mut().append_pair("state", &token);
    Ok(Json(json!({ "url": url.as_str(), "expires_in": 600 })))
}

async fn github_callback(
    State(state): State<AppState>,
    Query(query): Query<GithubCallback>,
) -> Result<Redirect, ApiError> {
    let github = state.github.as_ref().ok_or_else(|| {
        ApiError::dependency(
            "github_app_not_configured",
            "The GitHub App is not configured.",
        )
    })?;
    let organization_id = {
        let mut connection = state.db.connection().map_err(|_| ApiError::internal())?;
        let transaction = connection.transaction().map_err(|_| ApiError::internal())?;
        let organization_id: Option<String> = transaction
            .query_row(
                "SELECT organization_id FROM github_connect_states WHERE state_hash = ?1 AND expires_at >= ?2",
                params![hash_token(&query.state), now()],
                |row| row.get(0),
            )
            .optional()
            .map_err(|_| ApiError::internal())?;
        let organization_id = organization_id.ok_or_else(|| {
            ApiError::new(
                StatusCode::BAD_REQUEST,
                "github_state_invalid",
                "The GitHub connection expired. Start again.",
            )
        })?;
        transaction
            .execute(
                "DELETE FROM github_connect_states WHERE state_hash = ?1",
                [hash_token(&query.state)],
            )
            .map_err(|_| ApiError::internal())?;
        transaction.commit().map_err(|_| ApiError::internal())?;
        organization_id
    };

    let repositories = github.repositories(query.installation_id).await?;
    persist_github_repositories(
        &state.db,
        &organization_id,
        query.installation_id,
        &repositories,
    )?;
    Ok(Redirect::to(
        "https://lsp-readiness-check.sociobot.in/app/repositories?github=connected",
    ))
}

fn persist_github_repositories(
    db: &Database,
    organization_id: &str,
    installation_id: i64,
    repositories: &[GithubRepository],
) -> Result<(), ApiError> {
    if repositories.len() > 100 {
        return Err(ApiError::new(
            StatusCode::BAD_REQUEST,
            "too_many_repositories",
            "Connect at most 100 repositories per installation.",
        ));
    }
    let mut connection = db.connection().map_err(|_| ApiError::internal())?;
    let transaction = connection.transaction().map_err(|_| ApiError::internal())?;
    let installation_key = format!("ghi_{installation_id}");
    let account = repositories
        .first()
        .map(|repository| repository.owner.login.as_str())
        .unwrap_or("GitHub");
    let existing_organization: Option<String> = transaction
        .query_row(
            "SELECT organization_id FROM github_installations WHERE github_installation_id = ?1",
            [installation_id],
            |row| row.get(0),
        )
        .optional()
        .map_err(|_| ApiError::internal())?;
    if existing_organization
        .as_deref()
        .is_some_and(|existing| existing != organization_id)
    {
        return Err(ApiError::new(
            StatusCode::CONFLICT,
            "github_installation_owned",
            "This GitHub installation is already connected to another organization.",
        ));
    }
    transaction.execute(
        "INSERT INTO github_installations(id, organization_id, github_installation_id, account_login, created_at) VALUES (?1, ?2, ?3, ?4, ?5) ON CONFLICT(github_installation_id) DO UPDATE SET account_login = excluded.account_login",
        params![installation_key, organization_id, installation_id, account, now()],
    ).map_err(|_| ApiError::internal())?;
    for repository in repositories {
        validate_short_text(&repository.name, 100, "repository name")?;
        validate_short_text(&repository.owner.login, 100, "repository owner")?;
        let repository_id = format!("repo_{}", repository.id);
        transaction.execute(
            "INSERT INTO repositories(id, organization_id, github_installation_id, github_repository_id, owner, name, private, report_token_hash, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?9) ON CONFLICT(organization_id, github_repository_id) DO UPDATE SET owner = excluded.owner, name = excluded.name, private = excluded.private, updated_at = excluded.updated_at",
            params![repository_id, organization_id, installation_key, repository.id, repository.owner.login, repository.name, repository.private as i64, hash_token(&random_token()), now()],
        ).map_err(|_| ApiError::internal())?;
    }
    transaction.commit().map_err(|_| ApiError::internal())?;
    Ok(())
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct RunUpload {
    pull_request: Option<u64>,
    external_run_id: Option<String>,
    report: SignedPacket,
}

async fn upload_run(
    State(state): State<AppState>,
    ConnectInfo(address): ConnectInfo<SocketAddr>,
    AxumPath(repository_id): AxumPath<String>,
    headers: HeaderMap,
    Json(upload): Json<RunUpload>,
) -> Result<(StatusCode, Json<Value>), ApiError> {
    if let Err(retry) = state
        .limiter
        .check(&format!("upload-ip:{}", request_ip(&headers, address)))
    {
        return Err(ApiError::rate_limited(retry));
    }
    let token = headers
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Report "))
        .filter(|value| !value.is_empty())
        .ok_or_else(|| ApiError::unauthorized("A report token is required."))?;
    let connection = state.db.connection().map_err(|_| ApiError::internal())?;
    let organization_id: Option<String> = connection
        .query_row(
            "SELECT organization_id FROM repositories WHERE id = ?1 AND report_token_hash = ?2",
            params![repository_id, hash_token(token)],
            |row| row.get(0),
        )
        .optional()
        .map_err(|_| ApiError::internal())?;
    let organization_id = organization_id.ok_or_else(|| {
        ApiError::unauthorized("The report token does not match this repository.")
    })?;
    if let Err(retry) = state
        .limiter
        .check(&format!("upload-org:{organization_id}"))
    {
        return Err(ApiError::rate_limited(retry));
    }
    validate_upload(&upload)?;
    let run_id = random_id("run");
    let capabilities = serde_json::to_string(&upload.report.payload.capabilities)
        .map_err(|_| ApiError::internal())?;
    let languages = serde_json::to_string(&upload.report.payload.languages)
        .map_err(|_| ApiError::internal())?;
    connection.execute(
        "INSERT INTO runs(id, organization_id, repository_id, pull_request, external_run_id, schema_url, report_repository, generated_at, ready, languages_json, capabilities_json, source_digest, algorithm, public_key, signature, received_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16)",
        params![run_id, organization_id, repository_id, upload.pull_request.map(|value| value as i64), upload.external_run_id, upload.report.payload.schema, upload.report.payload.repository, upload.report.payload.generated_at as i64, upload.report.payload.ready as i64, languages, capabilities, upload.report.payload.source_digest, upload.report.algorithm, upload.report.public_key, upload.report.signature, now()],
    ).map_err(|_| ApiError::internal())?;
    Ok((
        StatusCode::CREATED,
        Json(json!({ "id": run_id, "ready": upload.report.payload.ready })),
    ))
}

fn validate_upload(upload: &RunUpload) -> Result<(), ApiError> {
    if upload
        .pull_request
        .is_some_and(|number| number == 0 || number > i64::MAX as u64)
    {
        return Err(ApiError::invalid("pull_request must be a positive integer"));
    }
    if let Some(run_id) = &upload.external_run_id {
        validate_short_text(run_id, 120, "external_run_id")?;
    }
    verify(&upload.report)
        .map_err(|_| ApiError::invalid("The readiness report signature is invalid."))?;
    if upload.report.payload.schema != SCHEMA {
        return Err(ApiError::invalid(
            "The readiness report schema is not supported.",
        ));
    }
    validate_short_text(&upload.report.payload.repository, 120, "report repository")?;
    if upload.report.payload.languages.len() > 10 || upload.report.payload.capabilities.len() > 30 {
        return Err(ApiError::invalid(
            "The readiness report exceeds the capability limits.",
        ));
    }
    for language in &upload.report.payload.languages {
        validate_short_text(language, 80, "language")?;
    }
    for capability in &upload.report.payload.capabilities {
        if !matches!(capability.kind.as_str(), "lsp" | "formatter" | "tests") {
            return Err(ApiError::invalid(
                "The readiness report contains an unknown capability kind.",
            ));
        }
        validate_short_text(&capability.name, 120, "capability name")?;
        validate_short_text(&capability.command, 160, "capability command")?;
        validate_evidence(&capability.evidence)?;
    }
    let digest = upload.report.payload.source_digest.strip_prefix("sha256:");
    if digest.is_none_or(|value| {
        value.len() != 64 || !value.chars().all(|character| character.is_ascii_hexdigit())
    }) {
        return Err(ApiError::invalid(
            "The source inventory digest must be SHA-256.",
        ));
    }
    Ok(())
}

fn validate_short_text(value: &str, maximum: usize, label: &str) -> Result<(), ApiError> {
    if value.trim().is_empty() || value.len() > maximum || value.chars().any(char::is_control) {
        return Err(ApiError::invalid(&format!(
            "{label} must be plain text no longer than {maximum} characters."
        )));
    }
    Ok(())
}

fn validate_evidence(value: &str) -> Result<(), ApiError> {
    validate_short_text(value, 240, "capability evidence")?;
    let lower = value.to_ascii_lowercase();
    let forbidden = [
        "-----begin",
        "password=",
        "password:",
        "secret=",
        "secret:",
        "token=",
        "token:",
        "private_key",
        "private key",
        "source-sentinel",
        "<script",
        "authorization:",
    ];
    if forbidden.iter().any(|marker| lower.contains(marker)) {
        return Err(ApiError::invalid(
            "Capability evidence looks like source or a secret and was not accepted.",
        ));
    }
    Ok(())
}

async fn billing_status() -> Json<Value> {
    Json(json!({
        "available": false,
        "operator_dependency": "Sociobot subscription registration and test-mode entitlement QA",
        "price_minor": SUBSCRIPTION_PRICE_MINOR,
        "currency": "USD",
        "interval": "month",
        "unit": "repository",
        "paid_features": ["private CI checks", "policy templates", "readiness history"]
    }))
}

async fn export_account(
    State(state): State<AppState>,
    Extension(user): Extension<AuthUser>,
) -> Result<Json<Value>, ApiError> {
    let connection = state.db.connection().map_err(|_| ApiError::internal())?;
    let organization = connection.query_row(
        "SELECT name, created_at FROM organizations WHERE id = ?1",
        [&user.organization_id],
        |row| Ok(json!({ "id": user.organization_id, "name": row.get::<_, String>(0)?, "created_at": row.get::<_, i64>(1)? })),
    ).map_err(|_| ApiError::internal())?;
    let repositories = export_rows(
        &connection,
        "SELECT id, owner, name, private, created_at, updated_at FROM repositories WHERE organization_id = ?1 ORDER BY id",
        &user.organization_id,
        |row| {
            Ok(
                json!({ "id": row.get::<_, String>(0)?, "owner": row.get::<_, String>(1)?, "name": row.get::<_, String>(2)?, "private": row.get::<_, i64>(3)? == 1, "created_at": row.get::<_, i64>(4)?, "updated_at": row.get::<_, i64>(5)? }),
            )
        },
    )?;
    let policies = export_rows(
        &connection,
        "SELECT repository_id, required_lsp, required_formatters, required_tests, version, updated_at FROM policies WHERE organization_id = ?1 ORDER BY repository_id",
        &user.organization_id,
        |row| {
            Ok(
                json!({ "repository_id": row.get::<_, String>(0)?, "required_lsp": row.get::<_, i64>(1)? == 1, "required_formatters": row.get::<_, i64>(2)? == 1, "required_tests": row.get::<_, i64>(3)? == 1, "version": row.get::<_, i64>(4)?, "updated_at": row.get::<_, i64>(5)? }),
            )
        },
    )?;
    let runs = export_rows(
        &connection,
        "SELECT id, repository_id, pull_request, external_run_id, generated_at, ready, languages_json, capabilities_json, source_digest, algorithm, public_key, signature, received_at FROM runs WHERE organization_id = ?1 ORDER BY received_at",
        &user.organization_id,
        |row| {
            let languages: Value =
                serde_json::from_str(&row.get::<_, String>(6)?).unwrap_or(Value::Array(vec![]));
            let capabilities: Value =
                serde_json::from_str(&row.get::<_, String>(7)?).unwrap_or(Value::Array(vec![]));
            Ok(
                json!({ "id": row.get::<_, String>(0)?, "repository_id": row.get::<_, String>(1)?, "pull_request": row.get::<_, Option<i64>>(2)?, "external_run_id": row.get::<_, Option<String>>(3)?, "generated_at": row.get::<_, i64>(4)?, "ready": row.get::<_, i64>(5)? == 1, "languages": languages, "capabilities": capabilities, "source_digest": row.get::<_, String>(8)?, "algorithm": row.get::<_, String>(9)?, "public_key": row.get::<_, String>(10)?, "signature": row.get::<_, String>(11)?, "received_at": row.get::<_, i64>(12)? }),
            )
        },
    )?;
    Ok(Json(
        json!({ "exported_at": now(), "organization": organization, "repositories": repositories, "policies": policies, "runs": runs }),
    ))
}

fn export_rows<F>(
    connection: &Connection,
    query: &str,
    organization_id: &str,
    map: F,
) -> Result<Vec<Value>, ApiError>
where
    F: FnMut(&rusqlite::Row<'_>) -> rusqlite::Result<Value>,
{
    let mut statement = connection
        .prepare(query)
        .map_err(|_| ApiError::internal())?;
    statement
        .query_map([organization_id], map)
        .map_err(|_| ApiError::internal())?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|_| ApiError::internal())
}

async fn delete_account(
    State(state): State<AppState>,
    Extension(user): Extension<AuthUser>,
) -> Result<StatusCode, ApiError> {
    ensure_owner(&user)?;
    let connection = state.db.connection().map_err(|_| ApiError::internal())?;
    let changed = connection
        .execute(
            "DELETE FROM organizations WHERE id = ?1",
            [&user.organization_id],
        )
        .map_err(|_| ApiError::internal())?;
    if changed != 1 {
        return Err(ApiError::not_found());
    }
    connection
        .execute(
            "DELETE FROM users WHERE id = ?1 AND NOT EXISTS (SELECT 1 FROM memberships WHERE user_id = ?1)",
            [&user.user_id],
        )
        .map_err(|_| ApiError::internal())?;
    Ok(StatusCode::NO_CONTENT)
}

fn ensure_repository(
    db: &Database,
    organization_id: &str,
    repository_id: &str,
) -> Result<(), ApiError> {
    let connection = db.connection().map_err(|_| ApiError::internal())?;
    let found = connection
        .query_row(
            "SELECT 1 FROM repositories WHERE organization_id = ?1 AND id = ?2",
            params![organization_id, repository_id],
            |_| Ok(()),
        )
        .optional()
        .map_err(|_| ApiError::internal())?;
    found.ok_or_else(ApiError::not_found)
}

fn ensure_owner(user: &AuthUser) -> Result<(), ApiError> {
    if user.role == "owner" {
        Ok(())
    } else {
        Err(ApiError::new(
            StatusCode::FORBIDDEN,
            "owner_required",
            "Only an organization owner can do that.",
        ))
    }
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct TestRepositoryInput {
    owner: String,
    name: String,
    private: bool,
}

async fn seed_test_repository(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(input): Json<TestRepositoryInput>,
) -> Result<(StatusCode, Json<Value>), ApiError> {
    if !matches!(state.auth, Authenticator::Debug) {
        return Err(ApiError::not_found());
    }
    let token = bearer_token(&headers)?;
    let identity = state.auth.authenticate(token).await?;
    let user = upsert_identity(&state.db, &identity).map_err(|_| ApiError::internal())?;
    validate_short_text(&input.owner, 100, "repository owner")?;
    validate_short_text(&input.name, 100, "repository name")?;
    let id = random_id("repo");
    let report_token = random_token();
    let connection = state.db.connection().map_err(|_| ApiError::internal())?;
    connection.execute(
        "INSERT INTO repositories(id, organization_id, github_repository_id, owner, name, private, report_token_hash, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?8)",
        params![id, user.organization_id, random_i63(), input.owner, input.name, input.private as i64, hash_token(&report_token), now()],
    ).map_err(|_| ApiError::internal())?;
    Ok((
        StatusCode::CREATED,
        Json(json!({ "id": id, "report_token": report_token })),
    ))
}

#[derive(Debug)]
pub struct ApiError {
    status: StatusCode,
    code: &'static str,
    message: String,
    retry_after: Option<u64>,
}

impl ApiError {
    fn new(status: StatusCode, code: &'static str, message: impl Into<String>) -> Self {
        Self {
            status,
            code,
            message: message.into(),
            retry_after: None,
        }
    }

    fn invalid(message: &str) -> Self {
        Self::new(StatusCode::BAD_REQUEST, "invalid_request", message)
    }

    fn unauthorized(message: &str) -> Self {
        Self::new(StatusCode::UNAUTHORIZED, "unauthorized", message)
    }

    fn not_found() -> Self {
        Self::new(
            StatusCode::NOT_FOUND,
            "not_found",
            "The requested record was not found.",
        )
    }

    fn dependency(code: &'static str, message: &str) -> Self {
        Self::new(StatusCode::SERVICE_UNAVAILABLE, code, message)
    }

    fn internal() -> Self {
        Self::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "internal_error",
            "The request could not be completed. Try again.",
        )
    }

    fn rate_limited(seconds: u64) -> Self {
        let mut error = Self::new(
            StatusCode::TOO_MANY_REQUESTS,
            "rate_limit",
            "Too many requests. Wait before trying again.",
        );
        error.retry_after = Some(seconds);
        error
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let mut response = (
            self.status,
            Json(json!({ "error": self.code, "message": self.message })),
        )
            .into_response();
        if let Some(seconds) = self.retry_after {
            response.headers_mut().insert(
                header::RETRY_AFTER,
                HeaderValue::from_str(&seconds.to_string())
                    .unwrap_or(HeaderValue::from_static("1")),
            );
        }
        response
    }
}

pub fn down_migration(connection: &Connection) -> anyhow::Result<()> {
    connection.execute_batch(MIGRATION_DOWN)?;
    Ok(())
}

fn now() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs() as i64
}

fn random_token() -> String {
    let mut bytes = [0_u8; 32];
    OsRng.fill_bytes(&mut bytes);
    format!("lrk_{}", URL_SAFE_NO_PAD.encode(bytes))
}

fn random_id(prefix: &str) -> String {
    let mut bytes = [0_u8; 12];
    OsRng.fill_bytes(&mut bytes);
    format!("{prefix}_{}", URL_SAFE_NO_PAD.encode(bytes))
}

fn random_i63() -> i64 {
    let mut bytes = [0_u8; 8];
    OsRng.fill_bytes(&mut bytes);
    (u64::from_le_bytes(bytes) & i64::MAX as u64) as i64
}

fn stable_id(prefix: &str, value: &str) -> String {
    let digest = Sha256::digest(value.as_bytes());
    format!("{prefix}_{}", URL_SAFE_NO_PAD.encode(&digest[..12]))
}

fn hash_token(token: &str) -> String {
    format!("sha256:{:x}", Sha256::digest(token.as_bytes()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn migrations_apply_and_reverse() {
        let directory = tempfile::tempdir().unwrap();
        let database = Database::open(directory.path().join("test.db")).unwrap();
        assert_eq!(database.health().unwrap(), 1);
        let connection = database.connection().unwrap();
        down_migration(&connection).unwrap();
        let tables: i64 = connection
            .query_row(
                "SELECT COUNT(*) FROM sqlite_master WHERE type = 'table' AND name IN ('users', 'organizations', 'repositories', 'runs')",
                [],
                |row| row.get(0),
            )
            .unwrap();
        assert_eq!(tables, 0);
    }

    #[test]
    fn sqlite_survives_restart_and_backup_restore() {
        let directory = tempfile::tempdir().unwrap();
        let path = directory.path().join("service.db");
        {
            let database = Database::open(&path).unwrap();
            let identity = Identity {
                subject: "restart-user".into(),
                name: Some("Restart User".into()),
                email: None,
            };
            upsert_identity(&database, &identity).unwrap();
            database
                .backup(&directory.path().join("backup.db"))
                .unwrap();
        }
        let reopened = Database::open(&path).unwrap();
        let count: i64 = reopened
            .connection()
            .unwrap()
            .query_row("SELECT COUNT(*) FROM organizations", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 1);
        let restored = directory.path().join("restored.db");
        Database::restore(&directory.path().join("backup.db"), &restored).unwrap();
        let restored = Database::open(restored).unwrap();
        assert_eq!(
            restored
                .connection()
                .unwrap()
                .query_row("SELECT COUNT(*) FROM organizations", [], |row| row
                    .get::<_, i64>(0))
                .unwrap(),
            1
        );
    }

    #[test]
    fn source_and_secret_markers_are_rejected() {
        assert!(validate_evidence("source-sentinel: export const secret = true").is_err());
        assert!(validate_evidence("token=abc123").is_err());
        assert!(validate_evidence("42 tests passed").is_ok());
    }

    #[test]
    fn release_build_cannot_enable_test_auth() {
        if !cfg!(debug_assertions) {
            assert!(!ServiceConfig::from_env().unwrap().allow_test_auth);
        }
    }
}
