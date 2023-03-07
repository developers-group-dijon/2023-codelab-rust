use anyhow::{bail, Result};
use chacha20poly1305::aead::Aead;
use chacha20poly1305::XChaCha20Poly1305;
use chacha20poly1305::{aead::OsRng, KeyInit};
use rand::RngCore;
use serde::{Deserialize, Serialize};

/// Representation of a crypted message.
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct EncryptedMessage {
    /// Cryptographic nonce.
    nonce: [u8; 24],
    /// Password salt.
    salt: [u8; 32],
    /// Crypted message.
    message: Vec<u8>,
}

/// Encrpyts a given message with a password security on top.
///
/// - The password is hashed using Argon2id encryption.
/// - The message is hashed using XChaCha20Poly1305 symetric encription with the hashed password.
///
/// This will return an error upon encryption error.
pub fn encrypt(content: &str, password: &str) -> Result<EncryptedMessage> {
    let mut message = EncryptedMessage::default();

    let argon2_config = argon2_config();

    let mut salt = [0u8; 32];
    let mut nonce = [0u8; 24];
    OsRng.fill_bytes(&mut salt);
    OsRng.fill_bytes(&mut nonce);

    message.salt = salt;
    message.nonce = nonce;

    let key = argon2::hash_raw(password.as_bytes(), &salt, &argon2_config)?;

    let cipher = XChaCha20Poly1305::new(key[..32].as_ref().into());

    let ciphertext_result = cipher.encrypt(nonce.as_ref().into(), content.as_bytes().as_ref());

    if let Err(error) = ciphertext_result {
        bail!("Error while ciphering data : {error}");
    }

    let ciphertext = ciphertext_result.unwrap();
    message.message = ciphertext;

    Ok(message)
}

/// Decrypts a given message with a password.
///
/// - The password is compared using Argon2id hashing.
/// - The message is deciphered using XChaCha20Poly1305 symetric encription with the hashed password.
///
/// This will return an error upon decryption error.
pub fn decrypt(crypted: EncryptedMessage, password: &str) -> Result<String> {
    let argon2_config = argon2_config();

    let salt = crypted.salt;
    let nonce = crypted.nonce;
    let ciphertext = crypted.message;

    let key = argon2::hash_raw(password.as_bytes(), &salt, &argon2_config)?;
    let cipher = XChaCha20Poly1305::new(key[..32].as_ref().into());

    let decoded_result = cipher.decrypt(nonce.as_ref().into(), ciphertext.as_ref());

    if let Err(error) = decoded_result {
        bail!("Error while deciphering data : {error}");
    }

    let decoded = decoded_result.unwrap();

    Ok(String::from_utf8(decoded).unwrap())
}

/// Generates a strong Argon2id configuration for ciphers.
fn argon2_config<'a>() -> argon2::Config<'a> {
    argon2::Config {
        variant: argon2::Variant::Argon2id,
        hash_length: 32,
        lanes: 8,
        mem_cost: 16 * 1024,
        time_cost: 8,
        ..Default::default()
    }
}
