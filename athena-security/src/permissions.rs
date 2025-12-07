use serde::{Deserialize, Serialize};
use std::collections::HashSet;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Capability {
    ReadNode { pattern: String },
    WriteNode { pattern: String },
    NetworkAccess { endpoints: Vec<String> },
    HardwareAccess { devices: Vec<String> },
    AICall { max_tokens: u32 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PermissionSet {
    capabilities: HashSet<Capability>,
}

impl PermissionSet {
    pub fn new() -> Self {
        Self {
            capabilities: HashSet::new(),
        }
    }

    pub fn with_capability(mut self, capability: Capability) -> Self {
        self.capabilities.insert(capability);
        self
    }

    pub fn has_capability(&self, capability: &Capability) -> bool {
        self.capabilities.contains(capability)
    }

    pub fn add_capability(&mut self, capability: Capability) {
        self.capabilities.insert(capability);
    }

    pub fn remove_capability(&mut self, capability: &Capability) {
        self.capabilities.remove(capability);
    }

    pub fn is_empty(&self) -> bool {
        self.capabilities.is_empty()
    }
}

impl Default for PermissionSet {
    fn default() -> Self {
        Self::new()
    }
}

