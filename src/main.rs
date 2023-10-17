use std::{net::SocketAddr, path::Path};

use axum::{
    routing::{get, post},
    Form, Router, Server,
};
use serde::Deserialize;
use tower_http::services::ServeDir;

mod templates;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .nest_service("/assets", ServeDir::new(Path::new("public")))
        .route("/", get(index))
        .route("/lap", post(lap));

    let addr = SocketAddr::from(([127, 0, 0, 1], 42069));

    println!("Starting server on: http://{}", addr);

    Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index() -> templates::Index {
    templates::Index {}
}

#[derive(Deserialize)]
struct LapRequest {
    time: u32,
}

async fn lap(Form(lap_request): Form<LapRequest>) -> templates::Lap {
    templates::Lap::new(0, lap_request.time)
}
