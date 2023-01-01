mod custom_header;
mod mirror_body_json;
mod mirror_body_string;
mod path_variables;
mod query_params;
mod root_route;
mod standard_header;

use axum::{
    body::Body,
    http::Method,
    routing::{get, post},
    Router,
};
use custom_header::custom_header;
use mirror_body_json::mirror_body_json;
use mirror_body_string::mirror_body_string;
use path_variables::path_variables;
use query_params::query_params;
use standard_header::standard_header;
use tower_http::cors::{Any, CorsLayer};

pub fn create_routes() -> Router<Body> {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    Router::new()
        .route("/", get(root_route::hello_world))
        .route("/mirror_body_string", post(mirror_body_string))
        .route("/mirror_body_json", post(mirror_body_json))
        .route("/path_variables/:id", get(path_variables))
        .route("/query_params", get(query_params))
        .route("/standard_header_user_agent", get(standard_header))
        .route("/custom_header", get(custom_header))
        .layer(cors)
}
