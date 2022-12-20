use auth0_jwt::claims::Claims;
use axum::{response::IntoResponse, routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(root));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// just use the given extractor
async fn root(Claims(claims): Claims<ClaimsContent>) -> impl IntoResponse {
    Json(claims)
}

// the output to our `create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}

#[derive(Deserialize, Serialize)]
struct ClaimsContent {
    iat: usize,
    exp: usize,
    aud: String,
}
