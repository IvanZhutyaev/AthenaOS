use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct VersionId(pub u64);

impl VersionId {
    pub fn new() -> Self {
        VersionId(1)
    }

    pub fn next(&self) -> Self {
        VersionId(self.0 + 1)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Checkpoint {
    pub id: Uuid,
    pub version: VersionId,
    pub timestamp: i64,
    pub hash: Vec<u8>,
}

