use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;

use crate::handlers::*;

pub fn create_router(handlers: Arc<ApiHandlers>) -> Router {
    Router::new()
        .route("/api/v1/health", get(health_handler))
        .route("/api/v1/nodes", get(list_nodes).post(create_node))
        .route("/api/v1/nodes/:id", get(get_node).delete(delete_node))
        .route("/api/v1/edges", get(list_edges).post(create_edge))
        .route("/api/v1/query", post(query_graph))
        .route("/api/v1/agents", get(list_agents).post(load_agent))
        .route("/api/v1/agents/:id", delete(unload_agent))
        .with_state(handlers)
}

