use athena_graph::entity::GraphUpdate;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SyncMessage {
    GraphUpdate {
        from: Vec<u8>, // Public key
        update: GraphUpdate,
        version: u64,
    },
    Presence {
        node_id: Vec<u8>,
        online: bool,
    },
    FileChunk {
        file_id: Uuid,
        chunk_index: u64,
        data: Vec<u8>,
    },
    Message {
        to: Vec<u8>,
        content: Vec<u8>,
        encrypted: bool,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncProtocol {
    pub version: String,
    pub message: SyncMessage,
    pub timestamp: i64,
    pub signature: Vec<u8>,
}

