use crate::agent::AthenaAgent;
use crate::metadata::{AgentMetadata, SystemRequirements};
use athena_graph::entity::{Entity, NodeId, PropertyValue};
use athena_graph::engine::GraphEngine;
use athena_security::permissions::{Capability, PermissionSet};
use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

pub struct WebClipperAgent {
    graph_engine: Arc<dyn GraphEngine + Send + Sync>,
}

impl WebClipperAgent {
    pub fn new(graph_engine: Arc<dyn GraphEngine + Send + Sync>) -> Self {
        Self { graph_engine }
    }

    pub async fn create_agent_entity(&self) -> Result<AthenaAgent> {
        let metadata = AgentMetadata {
            id: Uuid::parse_str("00000000-0000-0000-0000-000000000003")?,
            name: "Web Clipper".to_string(),
            version: "1.0.0".to_string(),
            author: vec![],
            capabilities: vec![],
            requirements: SystemRequirements {
                min_memory_mb: 128,
                min_cpu_cores: 1,
                required_capabilities: vec!["network".to_string()],
            },
            manifest_hash: vec![],
        };

        let permissions = PermissionSet::new()
            .with_capability(Capability::ReadNode {
                pattern: "*".to_string(),
            })
            .with_capability(Capability::WriteNode {
                pattern: "web:*".to_string(),
            })
            .with_capability(Capability::NetworkAccess {
                endpoints: vec!["https://*".to_string(), "http://*".to_string()],
            });

        let wasm_module = vec![];

        Ok(AthenaAgent::new(metadata, wasm_module))
    }

    pub async fn clip_url(&self, url: &str) -> Result<()> {
        // Basic web clipper - in production would fetch and parse HTML
        let mut properties = HashMap::new();
        properties.insert(
            "type".to_string(),
            PropertyValue::String("web_page".to_string()),
        );
        properties.insert("url".to_string(), PropertyValue::String(url.to_string()));
        properties.insert(
            "clipped_at".to_string(),
            PropertyValue::DateTime(chrono::Utc::now().timestamp()),
        );

        // Try to extract title from URL
        let title = url
            .split('/')
            .last()
            .unwrap_or(url)
            .split('?')
            .next()
            .unwrap_or(url)
            .to_string();

        let entity = Entity {
            id: NodeId::new(),
            label: format!("Web: {}", title),
            properties,
            created_at: chrono::Utc::now().timestamp(),
            updated_at: chrono::Utc::now().timestamp(),
            version: 1,
        };

        self.graph_engine.put_node(entity).await?;
        Ok(())
    }

    pub async fn clip_html(&self, html: &str, url: &str) -> Result<()> {
        // Basic HTML parsing - extract title and content
        let mut properties = HashMap::new();
        properties.insert(
            "type".to_string(),
            PropertyValue::String("web_page".to_string()),
        );
        properties.insert("url".to_string(), PropertyValue::String(url.to_string()));
        properties.insert(
            "html_content".to_string(),
            PropertyValue::String(html.to_string()),
        );

        // Extract title from HTML
        let title = if let Some(start) = html.find("<title>") {
            if let Some(end) = html[start..].find("</title>") {
                html[start + 7..start + end].trim().to_string()
            } else {
                "Untitled".to_string()
            }
        } else {
            "Untitled".to_string()
        };

        let entity = Entity {
            id: NodeId::new(),
            label: format!("Web: {}", title),
            properties,
            created_at: chrono::Utc::now().timestamp(),
            updated_at: chrono::Utc::now().timestamp(),
            version: 1,
        };

        self.graph_engine.put_node(entity).await?;
        Ok(())
    }
}

