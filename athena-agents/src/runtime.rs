use crate::agent::AthenaAgent;
use anyhow::Result;
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use wasmtime::*;

pub struct AgentRuntime {
    engine: Engine,
    agents: Arc<RwLock<HashMap<uuid::Uuid, AgentInstance>>>,
}

struct AgentInstance {
    agent: AthenaAgent,
    store: Store<()>,
    instance: Instance,
}

impl AgentRuntime {
    pub fn new() -> Result<Self> {
        let mut config = Config::default();
        config.async_support(true);

        let engine = Engine::new(&config)?;

        Ok(Self {
            engine,
            agents: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    pub async fn load_agent(&self, agent: AthenaAgent) -> Result<uuid::Uuid> {
        let id = agent.metadata.id;
        let module = Module::new(&self.engine, &agent.wasm_module)?;
        let mut store = Store::new(&self.engine, ());
        let instance = Instance::new(&mut store, &module, &[])?;

        let instance = AgentInstance {
            agent,
            store,
            instance,
        };

        self.agents.write().await.insert(id, instance);
        Ok(id)
    }

    pub async fn unload_agent(&self, id: &uuid::Uuid) -> Result<()> {
        self.agents.write().await.remove(id);
        Ok(())
    }

    pub async fn call_agent(&self, id: &uuid::Uuid, function: &str, input: &[u8]) -> Result<Vec<u8>> {
        let mut agents = self.agents.write().await;
        let instance = agents
            .get_mut(id)
            .ok_or_else(|| anyhow::anyhow!("Agent not found"))?;

        // Simple implementation - in production would call WASM function
        Ok(format!("Agent {} called function {} with input: {:?}", id, function, input).into_bytes())
    }

    pub async fn list_agents(&self) -> Vec<uuid::Uuid> {
        self.agents.read().await.keys().cloned().collect()
    }
}

#[async_trait]
pub trait AgentRuntimeTrait: Send + Sync {
    async fn load_agent(&self, agent: AthenaAgent) -> Result<uuid::Uuid>;
    async fn unload_agent(&self, id: &uuid::Uuid) -> Result<()>;
    async fn call_agent(&self, id: &uuid::Uuid, function: &str, input: &[u8]) -> Result<Vec<u8>>;
    async fn list_agents(&self) -> Vec<uuid::Uuid>;
}

#[async_trait]
impl AgentRuntimeTrait for AgentRuntime {
    async fn load_agent(&self, agent: AthenaAgent) -> Result<uuid::Uuid> {
        AgentRuntime::load_agent(self, agent).await
    }

    async fn unload_agent(&self, id: &uuid::Uuid) -> Result<()> {
        AgentRuntime::unload_agent(self, id).await
    }

    async fn call_agent(&self, id: &uuid::Uuid, function: &str, input: &[u8]) -> Result<Vec<u8>> {
        AgentRuntime::call_agent(self, id, function, input).await
    }

    async fn list_agents(&self) -> Vec<uuid::Uuid> {
        AgentRuntime::list_agents(self).await
    }
}
