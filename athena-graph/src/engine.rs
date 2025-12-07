use crate::entity::{Edge, Entity, GraphUpdate, NodeId};
use crate::query::{GraphPattern, GraphQuery, QueryResult};
use crate::storage::GraphStorage;
use crate::version::{Checkpoint, VersionId};
use anyhow::Result;
use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::RwLock;

#[async_trait]
pub trait GraphEngine: Send + Sync {
    async fn query(&self, pattern: &GraphPattern) -> Result<QueryResult>;
    async fn update(&self, update: &GraphUpdate) -> Result<VersionId>;
    async fn subscribe(&self, pattern: &GraphPattern) -> Result<tokio::sync::mpsc::Receiver<GraphUpdate>>;
    async fn checkpoint(&self) -> Result<Checkpoint>;
    async fn get_node(&self, id: &NodeId) -> Result<Option<Entity>>;
    async fn put_node(&self, entity: Entity) -> Result<()>;
    async fn delete_node(&self, id: &NodeId) -> Result<()>;
    async fn get_edge(&self, id: &uuid::Uuid) -> Result<Option<Edge>>;
    async fn put_edge(&self, edge: Edge) -> Result<()>;
    async fn delete_edge(&self, id: &uuid::Uuid) -> Result<()>;
}

pub struct DefaultGraphEngine {
    storage: Arc<GraphStorage>,
    version: Arc<RwLock<VersionId>>,
}

impl DefaultGraphEngine {
    pub fn new(storage: GraphStorage) -> Self {
        Self {
            storage: Arc::new(storage),
            version: Arc::new(RwLock::new(VersionId::new())),
        }
    }
}

#[async_trait]
impl GraphEngine for DefaultGraphEngine {
    async fn query(&self, pattern: &GraphPattern) -> Result<QueryResult> {
        let mut nodes = Vec::new();
        let mut edges = Vec::new();

        // Simple implementation - iterate all nodes and filter
        for node_result in self.storage.iter_nodes() {
            let node = node_result?;
            let mut matches = true;

            for filter in &pattern.node_filters {
                match filter.property.as_str() {
                    "label" => {
                        matches = match filter.operator {
                            crate::query::FilterOperator::Equals => node.label == filter.value,
                            crate::query::FilterOperator::Contains => node.label.contains(&filter.value),
                            crate::query::FilterOperator::StartsWith => node.label.starts_with(&filter.value),
                            _ => false,
                        };
                    }
                    _ => {
                        if let Some(prop_value) = node.properties.get(&filter.property) {
                            matches = match filter.operator {
                                crate::query::FilterOperator::Equals => {
                                    format!("{:?}", prop_value) == filter.value
                                }
                                _ => false,
                            };
                        } else {
                            matches = false;
                        }
                    }
                }
                if !matches {
                    break;
                }
            }

            if matches {
                nodes.push(node);
            }

            if let Some(limit) = pattern.limit {
                if nodes.len() >= limit {
                    break;
                }
            }
        }

        // Find edges
        for edge_result in self.storage.iter_edges() {
            let edge = edge_result?;
            let mut matches = true;

            for filter in &pattern.edge_filters {
                if let Some(ref from) = filter.from {
                    if &edge.from != from {
                        matches = false;
                        break;
                    }
                }
                if let Some(ref to) = filter.to {
                    if &edge.to != to {
                        matches = false;
                        break;
                    }
                }
                if let Some(ref label) = filter.label {
                    if &edge.label != label {
                        matches = false;
                        break;
                    }
                }
            }

            if matches {
                edges.push(edge);
            }
        }

        Ok(QueryResult { nodes, edges })
    }

    async fn update(&self, update: &GraphUpdate) -> Result<VersionId> {
        let mut version = self.version.write().await;

        // Apply updates
        for node in &update.nodes {
            self.storage.put_node(node)?;
        }

        for edge in &update.edges {
            self.storage.put_edge(edge)?;
        }

        for node_id in &update.deleted_nodes {
            self.storage.delete_node(node_id)?;
        }

        for edge_id in &update.deleted_edges {
            self.storage.delete_edge(edge_id)?;
        }

        *version = version.next();
        Ok(*version)
    }

    async fn subscribe(&self, _pattern: &GraphPattern) -> Result<tokio::sync::mpsc::Receiver<GraphUpdate>> {
        // Simple implementation - return empty channel
        let (tx, rx) = tokio::sync::mpsc::channel(100);
        drop(tx); // Close sender immediately
        Ok(rx)
    }

    async fn checkpoint(&self) -> Result<Checkpoint> {
        let version = *self.version.read().await;
        Ok(Checkpoint {
            id: uuid::Uuid::new_v4(),
            version,
            timestamp: chrono::Utc::now().timestamp(),
            hash: vec![], // TODO: compute hash
        })
    }

    async fn get_node(&self, id: &NodeId) -> Result<Option<Entity>> {
        Ok(self.storage.get_node(id)?)
    }

    async fn put_node(&self, mut entity: Entity) -> Result<()> {
        entity.updated_at = chrono::Utc::now().timestamp();
        if entity.created_at == 0 {
            entity.created_at = entity.updated_at;
        }
        self.storage.put_node(&entity)?;
        Ok(())
    }

    async fn delete_node(&self, id: &NodeId) -> Result<()> {
        self.storage.delete_node(id)?;
        Ok(())
    }

    async fn get_edge(&self, id: &uuid::Uuid) -> Result<Option<Edge>> {
        Ok(self.storage.get_edge(id)?)
    }

    async fn put_edge(&self, edge: Edge) -> Result<()> {
        self.storage.put_edge(&edge)?;
        Ok(())
    }

    async fn delete_edge(&self, id: &uuid::Uuid) -> Result<()> {
        self.storage.delete_edge(id)?;
        Ok(())
    }
}

