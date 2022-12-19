use crate::error::{Error, Result};
use alcoholic_jwt::JWKS;

pub fn jwks_uri(issuer: &str) -> String {
    let suffix = ".well-known/jwks.json";

    if issuer.ends_with("/") {
        return format!("{}{}", issuer, suffix);
    }

    format!("{}/{}", issuer, suffix)
}

pub async fn fetch_jwks(issuer: &str) -> Result<JWKS> {
    let uri = jwks_uri(issuer);

    let res = reqwest::get(&uri).await.map_err(Error::ReqwestError)?;
    let val = res.json::<JWKS>().await.map_err(Error::ReqwestError)?;

    Ok(val)
}

#[cfg(test)]
mod tests {
    use super::jwks_uri;

    #[test]
    fn test_jwks_uri() {
        let result = jwks_uri("https://acme-website.org");
        assert_eq!(result, "https://acme-website.org/.well-known/jwks.json");
    }

    #[test]
    fn test_jwks_uri_with_double_slash() {
        let result = jwks_uri("https://acme-website.org/");
        assert_eq!(result, "https://acme-website.org/.well-known/jwks.json");
    }
}
