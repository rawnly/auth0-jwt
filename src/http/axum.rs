use async_trait::async_trait;
use axum::{extract::FromRequestParts, http::request::Parts, RequestPartsExt, TypedHeader};
use headers::{authorization::Bearer, Authorization};
use serde::de::DeserializeOwned;

use crate::{claims::Claims, error::Error, Config};

#[async_trait]
impl<T, S> FromRequestParts<S> for Claims<T>
where
    S: Sync,
    T: DeserializeOwned,
{
    type Rejection = Error;

    async fn from_request_parts(
        parts: &mut Parts,
        _: &S,
    ) -> std::result::Result<Self, Self::Rejection> {
        let issuer = std::env::var("AUTH0_ISSUER").expect("AUTH0_ISSUER must be set");

        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(Error::InvalidTokenHeader)?;

        let token = bearer.token();
        let config = Config::new(&issuer);
        let claims = Self::from_token(token, config).await?;

        Ok(Claims(claims))
    }
}

pub struct Token(pub String);

#[async_trait]
impl<S> FromRequestParts<S> for Token
where
    S: Sync,
{
    type Rejection = Error;

    async fn from_request_parts(
        parts: &mut Parts,
        _: &S,
    ) -> std::result::Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(Error::InvalidTokenHeader)?;

        Ok(Token(bearer.token().to_string()))
    }
}
