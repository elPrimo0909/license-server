use axum::{routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // Rutas mínimas
    let app = Router::new()
        .route("/health", get(|| async { "ok" }))
        .route("/salt", get(|| async { "salt-endpoint-ok" }));

    // Render define el puerto en la variable PORT
    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a number");

    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    println!("License server listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
