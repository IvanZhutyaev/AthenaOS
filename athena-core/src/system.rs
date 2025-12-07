use crate::config::AthenaConfig;
use anyhow::Result;
use athena_agents::runtime::AgentRuntime;
use athena_graph::engine::{DefaultGraphEngine, GraphEngine};
use athena_graph::storage::GraphStorage;
use athena_security::key_manager::KeyManager;
use athena_sync::p2p::P2PNode;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct AthenaSystem {
    pub config: AthenaConfig,
    pub key_manager: Arc<RwLock<KeyManager>>,
    pub graph_engine: Arc<dyn GraphEngine + Send + Sync>,
    pub agent_runtime: Arc<RwLock<AgentRuntime>>,
    pub p2p_node: Arc<RwLock<Option<P2PNode>>>,
}

impl AthenaSystem {
    pub async fn new(config: AthenaConfig) -> Result<Self> {
        std::fs::create_dir_all(&config.data_dir)?;

        // Initialize key manager
        let key_manager = KeyManager::new(&config.key_store_path)?;
        let key_manager = Arc::new(RwLock::new(key_manager));

        // Initialize graph engine
        let storage = GraphStorage::open(&config.graph_db_path)?;
        let graph_engine: Arc<dyn GraphEngine + Send + Sync> =
            Arc::new(DefaultGraphEngine::new(storage));

        // Initialize agent runtime
        let agent_runtime = AgentRuntime::new()?;
        let agent_runtime = Arc::new(RwLock::new(agent_runtime));

        // Initialize P2P node (optional)
        let p2p_node = if config.enable_p2p {
            Some(P2PNode::new()?)
        } else {
            None
        };
        let p2p_node = Arc::new(RwLock::new(p2p_node));

        Ok(Self {
            config,
            key_manager,
            graph_engine,
            agent_runtime,
            p2p_node,
        })
    }

    pub async fn initialize(&self) -> Result<()> {
        // Generate default key if none exists
        {
            let mut km = self.key_manager.write().await;
            if km.get_default_public_key().is_none() {
                km.generate_key("default".to_string())?;
            }
        }

        Ok(())
    }
}

