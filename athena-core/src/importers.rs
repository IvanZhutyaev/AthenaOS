use athena_graph::entity::{Entity, NodeId, PropertyValue};
use athena_graph::engine::GraphEngine;
use anyhow::Result;
use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

pub struct DataImporter {
    graph_engine: Arc<dyn GraphEngine + Send + Sync>,
}

impl DataImporter {
    pub fn new(graph_engine: Arc<dyn GraphEngine + Send + Sync>) -> Self {
        Self { graph_engine }
    }

    pub async fn import_markdown_file(&self, file_path: &Path) -> Result<()> {
        let content = std::fs::read_to_string(file_path)?;
        let file_name = file_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        let mut properties = HashMap::new();
        properties.insert(
            "type".to_string(),
            PropertyValue::String("markdown".to_string()),
        );
        properties.insert(
            "file_path".to_string(),
            PropertyValue::String(file_path.to_string_lossy().to_string()),
        );
        properties.insert(
            "content".to_string(),
            PropertyValue::String(content.clone()),
        );

        // Extract title from first heading or filename
        let title = content
            .lines()
            .find(|l| l.starts_with("#"))
            .map(|l| l.trim_start_matches('#').trim().to_string())
            .unwrap_or_else(|| file_name.trim_end_matches(".md").to_string());

        let entity = Entity {
            id: NodeId::new(),
            label: format!("Note: {}", title),
            properties,
            created_at: chrono::Utc::now().timestamp(),
            updated_at: chrono::Utc::now().timestamp(),
            version: 1,
        };

        self.graph_engine.put_node(entity).await?;
        Ok(())
    }

    pub async fn import_text_file(&self, file_path: &Path) -> Result<()> {
        let content = std::fs::read_to_string(file_path)?;
        let file_name = file_path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        let mut properties = HashMap::new();
        properties.insert(
            "type".to_string(),
            PropertyValue::String("text".to_string()),
        );
        properties.insert(
            "file_path".to_string(),
            PropertyValue::String(file_path.to_string_lossy().to_string()),
        );
        properties.insert(
            "content".to_string(),
            PropertyValue::String(content),
        );

        let entity = Entity {
            id: NodeId::new(),
            label: format!("Text: {}", file_name),
            properties,
            created_at: chrono::Utc::now().timestamp(),
            updated_at: chrono::Utc::now().timestamp(),
            version: 1,
        };

        self.graph_engine.put_node(entity).await?;
        Ok(())
    }

    pub async fn import_directory(&self, dir_path: &Path) -> Result<usize> {
        let mut count = 0;

        if dir_path.is_dir() {
            for entry in std::fs::read_dir(dir_path)? {
                let entry = entry?;
                let path = entry.path();

                if path.is_file() {
                    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
                        match ext {
                            "md" | "markdown" => {
                                self.import_markdown_file(&path).await?;
                                count += 1;
                            }
                            "txt" => {
                                self.import_text_file(&path).await?;
                                count += 1;
                            }
                            _ => {}
                        }
                    }
                } else if path.is_dir() {
                    count += self.import_directory(&path).await?;
                }
            }
        }

        Ok(count)
    }
}

