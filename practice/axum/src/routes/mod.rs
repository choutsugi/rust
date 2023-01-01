mod hello_world;
mod middleware_message;
mod mirror_body_json;
mod mirror_body_string;
mod mirror_custom_header;
mod mirror_user_agent;
mod path_variables;
mod query_params;
mod read_middleware_custom_header;
mod set_middleware_custom_header;

use axum::{
    body::Body,
    http::Method,
    middleware,
    routing::{get, post},
    Extension, Router,
};
use hello_world::hello_world;
use middleware_message::middleware_message;
use mirror_body_json::mirror_body_json;
use mirror_body_string::mirror_body_string;
use mirror_custom_header::mirror_custom_header;
use mirror_user_agent::mirror_user_agent;
use path_variables::path_variables;
use query_params::query_params;
use read_middleware_custom_header::read_middleware_custom_header;

use self::set_middleware_custom_header::set_middleware_custom_header;

use tower_http::cors::{Any, CorsLayer};

#[derive(Clone)]
pub struct ShareData {
    pub message: String,
}

pub fn create_routes() -> Router<Body> {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST])
        .allow_origin(Any);

    let share_data = ShareData {
        message: "Hello from share data".to_owned(),
    };

    Router::new()
        .route(
            "/read_middleware_custom_header",
            get(read_middleware_custom_header),
        )
        .route_layer(middleware::from_fn(set_middleware_custom_header))
        .route("/", get(hello_world))
        .route("/mirror_body_string", post(mirror_body_string))
        .route("/mirror_body_json", post(mirror_body_json))
        .route("/path_variables/:id", get(path_variables))
        .route("/query_params", get(query_params))
        .route("/mirror_user_agent", get(mirror_user_agent))
        .route("/mirror_custom_header", get(mirror_custom_header))
        .route("/middleware_message", get(middleware_message))
        .layer(cors)
        .layer(Extension(share_data))
}
