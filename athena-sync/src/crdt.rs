use athena_graph::entity::{GraphUpdate, NodeId};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CRDTState {
    pub nodes: HashMap<NodeId, CRDTNode>,
    pub edges: HashMap<Uuid, CRDTEdge>,
    pub vector_clock: HashMap<Vec<u8>, u64>, // Peer ID -> counter
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CRDTNode {
    pub node_id: NodeId,
    pub data: Vec<u8>,
    pub timestamp: i64,
    pub peer_id: Vec<u8>,
    pub counter: u64,
    pub deleted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CRDTEdge {
    pub edge_id: Uuid,
    pub from: NodeId,
    pub to: NodeId,
    pub data: Vec<u8>,
    pub timestamp: i64,
    pub peer_id: Vec<u8>,
    pub counter: u64,
    pub deleted: bool,
}

impl CRDTState {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            edges: HashMap::new(),
            vector_clock: HashMap::new(),
        }
    }

    pub fn merge(&mut self, other: &CRDTState) {
        // Simple last-write-wins merge
        for (node_id, node) in &other.nodes {
            let existing = self.nodes.get(node_id);
            if existing.is_none()
                || existing.map(|n| n.timestamp < node.timestamp).unwrap_or(false)
            {
                self.nodes.insert(node_id.clone(), node.clone());
            }
        }

        for (edge_id, edge) in &other.edges {
            let existing = self.edges.get(edge_id);
            if existing.is_none()
                || existing.map(|e| e.timestamp < edge.timestamp).unwrap_or(false)
            {
                self.edges.insert(*edge_id, edge.clone());
            }
        }

        // Merge vector clock
        for (peer_id, counter) in &other.vector_clock {
            let current = self.vector_clock.get(peer_id).copied().unwrap_or(0);
            self.vector_clock.insert(peer_id.clone(), current.max(*counter));
        }
    }

    pub fn apply_update(&mut self, update: &GraphUpdate, peer_id: Vec<u8>) {
        let mut counter = self.vector_clock.get(&peer_id).copied().unwrap_or(0);
        let timestamp = chrono::Utc::now().timestamp();

        for node in &update.nodes {
            counter += 1;
            let crdt_node = CRDTNode {
                node_id: node.id.clone(),
                data: bincode::serialize(node).unwrap_or_default(),
                timestamp,
                peer_id: peer_id.clone(),
                counter,
                deleted: false,
            };
            self.nodes.insert(node.id.clone(), crdt_node);
        }

        for edge in &update.edges {
            counter += 1;
            let crdt_edge = CRDTEdge {
                edge_id: edge.id,
                from: edge.from.clone(),
                to: edge.to.clone(),
                data: bincode::serialize(edge).unwrap_or_default(),
                timestamp,
                peer_id: peer_id.clone(),
                counter,
                deleted: false,
            };
            self.edges.insert(edge.id, crdt_edge);
        }

        for node_id in &update.deleted_nodes {
            if let Some(node) = self.nodes.get_mut(node_id) {
                node.deleted = true;
                node.timestamp = timestamp;
            }
        }

        for edge_id in &update.deleted_edges {
            if let Some(edge) = self.edges.get_mut(edge_id) {
                edge.deleted = true;
                edge.timestamp = timestamp;
            }
        }

        self.vector_clock.insert(peer_id, counter);
    }
}

impl Default for CRDTState {
    fn default() -> Self {
        Self::new()
    }
}

