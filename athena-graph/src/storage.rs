use crate::entity::{Edge, Entity, NodeId};
use anyhow::Result;
use rocksdb::{IteratorMode, Options, DB};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::Arc;

const NODE_PREFIX: &[u8] = b"node:";
const EDGE_PREFIX: &[u8] = b"edge:";
const INDEX_PREFIX: &[u8] = b"idx:";

pub struct GraphStorage {
    db: Arc<DB>,
}

impl GraphStorage {
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let mut opts = Options::default();
        opts.create_if_missing(true);
        opts.set_max_open_files(10000);
        opts.set_write_buffer_size(64 * 1024 * 1024);

        let db = DB::open(&opts, path)?;
        Ok(Self { db: Arc::new(db) })
    }

    pub fn put_node(&self, entity: &Entity) -> Result<()> {
        let key = self.node_key(&entity.id);
        let value = bincode::serialize(entity)?;
        self.db.put(key, value)?;
        Ok(())
    }

    pub fn get_node(&self, id: &NodeId) -> Result<Option<Entity>> {
        let key = self.node_key(id);
        match self.db.get(key)? {
            Some(data) => Ok(Some(bincode::deserialize(&data)?)),
            None => Ok(None),
        }
    }

    pub fn delete_node(&self, id: &NodeId) -> Result<()> {
        let key = self.node_key(id);
        self.db.delete(key)?;
        Ok(())
    }

    pub fn put_edge(&self, edge: &Edge) -> Result<()> {
        let key = self.edge_key(&edge.id);
        let value = bincode::serialize(edge)?;
        self.db.put(key, value)?;
        Ok(())
    }

    pub fn get_edge(&self, id: &uuid::Uuid) -> Result<Option<Edge>> {
        let key = self.edge_key(id);
        match self.db.get(key)? {
            Some(data) => Ok(Some(bincode::deserialize(&data)?)),
            None => Ok(None),
        }
    }

    pub fn delete_edge(&self, id: &uuid::Uuid) -> Result<()> {
        let key = self.edge_key(id);
        self.db.delete(key)?;
        Ok(())
    }

    pub fn iter_nodes(&self) -> impl Iterator<Item = Result<Entity>> + '_ {
        self.db
            .iterator(IteratorMode::From(NODE_PREFIX, rocksdb::Direction::Forward))
            .take_while(|item| {
                item.as_ref()
                    .map(|(k, _)| k.starts_with(NODE_PREFIX))
                    .unwrap_or(false)
            })
            .map(|item| {
                let (_, value) = item?;
                Ok(bincode::deserialize(&value)?)
            })
    }

    pub fn iter_edges(&self) -> impl Iterator<Item = Result<Edge>> + '_ {
        self.db
            .iterator(IteratorMode::From(EDGE_PREFIX, rocksdb::Direction::Forward))
            .take_while(|item| {
                item.as_ref()
                    .map(|(k, _)| k.starts_with(EDGE_PREFIX))
                    .unwrap_or(false)
            })
            .map(|item| {
                let (_, value) = item?;
                Ok(bincode::deserialize(&value)?)
            })
    }

    fn node_key(&self, id: &NodeId) -> Vec<u8> {
        let mut key = NODE_PREFIX.to_vec();
        key.extend_from_slice(&id.0.as_bytes());
        key
    }

    fn edge_key(&self, id: &uuid::Uuid) -> Vec<u8> {
        let mut key = EDGE_PREFIX.to_vec();
        key.extend_from_slice(&id.as_bytes());
        key
    }
}

