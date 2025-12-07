use crate::metadata::AgentMetadata;
use athena_security::permissions::PermissionSet;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AthenaAgent {
    pub metadata: AgentMetadata,
    pub permissions: PermissionSet,
    pub wasm_module: Vec<u8>,
    pub persistent_storage: Option<Uuid>,
    pub event_subscriptions: Vec<String>,
    pub api_endpoints: HashMap<String, String>,
}

impl AthenaAgent {
    pub fn new(metadata: AgentMetadata, wasm_module: Vec<u8>) -> Self {
        Self {
            metadata,
            permissions: PermissionSet::new(),
            wasm_module,
            persistent_storage: None,
            event_subscriptions: Vec::new(),
            api_endpoints: HashMap::new(),
        }
    }
}

