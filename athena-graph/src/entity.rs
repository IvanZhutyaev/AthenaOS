use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeId(pub Uuid);

impl NodeId {
    pub fn new() -> Self {
        NodeId(Uuid::new_v4())
    }

    pub fn from_uuid(uuid: Uuid) -> Self {
        NodeId(uuid)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub id: NodeId,
    pub label: String,
    pub properties: HashMap<String, PropertyValue>,
    pub created_at: i64,
    pub updated_at: i64,
    pub version: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum PropertyValue {
    String(String),
    Number(f64),
    Boolean(bool),
    DateTime(i64),
    Reference(NodeId),
    List(Vec<PropertyValue>),
    Map(HashMap<String, PropertyValue>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edge {
    pub id: Uuid,
    pub from: NodeId,
    pub to: NodeId,
    pub label: String,
    pub properties: HashMap<String, PropertyValue>,
    pub created_at: i64,
    pub version: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphUpdate {
    pub nodes: Vec<Entity>,
    pub edges: Vec<Edge>,
    pub deleted_nodes: Vec<NodeId>,
    pub deleted_edges: Vec<Uuid>,
}

