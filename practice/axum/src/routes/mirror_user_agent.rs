use axum::{headers::UserAgent, TypedHeader};

pub async fn mirror_user_agent(TypedHeader(agent): TypedHeader<UserAgent>) -> String {
    agent.to_string()
}
