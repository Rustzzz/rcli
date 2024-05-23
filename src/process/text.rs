use crate::{get_reader, process_genpass, TextSignFormat};
use anyhow::Result;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use chacha20poly1305::{aead::Aead, AeadCore, ChaCha20Poly1305, KeyInit, Nonce};
use core::str;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use rand::rngs::OsRng;
use std::{fs, io::Read, path::Path};

pub trait TextSign {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;
}

pub trait TextVerify {
    fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> Result<bool>;
}

pub trait TextEncrypt {
    fn encrypt(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;
}

pub trait TextDecrypt {
    fn decrypt(&self, data: Vec<u8>) -> Result<Vec<u8>>;
}

pub trait KeyLoader {
    fn load(key: impl AsRef<Path>) -> Result<Self>
    where
        Self: Sized;
}

pub trait KeyGenerator {
    fn generate() -> Result<Vec<Vec<u8>>>;
}

pub struct Blake3 {
    key: [u8; 32],
}

pub struct Ed25519Signer {
    key: SigningKey,
}

pub struct Ed25519Verifier {
    key: VerifyingKey,
}

pub struct Chacha20poly1305EncryptAndDecrypt {
    cipher: ChaCha20Poly1305,
    nonce: Vec<u8>,
}

pub fn process_sign(input: &str, key: &str, format: TextSignFormat) -> Result<String> {
    let mut reader = get_reader(input)?;
    let signed = match format {
        TextSignFormat::Blake3 => {
            let signer = Blake3::load(key)?;
            signer.sign(&mut reader)?
        }
        TextSignFormat::Ed25519 => {
            let signer = Ed25519Signer::load(key)?;
            signer.sign(&mut reader)?
        }
        #[allow(dead_code)]
        _ => todo!(),
    };
    let signed = URL_SAFE_NO_PAD.encode(&signed);
    Ok(signed)
}

pub fn process_encrypt(input: &str, key: &str) -> Result<String> {
    let mut reader = get_reader(input)?;
    let encrypt = Chacha20poly1305EncryptAndDecrypt::load(key)?;
    let ret = encrypt.encrypt(&mut reader)?;
    let encrypted = URL_SAFE_NO_PAD.encode(&ret);
    Ok(encrypted)
}

pub fn process_decrypt(sig: &str, key: &str) -> Result<String> {
    let sig = URL_SAFE_NO_PAD.decode(sig)?;
    let encrypt = Chacha20poly1305EncryptAndDecrypt::load(key)?;
    let ret = encrypt.decrypt(sig)?;
    Ok(String::from_utf8_lossy(&ret).to_string())
}

pub fn process_verify(input: &str, key: &str, format: TextSignFormat, sig: &str) -> Result<bool> {
    let mut reader = get_reader(input)?;
    let sig = URL_SAFE_NO_PAD.decode(sig)?;
    let verified = match format {
        TextSignFormat::Blake3 => {
            let verifier = Blake3::load(key)?;
            verifier.verify(&mut reader, &sig)?
        }
        TextSignFormat::Ed25519 => {
            let verifier = Ed25519Verifier::load(key)?;
            verifier.verify(&mut reader, &sig)?
        }
        #[allow(dead_code)]
        TextSignFormat::Chacha20poly1305 => todo!(),
    };
    Ok(verified)
}

pub fn process_key_generate(format: TextSignFormat) -> Result<Vec<Vec<u8>>> {
    match format {
        TextSignFormat::Blake3 => Blake3::generate(),
        TextSignFormat::Ed25519 => Ed25519Signer::generate(),
        TextSignFormat::Chacha20poly1305 => Chacha20poly1305EncryptAndDecrypt::generate(),
    }
}

impl TextSign for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        Ok(blake3::keyed_hash(&self.key, &buf).as_bytes().to_vec())
    }
}

impl TextVerify for Blake3 {
    fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let hash = blake3::keyed_hash(&self.key, &buf);
        let hash = hash.as_bytes();
        Ok(hash == sig)
    }
}

impl TextSign for Ed25519Signer {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let sig = self.key.sign(&buf);
        Ok(sig.to_bytes().to_vec())
    }
}

impl TextVerify for Ed25519Verifier {
    fn verify(&self, reader: &mut dyn Read, sig: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let sig = Signature::from_bytes(sig.try_into()?);
        let ret = self.key.verify(&buf, &sig).is_ok();
        Ok(ret)
    }
}
impl Blake3 {
    // pub fn new(key: [u8; 32]) -> Self {
    //     Self { key }
    // }
    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = &key[0..32];
        let key = key.try_into()?;
        Ok(Self { key })
    }
}

impl TextEncrypt for Chacha20poly1305EncryptAndDecrypt {
    fn encrypt(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let nonce = Nonce::from_slice(self.nonce.as_slice());
        let encrypt = self
            .cipher
            .encrypt(nonce, buf.as_ref())
            .map_err(|e| anyhow::anyhow!(e))?;
        Ok(encrypt.to_vec())
    }
}

impl TextDecrypt for Chacha20poly1305EncryptAndDecrypt {
    fn decrypt(&self, data: Vec<u8>) -> Result<Vec<u8>> {
        let nonce = Nonce::from_slice(self.nonce.as_slice());
        let decrypt = self
            .cipher
            .decrypt(&nonce, data.as_slice())
            .map_err(|e| anyhow::anyhow!(e))?;
        Ok(decrypt.to_vec())
    }
}

impl Ed25519Signer {
    pub fn new(key: SigningKey) -> Self {
        Self { key }
    }
    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = SigningKey::from_bytes(key.try_into()?);
        let signer = Ed25519Signer::new(key);
        Ok(signer)
    }
}

impl Ed25519Verifier {
    pub fn new(key: VerifyingKey) -> Self {
        Self { key }
    }
    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = VerifyingKey::from_bytes(key.try_into()?)?;
        let signer = Ed25519Verifier::new(key);
        Ok(signer)
    }
}

impl Chacha20poly1305EncryptAndDecrypt {
    pub fn new(cipher: ChaCha20Poly1305, nonce: Vec<u8>) -> Self {
        Self { cipher, nonce }
    }
    pub fn try_new(key: &[u8], nonce: &[u8]) -> Result<Self> {
        let cipher = ChaCha20Poly1305::new_from_slice(key)?;
        let encrypt = Chacha20poly1305EncryptAndDecrypt::new(cipher, nonce.into());
        Ok(encrypt)
    }
}
impl KeyLoader for Blake3 {
    fn load(key: impl AsRef<Path>) -> Result<Self>
    where
        Self: Sized,
    {
        let path = key.as_ref();
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

impl KeyLoader for Ed25519Signer {
    fn load(key: impl AsRef<Path>) -> Result<Self>
    where
        Self: Sized,
    {
        let path = key.as_ref();
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

impl KeyLoader for Ed25519Verifier {
    fn load(key: impl AsRef<Path>) -> Result<Self>
    where
        Self: Sized,
    {
        let path = key.as_ref();
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

impl KeyLoader for Chacha20poly1305EncryptAndDecrypt {
    fn load(path: impl AsRef<Path>) -> Result<Self>
    where
        Self: Sized,
    {
        let path = path.as_ref();
        let nonce_key = fs::read(path)?;
        // 由于nonce固定为96 bits，所以这里分割前12字节为nonce，后面即为key
        let (nonce_bytes, key) = nonce_key.split_at(12);
        Self::try_new(&key, nonce_bytes)
    }
}

impl KeyGenerator for Blake3 {
    fn generate() -> Result<Vec<Vec<u8>>> {
        let key = process_genpass(32, true, true, true, true)?;
        let key = key.as_bytes().to_vec();
        Ok(vec![key])
    }
}

impl KeyGenerator for Ed25519Signer {
    fn generate() -> Result<Vec<Vec<u8>>> {
        let mut csprng = OsRng;
        let key = SigningKey::generate(&mut csprng);
        let pub_key = key.verifying_key().as_bytes().to_vec();
        let secret_key = key.as_bytes().to_vec();
        Ok(vec![pub_key, secret_key])
    }
}

impl KeyGenerator for Chacha20poly1305EncryptAndDecrypt {
    fn generate() -> Result<Vec<Vec<u8>>> {
        // 生成nonce和key 写入文件
        let mut nonce_key = Vec::new();
        let key = ChaCha20Poly1305::generate_key(&mut OsRng);
        let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng); // 96-bits; unique per message
        let nonce_bytes = nonce.as_slice();
        nonce_key.extend_from_slice(nonce_bytes);
        nonce_key.extend_from_slice(&key);
        Ok(vec![nonce_key])
    }
}
