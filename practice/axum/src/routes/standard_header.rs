use axum::{headers::UserAgent, TypedHeader};

pub async fn standard_header(TypedHeader(agent): TypedHeader<UserAgent>) -> String {
    agent.to_string()
}
