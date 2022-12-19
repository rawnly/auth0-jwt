use crate::error::{Error, Result};
use crate::{get_claims, Config};
use serde::de::DeserializeOwned;

pub struct Claims<T: DeserializeOwned>(pub T);

impl<T> Claims<T>
where
    T: DeserializeOwned,
{
    pub async fn from_token(token: &str, config: Config) -> Result<T> {
        let value = get_claims(token, config).await?;
        let claims = serde_json::from_value::<T>(value).map_err(Error::DeserializationError)?;

        // since the token is valid we return true
        Ok(claims)
    }
}
