use crate::{PrivateKey, PublicKey};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyStore {
    keys: HashMap<String, KeyEntry>,
    default_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct KeyEntry {
    public_key: Vec<u8>,
    encrypted_private_key: Vec<u8>,
    created_at: i64,
}

pub struct KeyManager {
    store_path: PathBuf,
    store: KeyStore,
    master_key: [u8; 32],
}

impl KeyManager {
    pub fn new<P: AsRef<Path>>(store_path: P) -> Result<Self> {
        let store_path = store_path.as_ref().to_path_buf();
        let store = if store_path.exists() {
            let data = std::fs::read(&store_path)?;
            bincode::deserialize(&data)?
        } else {
            KeyStore {
                keys: HashMap::new(),
                default_key: None,
            }
        };

        // In production, master key should be derived from user password
        let master_key = [0u8; 32]; // Placeholder

        Ok(Self {
            store_path,
            store,
            master_key,
        })
    }

    pub fn generate_key(&mut self, name: String) -> Result<PublicKey> {
        let private = PrivateKey::generate();
        let public = private.public_key();

        // Encrypt private key with master key
        let cipher = crate::Cipher::new(&self.master_key);
        let encrypted = cipher.encrypt(&private.to_bytes())?;

        let entry = KeyEntry {
            public_key: public.to_bytes().to_vec(),
            encrypted_private_key: encrypted,
            created_at: chrono::Utc::now().timestamp(),
        };

        if self.store.default_key.is_none() {
            self.store.default_key = Some(name.clone());
        }

        self.store.keys.insert(name, entry);
        self.save()?;

        Ok(public)
    }

    pub fn get_public_key(&self, name: &str) -> Option<PublicKey> {
        self.store.keys.get(name).and_then(|entry| {
            PublicKey::from_bytes(&entry.public_key).ok()
        })
    }

    pub fn get_default_public_key(&self) -> Option<PublicKey> {
        self.store
            .default_key
            .as_ref()
            .and_then(|name| self.get_public_key(name))
    }

    pub fn get_private_key(&self, name: &str) -> Result<Option<PrivateKey>> {
        let entry = match self.store.keys.get(name) {
            Some(e) => e,
            None => return Ok(None),
        };

        let cipher = crate::Cipher::new(&self.master_key);
        let decrypted = cipher.decrypt(&entry.encrypted_private_key)?;
        let private = PrivateKey::from_bytes(&decrypted)?;
        Ok(Some(private))
    }

    fn save(&self) -> Result<()> {
        if let Some(parent) = self.store_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let data = bincode::serialize(&self.store)?;
        std::fs::write(&self.store_path, data)?;
        Ok(())
    }
}

