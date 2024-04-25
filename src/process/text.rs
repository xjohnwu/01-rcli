use std::{fs, io::Read, path::Path};

use crate::{cli::TextSignFormat, get_reader, process_genpass};
use anyhow::Result;
use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use chacha20poly1305::{
    aead::{Aead, AeadCore, OsRng},
    ChaCha20Poly1305, Key, KeyInit, Nonce,
};
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};

pub trait TextSign {
    /// Sign the data from the reader and return the signature.
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>>;
}

pub trait TextVerify {
    /// Verify the data from the reader with the key.
    fn verify<R: Read>(&self, reader: R, sig: &[u8]) -> Result<bool>;
}

pub trait KeyLoader {
    fn load(path: impl AsRef<Path>) -> Result<Self>
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

pub struct ChaCha20Poly1305Cipher {
    cipher: ChaCha20Poly1305,
}

pub fn process_text_sign(input: &str, key: &str, format: TextSignFormat) -> Result<String> {
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
        TextSignFormat::ChaCha20Poly1305 => todo!(),
    };
    let signed = URL_SAFE_NO_PAD.encode(signed);
    Ok(signed)
}

pub fn process_text_verify(
    input: &str,
    key: &str,
    format: TextSignFormat,
    sig: &str,
) -> Result<bool> {
    let reader = get_reader(input)?;
    let sig = URL_SAFE_NO_PAD.decode(sig)?;
    let verified = match format {
        TextSignFormat::Blake3 => {
            let verifier = Blake3::load(key)?;
            verifier.verify(reader, &sig)?
        }
        TextSignFormat::Ed25519 => {
            let verifier = Ed25519Verifier::load(key)?;
            verifier.verify(reader, &sig)?
        }
        TextSignFormat::ChaCha20Poly1305 => todo!(),
    };
    Ok(verified)
}

pub fn process_text_generate(format: TextSignFormat) -> Result<Vec<Vec<u8>>> {
    match format {
        TextSignFormat::Blake3 => Blake3::generate(),
        TextSignFormat::Ed25519 => Ed25519Signer::generate(),
        TextSignFormat::ChaCha20Poly1305 => ChaCha20Poly1305Cipher::generate(),
    }
}

pub fn process_text_encrypt(input: &str, key: &str) -> Result<String> {
    let cipher = ChaCha20Poly1305Cipher::load(key)?;

    let mut reader = get_reader(input)?;
    let mut buf = Vec::new();
    // TODO: improve perf by reading in chunks.
    reader.read_to_end(&mut buf)?;

    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng); // 96-bits; unique per message
    let result = cipher.cipher.encrypt(&nonce, buf.as_ref());
    match result {
        Ok(encrypted) => {
            let mut v = Vec::new();
            v.extend(nonce);
            v.extend(encrypted);
            let ciphertext = URL_SAFE_NO_PAD.encode(v);
            Ok(ciphertext)
        }
        Err(e) => anyhow::bail!("Error encrypting: {}", e),
    }
}

pub fn process_text_decrypt(input: &str, key: &str) -> Result<String> {
    let cipher = ChaCha20Poly1305Cipher::load(key)?;
    let mut reader = get_reader(input)?;
    let mut s = String::new();
    // TODO: improve perf by reading in chunks.
    reader.read_to_string(&mut s)?;

    let buf = URL_SAFE_NO_PAD.decode(s.trim())?;

    let nonce = &buf[..12];
    let ciphertext = &buf[12..];
    let result = cipher
        .cipher
        .decrypt(Nonce::from_slice(nonce), ciphertext.as_ref());
    let decrypted = match result {
        Ok(decrypted) => String::from_utf8(decrypted)?,
        Err(e) => anyhow::bail!(e.to_string()),
    };
    Ok(decrypted)
}

impl TextSign for Blake3 {
    fn sign(&self, reader: &mut dyn Read) -> Result<Vec<u8>> {
        let mut buf = Vec::new();
        // TODO: improve perf by reading in chunks.
        reader.read_to_end(&mut buf)?;
        let key = blake3::keyed_hash(&self.key, &buf);
        Ok(key.as_bytes().to_vec())
    }
}

impl TextVerify for Blake3 {
    fn verify<R: Read>(&self, mut reader: R, sig: &[u8]) -> Result<bool> {
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
    fn verify<R: Read>(&self, mut reader: R, sig: &[u8]) -> Result<bool> {
        let mut buf = Vec::new();
        reader.read_to_end(&mut buf)?;
        let sig = Signature::from_bytes(sig.try_into()?);
        Ok(self.key.verify(&buf, &sig).is_ok())
    }
}

impl KeyLoader for Blake3 {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

impl Blake3 {
    pub fn new(key: [u8; 32]) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = &key[..32];
        let key = key.try_into()?;
        let signer = Blake3::new(key);
        Ok(signer)
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

impl KeyLoader for Ed25519Signer {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

impl Ed25519Verifier {
    pub fn new(key: VerifyingKey) -> Self {
        Self { key }
    }

    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = VerifyingKey::from_bytes(key.try_into()?)?;
        let verifier = Ed25519Verifier::new(key);
        Ok(verifier)
    }
}
impl KeyLoader for Ed25519Verifier {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
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
        let sk = SigningKey::generate(&mut csprng);
        let pk = sk.verifying_key().to_bytes().to_vec();
        let sk = sk.to_bytes().to_vec();
        Ok(vec![sk, pk])
    }
}

impl KeyGenerator for ChaCha20Poly1305Cipher {
    fn generate() -> Result<Vec<Vec<u8>>> {
        let key = ChaCha20Poly1305::generate_key(&mut OsRng);
        Ok(vec![key.to_vec()])
    }
}

impl KeyLoader for ChaCha20Poly1305Cipher {
    fn load(path: impl AsRef<Path>) -> Result<Self> {
        let key = fs::read(path)?;
        Self::try_new(&key)
    }
}

impl ChaCha20Poly1305Cipher {
    pub fn new(key: Key) -> Self {
        let cipher = ChaCha20Poly1305::new(&key);
        Self { cipher }
    }

    pub fn try_new(key: &[u8]) -> Result<Self> {
        let key = Key::clone_from_slice(key);
        Ok(ChaCha20Poly1305Cipher::new(key))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn take_blake3_sign_verify() -> Result<()> {
        let blake3 = Blake3::load("fixtures/blake3.txt")?;
        let data = b"hello";
        let sig = blake3.sign(&mut &data[..]).unwrap();
        assert!(blake3.verify(&mut &data[..], &sig).unwrap());
        Ok(())
    }

    #[test]
    fn test_ed25519_sign_verify() -> Result<()> {
        let sk = Ed25519Signer::load("fixtures/ed25519.sk")?;
        let pk = Ed25519Verifier::load("fixtures/ed25519.pk")?;

        let data = b"hello";
        let sig = sk.sign(&mut &data[..]).unwrap();
        assert!(pk.verify(&data[..], &sig).unwrap());
        Ok(())
    }

    #[test]
    fn test_chacha20poly1305() -> Result<(), chacha20poly1305::Error> {
        let key = ChaCha20Poly1305::generate_key(&mut OsRng);
        let cipher = ChaCha20Poly1305::new(&key);
        let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng); // 96-bits; unique per message
        let ciphertext = cipher.encrypt(&nonce, b"plaintext message".as_ref())?;
        let plaintext = cipher.decrypt(&nonce, ciphertext.as_ref())?;
        assert_eq!(&plaintext, b"plaintext message");
        Ok(())
    }
}
