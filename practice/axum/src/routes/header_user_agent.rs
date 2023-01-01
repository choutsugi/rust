use axum::{headers::UserAgent, TypedHeader};

pub async fn header_user_agent(TypedHeader(agent): TypedHeader<UserAgent>) -> String {
    agent.to_string()
}
