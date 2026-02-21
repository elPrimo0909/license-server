use axum::{routing::{get, post}, Json, Router};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[derive(Deserialize)]
struct ActivateRequest {
    key: String,
    #[serde(rename = "machineId")]
    machine_id: String,
}

#[derive(Serialize)]
struct ActivateResponse {
    success: bool,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<LicenseData>,
}

#[derive(Serialize)]
struct LicenseData {
    expires: Option<String>,
    features: Vec<String>,
}

async fn activate(Json(payload): Json<ActivateRequest>) -> Json<ActivateResponse> {
    // Simple validation: if key length >= 8, activate
    if payload.key.len() >= 8 {
        Json(ActivateResponse {
            success: true,
            message: "Licencia activada exitosamente".to_string(),
            data: Some(LicenseData {
                expires: None, // No expiration
                features: vec!["pro".to_string(), "unlimited_files".to_string()],
            }),
        })
    } else {
        Json(ActivateResponse {
            success: false,
            message: "Clave de licencia inválida".to_string(),
            data: None,
        })
    }
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(|| async { "ok" }))
        .route("/salt", get(|| async { "salt-endpoint-ok" }))
        .route("/activate", post(activate));

    let port: u16 = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("PORT must be a number");

    let addr = SocketAddr::from(([0, 0, 0, 0], port));

    println!("License server listening on {}", addr);

    let listener = TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}
