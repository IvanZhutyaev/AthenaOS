use athena_core::system::AthenaSystem;
use athena_graph::entity::{Edge, Entity, NodeId};
use athena_graph::query::GraphPattern;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Clone)]
pub struct ApiHandlers {
    pub system: Arc<AthenaSystem>,
}

#[derive(Serialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
}

pub async fn health_handler() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        version: "0.1.0".to_string(),
    })
}

#[derive(Serialize)]
pub struct NodeListResponse {
    pub nodes: Vec<Entity>,
}

pub async fn list_nodes(State(handlers): State<Arc<ApiHandlers>>) -> Result<Json<NodeListResponse>, StatusCode> {
    let pattern = GraphPattern {
        node_filters: vec![],
        edge_filters: vec![],
        limit: Some(100),
    };

    let result = handlers
        .system
        .graph_engine
        .query(&pattern)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(NodeListResponse {
        nodes: result.nodes,
    }))
}

#[derive(Deserialize)]
pub struct CreateNodeRequest {
    pub label: String,
    pub properties: Option<std::collections::HashMap<String, athena_graph::entity::PropertyValue>>,
}

pub async fn create_node(
    State(handlers): State<Arc<ApiHandlers>>,
    Json(request): Json<CreateNodeRequest>,
) -> Result<Json<Entity>, StatusCode> {
    let entity = Entity {
        id: NodeId::new(),
        label: request.label,
        properties: request.properties.unwrap_or_default(),
        created_at: chrono::Utc::now().timestamp(),
        updated_at: chrono::Utc::now().timestamp(),
        version: 1,
    };

    handlers
        .system
        .graph_engine
        .put_node(entity.clone())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(entity))
}

pub async fn get_node(
    State(handlers): State<Arc<ApiHandlers>>,
    Path(id): Path<String>,
) -> Result<Json<Entity>, StatusCode> {
    let uuid = Uuid::parse_str(&id).map_err(|_| StatusCode::BAD_REQUEST)?;
    let node_id = NodeId::from_uuid(uuid);

    let node = handlers
        .system
        .graph_engine
        .get_node(&node_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    Ok(Json(node))
}

pub async fn delete_node(
    State(handlers): State<Arc<ApiHandlers>>,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    let uuid = Uuid::parse_str(&id).map_err(|_| StatusCode::BAD_REQUEST)?;
    let node_id = NodeId::from_uuid(uuid);

    handlers
        .system
        .graph_engine
        .delete_node(&node_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::NO_CONTENT)
}

#[derive(Serialize)]
pub struct EdgeListResponse {
    pub edges: Vec<Edge>,
}

pub async fn list_edges(State(handlers): State<Arc<ApiHandlers>>) -> Result<Json<EdgeListResponse>, StatusCode> {
    let pattern = GraphPattern {
        node_filters: vec![],
        edge_filters: vec![],
        limit: Some(100),
    };

    let result = handlers
        .system
        .graph_engine
        .query(&pattern)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(EdgeListResponse {
        edges: result.edges,
    }))
}

#[derive(Deserialize)]
pub struct CreateEdgeRequest {
    pub from: String,
    pub to: String,
    pub label: String,
}

pub async fn create_edge(
    State(handlers): State<Arc<ApiHandlers>>,
    Json(request): Json<CreateEdgeRequest>,
) -> Result<Json<Edge>, StatusCode> {
    let from_uuid = Uuid::parse_str(&request.from).map_err(|_| StatusCode::BAD_REQUEST)?;
    let to_uuid = Uuid::parse_str(&request.to).map_err(|_| StatusCode::BAD_REQUEST)?;

    let edge = Edge {
        id: Uuid::new_v4(),
        from: NodeId::from_uuid(from_uuid),
        to: NodeId::from_uuid(to_uuid),
        label: request.label,
        properties: std::collections::HashMap::new(),
        created_at: chrono::Utc::now().timestamp(),
        version: 1,
    };

    handlers
        .system
        .graph_engine
        .put_edge(edge.clone())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(edge))
}

#[derive(Deserialize)]
pub struct QueryRequest {
    pub pattern: GraphPattern,
}

pub async fn query_graph(
    State(handlers): State<Arc<ApiHandlers>>,
    Json(request): Json<QueryRequest>,
) -> Result<Json<athena_graph::query::QueryResult>, StatusCode> {
    let result = handlers
        .system
        .graph_engine
        .query(&request.pattern)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(result))
}

#[derive(Serialize)]
pub struct AgentListResponse {
    pub agents: Vec<Uuid>,
}

pub async fn list_agents(State(handlers): State<Arc<ApiHandlers>>) -> Result<Json<AgentListResponse>, StatusCode> {
    let runtime = handlers.system.agent_runtime.read().await;
    let agents = runtime.list_agents().await;

    Ok(Json(AgentListResponse { agents }))
}

#[derive(Deserialize)]
pub struct LoadAgentRequest {
    pub wasm_module: Vec<u8>,
    pub metadata: athena_agents::metadata::AgentMetadata,
}

pub async fn load_agent(
    State(handlers): State<Arc<ApiHandlers>>,
    Json(request): Json<LoadAgentRequest>,
) -> Result<Json<Uuid>, StatusCode> {
    let agent = athena_agents::agent::AthenaAgent::new(request.metadata, request.wasm_module);
    let id = agent.metadata.id;

    handlers
        .system
        .agent_runtime
        .write()
        .await
        .load_agent(agent)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(id))
}

pub async fn unload_agent(
    State(handlers): State<Arc<ApiHandlers>>,
    Path(id): Path<String>,
) -> Result<StatusCode, StatusCode> {
    let uuid = Uuid::parse_str(&id).map_err(|_| StatusCode::BAD_REQUEST)?;

    handlers
        .system
        .agent_runtime
        .write()
        .await
        .unload_agent(&uuid)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::NO_CONTENT)
}

