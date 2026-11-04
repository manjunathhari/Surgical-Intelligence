use axum::{
    extract::State,
    http::{header, HeaderValue, Method, StatusCode},
    response::IntoResponse,
    routing::{get, put},
    Json, Router,
};
use serde::Serialize;
use si_core::PortableStateV2;
use std::{env, net::SocketAddr, path::PathBuf, sync::Arc};
use tokio::sync::RwLock;
use tower_http::cors::CorsLayer;

#[derive(Clone)]
struct AppState {
    path: PathBuf,
    state: Arc<RwLock<PortableStateV2>>,
}

#[derive(Serialize)]
struct Health {
    status: &'static str,
    engine: &'static str,
    state_path: String,
}

#[derive(Serialize)]
struct Summary {
    format: String,
    version: u32,
    nodes: usize,
    edges: usize,
    learning_records: usize,
    publications: usize,
    evidence_claims: usize,
    source_artifacts: usize,
    events: usize,
    embedding_profiles: usize,
}

#[tokio::main]
async fn main() {
    let path = env::var_os("SI_STATE_PATH")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("./data/state-v2.json"));
    let state = load_or_create(&path).unwrap_or_else(|error| {
        eprintln!("Cannot load Surgical Intelligence state: {error}");
        std::process::exit(1);
    });
    let app_state = AppState { path: path.clone(), state: Arc::new(RwLock::new(state)) };

    let cors = CorsLayer::new()
        .allow_origin([
            HeaderValue::from_static("http://localhost:5173"),
            HeaderValue::from_static("http://127.0.0.1:5173"),
        ])
        .allow_methods([Method::GET, Method::PUT, Method::OPTIONS])
        .allow_headers([header::CONTENT_TYPE]);

    let app = Router::new()
        .route("/api/health", get(health))
        .route("/api/state", get(get_state).put(put_state))
        .route("/api/summary", get(summary))
        .route("/api/validate", get(validate))
        .with_state(app_state)
        .layer(cors);

    let address = SocketAddr::from(([127, 0, 0, 1], 8787));
    println!("Surgical Intelligence API listening at http://{address}");
    println!("Portable state: {}", path.display());
    let listener = tokio::net::TcpListener::bind(address).await.expect("bind local API");
    axum::serve(listener, app).await.expect("serve local API");
}

fn load_or_create(path: &PathBuf) -> Result<PortableStateV2, Box<dyn std::error::Error>> {
    if path.exists() {
        let raw = std::fs::read_to_string(path)?;
        Ok(PortableStateV2::from_json(&raw).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?)
    } else {
        if let Some(parent) = path.parent() {
            if !parent.as_os_str().is_empty() { std::fs::create_dir_all(parent)?; }
        }
        let state = PortableStateV2::empty();
        std::fs::write(path, state.to_json().map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?)?;
        Ok(state)
    }
}

async fn health(State(app): State<AppState>) -> Json<Health> {
    Json(Health { status: "ok", engine: "si-core", state_path: app.path.display().to_string() })
}

async fn get_state(State(app): State<AppState>) -> Json<PortableStateV2> {
    Json(app.state.read().await.clone())
}

async fn summary(State(app): State<AppState>) -> Json<Summary> {
    let state = app.state.read().await;
    Json(Summary {
        format: state.format.clone(),
        version: state.version,
        nodes: state.graph.nodes.len(),
        edges: state.graph.edges.len(),
        learning_records: state.learning_records.len(),
        publications: state.publications.len(),
        evidence_claims: state.evidence_claims.len(),
        source_artifacts: state.source_artifacts.len(),
        events: state.events.len(),
        embedding_profiles: state.embedding_profiles.len(),
    })
}

async fn validate(State(app): State<AppState>) -> impl IntoResponse {
    let state = app.state.read().await;
    match state.validate() {
        Ok(()) => (StatusCode::OK, Json(serde_json::json!({"valid": true, "message": "Portable state passes si-core validation"}))),
        Err(error) => (StatusCode::UNPROCESSABLE_ENTITY, Json(serde_json::json!({"valid": false, "message": error.to_string()}))),
    }
}

async fn put_state(State(app): State<AppState>, Json(candidate): Json<PortableStateV2>) -> impl IntoResponse {
    if let Err(error) = candidate.validate() {
        return (StatusCode::UNPROCESSABLE_ENTITY, Json(serde_json::json!({"saved": false, "message": error.to_string()})));
    }
    let json = match candidate.to_json() {
        Ok(json) => json,
        Err(error) => return (StatusCode::UNPROCESSABLE_ENTITY, Json(serde_json::json!({"saved": false, "message": error.to_string()}))),
    };
    let temp = app.path.with_extension("json.tmp");
    if let Some(parent) = app.path.parent() {
        if !parent.as_os_str().is_empty() {
            if let Err(error) = std::fs::create_dir_all(parent) {
                return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"saved": false, "message": error.to_string()})));
            }
        }
    }
    if let Err(error) = std::fs::write(&temp, json).and_then(|_| std::fs::rename(&temp, &app.path)) {
        return (StatusCode::INTERNAL_SERVER_ERROR, Json(serde_json::json!({"saved": false, "message": error.to_string()})));
    }
    *app.state.write().await = candidate;
    (StatusCode::OK, Json(serde_json::json!({"saved": true, "message": "Validated portable state saved"})))
}
