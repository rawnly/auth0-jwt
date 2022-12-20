# Auth0 JWT
> Auth0 utility to check if the given JWT is valid.

## Usage
```rust
use auth0_jwt::get_claims;

#[tokio::async]
async fn main() {
  let token = env!("JWT_TOKEN");
  let claims = get_claims(&token).unwrap();

  println!("Claims {}", claims);
}
```

## Features 
- `claims` - Exports the `Claims` struct which is useful for deserialization. 
- `axum` - Implements the `FromRequestParts<T>` trait for `Claims` and provides a `Token(String)` extractor for convenience.

## Axum Example 
Detailed example [here](./examples/axum-hello-world)

```rust
use auth0_jwt::claims::Claims;
use axum::{response::IntoResponse, routing::get, Json, Router};
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    dotenv().ok();

    // build our application with a route
    let app = Router::new().route("/", get(handler));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Serialize)]
struct ResponseBody {
    message: &'static str,
}

#[derive(Deserialize, Serialize)]
struct ClaimsContent {
    pub exp: usize,
    pub iat: usize
}

async fn handler(Claims(claims): Claims<ClaimsContent>) -> impl IntoResponse {
    println!("{:?}", claims.exp);

    Json(ResponseBody {
        message: "hello world",
    })
}
```
