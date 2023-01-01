mod root_route;

use axum::{body::Body, routing::get, Router};

pub fn create_routes() -> Router<Body> {
    Router::new().route("/", get(root_route::hello_world))
}
