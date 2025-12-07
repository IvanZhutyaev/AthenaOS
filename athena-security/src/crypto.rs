use anyhow::Result;
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305, Key, Nonce,
};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use sha2::{Digest, Sha256};
use std::fmt;

#[derive(Clone, Debug)]
pub struct PublicKey(pub VerifyingKey);

#[derive(Clone)]
pub struct PrivateKey(pub SigningKey);

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct KeyId(pub [u8; 32]);

impl PublicKey {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        Ok(PublicKey(VerifyingKey::from_bytes(
            bytes.try_into().map_err(|_| anyhow::anyhow!("Invalid key length"))?,
        )?))
    }

    pub fn to_bytes(&self) -> [u8; 32] {
        self.0.to_bytes()
    }

    pub fn verify(&self, message: &[u8], signature: &Signature) -> Result<()> {
        self.0.verify(message, signature)?;
        Ok(())
    }

    pub fn key_id(&self) -> KeyId {
        KeyId(Sha256::digest(self.0.as_bytes()).into())
    }
}

impl PrivateKey {
    pub fn generate() -> Self {
        PrivateKey(SigningKey::generate(&mut OsRng))
    }

    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        Ok(PrivateKey(SigningKey::from_bytes(
            bytes.try_into().map_err(|_| anyhow::anyhow!("Invalid key length"))?,
        )))
    }

    pub fn to_bytes(&self) -> [u8; 32] {
        self.0.to_bytes()
    }

    pub fn public_key(&self) -> PublicKey {
        PublicKey(self.0.verifying_key())
    }

    pub fn sign(&self, message: &[u8]) -> Signature {
        self.0.sign(message)
    }
}

impl fmt::Debug for PrivateKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PrivateKey").finish_non_exhaustive()
    }
}

impl KeyId {
    pub fn from_bytes(bytes: &[u8]) -> Result<Self> {
        Ok(KeyId(
            bytes.try_into().map_err(|_| anyhow::anyhow!("Invalid key ID length"))?,
        ))
    }

    pub fn to_bytes(&self) -> [u8; 32] {
        self.0
    }
}

impl fmt::Display for KeyId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Display first 8 bytes as hex
        for byte in &self.0[..8] {
            write!(f, "{:02x}", byte)?;
        }
        Ok(())
    }
}

pub struct Cipher {
    cipher: ChaCha20Poly1305,
}

impl Cipher {
    pub fn new(key: &[u8; 32]) -> Self {
        let key = Key::from_slice(key);
        Self {
            cipher: ChaCha20Poly1305::new(key),
        }
    }

    pub fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>> {
        let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
        let ciphertext = self.cipher.encrypt(&nonce, plaintext)?;
        let mut result = nonce.to_vec();
        result.extend_from_slice(&ciphertext);
        Ok(result)
    }

    pub fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>> {
        if ciphertext.len() < 12 {
            return Err(anyhow::anyhow!("Ciphertext too short"));
        }
        let nonce = Nonce::from_slice(&ciphertext[..12]);
        let plaintext = self.cipher.decrypt(nonce, &ciphertext[12..])?;
        Ok(plaintext)
    }
}

pub fn hash(data: &[u8]) -> [u8; 32] {
    Sha256::digest(data).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_generation() {
        let private = PrivateKey::generate();
        let public = private.public_key();
        assert_eq!(public.key_id(), public.key_id());
    }

    #[test]
    fn test_sign_verify() {
        let private = PrivateKey::generate();
        let public = private.public_key();
        let message = b"test message";
        let signature = private.sign(message);
        public.verify(message, &signature).unwrap();
    }

    #[test]
    fn test_encrypt_decrypt() {
        let key = [0u8; 32];
        let cipher = Cipher::new(&key);
        let plaintext = b"secret message";
        let ciphertext = cipher.encrypt(plaintext).unwrap();
        let decrypted = cipher.decrypt(&ciphertext).unwrap();
        assert_eq!(plaintext, decrypted.as_slice());
    }
}

