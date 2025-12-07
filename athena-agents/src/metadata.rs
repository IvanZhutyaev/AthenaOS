use athena_security::permissions::Capability;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentMetadata {
    pub id: Uuid,
    pub name: String,
    pub version: String,
    pub author: Vec<u8>, // Public key bytes
    pub capabilities: Vec<Capability>,
    pub requirements: SystemRequirements,
    pub manifest_hash: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemRequirements {
    pub min_memory_mb: u64,
    pub min_cpu_cores: u32,
    pub required_capabilities: Vec<String>,
}

