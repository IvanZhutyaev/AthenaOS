use crate::agent::AthenaAgent;
use crate::metadata::{AgentMetadata, SystemRequirements};
use athena_graph::entity::{Entity, NodeId, PropertyValue};
use athena_graph::engine::GraphEngine;
use athena_security::permissions::{Capability, PermissionSet};
use anyhow::Result;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

pub struct FileWatcherAgent {
    watch_paths: Vec<PathBuf>,
    graph_engine: Arc<dyn GraphEngine + Send + Sync>,
}

impl FileWatcherAgent {
    pub fn new(
        watch_paths: Vec<PathBuf>,
        graph_engine: Arc<dyn GraphEngine + Send + Sync>,
    ) -> Self {
        Self {
            watch_paths,
            graph_engine,
        }
    }

    pub async fn create_agent_entity(&self) -> Result<AthenaAgent> {
        let metadata = AgentMetadata {
            id: Uuid::parse_str("00000000-0000-0000-0000-000000000001")?,
            name: "File Watcher".to_string(),
            version: "1.0.0".to_string(),
            author: vec![], // System agent
            capabilities: vec![],
            requirements: SystemRequirements {
                min_memory_mb: 64,
                min_cpu_cores: 1,
                required_capabilities: vec![],
            },
            manifest_hash: vec![],
        };

        let permissions = PermissionSet::new()
            .with_capability(Capability::ReadNode {
                pattern: "*".to_string(),
            })
            .with_capability(Capability::WriteNode {
                pattern: "file:*".to_string(),
            });

        // Placeholder WASM module (in production would be actual WASM)
        let wasm_module = vec![];

        Ok(AthenaAgent::new(metadata, wasm_module))
    }

    pub async fn watch_and_index(&self) -> Result<()> {
        for path in &self.watch_paths {
            if path.exists() {
                self.index_path(path).await?;
            }
        }
        Ok(())
    }

    async fn index_path(&self, path: &PathBuf) -> Result<()> {
        if path.is_file() {
            self.index_file(path).await?;
        } else if path.is_dir() {
            self.index_directory(path).await?;
        }
        Ok(())
    }

    async fn index_file(&self, file_path: &PathBuf) -> Result<()> {
        let file_name = file_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        let mut properties = HashMap::new();
        properties.insert(
            "type".to_string(),
            PropertyValue::String("file".to_string()),
        );
        properties.insert(
            "path".to_string(),
            PropertyValue::String(file_path.to_string_lossy().to_string()),
        );
        properties.insert(
            "extension".to_string(),
            PropertyValue::String(
                file_path
                    .extension()
                    .and_then(|e| e.to_str())
                    .unwrap_or("")
                    .to_string(),
            ),
        );

        if let Ok(metadata) = std::fs::metadata(file_path) {
            if let Ok(modified) = metadata.modified() {
                properties.insert(
                    "modified_at".to_string(),
                    PropertyValue::DateTime(
                        modified
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap()
                            .as_secs() as i64,
                    ),
                );
            }
            properties.insert(
                "size".to_string(),
                PropertyValue::Number(metadata.len() as f64),
            );
        }

        let entity = Entity {
            id: NodeId::new(),
            label: format!("File: {}", file_name),
            properties,
            created_at: chrono::Utc::now().timestamp(),
            updated_at: chrono::Utc::now().timestamp(),
            version: 1,
        };

        self.graph_engine.put_node(entity).await?;
        Ok(())
    }

    async fn index_directory(&self, dir_path: &PathBuf) -> Result<()> {
        let dir_name = dir_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        let mut properties = HashMap::new();
        properties.insert(
            "type".to_string(),
            PropertyValue::String("directory".to_string()),
        );
        properties.insert(
            "path".to_string(),
            PropertyValue::String(dir_path.to_string_lossy().to_string()),
        );

        let entity = Entity {
            id: NodeId::new(),
            label: format!("Directory: {}", dir_name),
            properties,
            created_at: chrono::Utc::now().timestamp(),
            updated_at: chrono::Utc::now().timestamp(),
            version: 1,
        };

        self.graph_engine.put_node(entity).await?;

        // Recursively index contents
        if let Ok(entries) = std::fs::read_dir(dir_path) {
            for entry in entries.flatten() {
                self.index_path(&entry.path()).await?;
            }
        }

        Ok(())
    }
}

