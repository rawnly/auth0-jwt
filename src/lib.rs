//! Copyright (C) 2022 Federico Vitale
//!
//! Implements a library to iteract and decode JWTs
//! generated from auth0
//!
//! ## Usage Example (via Axum)
//! > with `axum` and `claims` features enabled
//!
//!
//! ```no_run
//! #[cfg(all(feature = "claims", feature = "claims"))]
//! use auth0_jwt::claims::Claims;
//! #[cfg(all(feature = "claims", feature = "claims"))]
//! use axum::response::IntoResponse;
//!
//! struct ClaimsContent {
//!     exp: usize,
//!     iat: usize,
//! }
//!
//! // Using the `Claims` struct lets you decode automatically the value into a struct.
//! #[cfg(all(feature = "claims", feature = "claims"))]
//! async fn handler(Claims(claims): Claims<ClaimsContent>) -> impl IntoResponse {
//!     // your claims
//!     println!("Exp: {}, Iat: {}", claims.exp, claims.iat);
//! }
//! ```

use alcoholic_jwt::{token_kid, validate};
pub use alcoholic_jwt::{Validation, JWKS};

pub mod error;

/// Includes feature flagged adapters for various
/// http servers such as `axum`, `actix-web`, `rocket` etc
pub mod http;

#[cfg(feature = "claims")]
pub mod claims;

mod util;

use crate::error::{Error, Result};

/// Configuration for the JWT params
pub struct Config {
    pub issuer: String,
    pub validations: Vec<Validation>,
}

impl Config {
    /// Initialize the `Config` with issuer and default validations.
    /// Default validations are: NotExpired, Issuer
    pub fn new(issuer: &str) -> Self {
        Self {
            issuer: issuer.to_string(),
            validations: vec![
                Validation::Issuer(issuer.to_string()),
                Validation::NotExpired,
            ],
        }
    }
}

/// Returns the claims stored into the given JWT
pub async fn get_claims(token: &str, config: Config) -> Result<serde_json::Value> {
    let jwks = util::fetch_jwks(&config.issuer).await?;
    let kid = token_kid(token)
        .map_err(Error::ValidationError)?
        .expect("Failed to decode token kid");

    let jwk = jwks.find(&kid).expect("Specified key not found in set");

    // validate or throw error
    let res = validate(token, jwk, config.validations).map_err(Error::ValidationError)?;

    Ok(res.claims)
}
