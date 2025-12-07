use crate::entity::{Edge, Entity, NodeId};
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryResult {
    pub nodes: Vec<Entity>,
    pub edges: Vec<Edge>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphPattern {
    pub node_filters: Vec<NodeFilter>,
    pub edge_filters: Vec<EdgeFilter>,
    pub limit: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeFilter {
    pub property: String,
    pub operator: FilterOperator,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeFilter {
    pub from: Option<NodeId>,
    pub to: Option<NodeId>,
    pub label: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FilterOperator {
    Equals,
    Contains,
    StartsWith,
    EndsWith,
    GreaterThan,
    LessThan,
}

pub trait GraphQuery {
    fn query(&self, pattern: &GraphPattern) -> Result<QueryResult>;
    fn find_node_by_label(&self, label: &str) -> Result<Vec<Entity>>;
    fn find_edges(&self, from: Option<&NodeId>, to: Option<&NodeId>) -> Result<Vec<Edge>>;
}

