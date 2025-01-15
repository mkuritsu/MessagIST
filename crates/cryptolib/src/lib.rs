use argon2::{password_hash::SaltString, Argon2, PasswordHash, PasswordVerifier};
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305,
};
use rsa::Pkcs1v15Encrypt;

pub use rsa::{
    pkcs8::{DecodePrivateKey, DecodePublicKey, EncodePrivateKey, EncodePublicKey, LineEnding},
    RsaPrivateKey, RsaPublicKey,
};

pub mod utils;

pub fn protect(
    plaintext: &[u8],
    key: &[u8],
) -> Result<(Vec<u8>, Vec<u8>), chacha20poly1305::Error> {
    let cipher = ChaCha20Poly1305::new(key.into());
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
    let ciphertext = cipher.encrypt(&nonce, plaintext)?;
    Ok((ciphertext, nonce.to_vec()))
}

pub fn check(ciphertext: &[u8], key: &[u8], nonce: &[u8]) -> bool {
    let cipher = ChaCha20Poly1305::new(key.into());
    match cipher.decrypt(nonce.into(), ciphertext) {
        Ok(_) => true,
        Err(_) => false,
    }
}

pub fn unprotect(
    ciphertext: &[u8],
    key: &[u8],
    nonce: &[u8],
) -> Result<Vec<u8>, chacha20poly1305::Error> {
    let cipher = ChaCha20Poly1305::new(key.into());
    let plaintext = cipher.decrypt(nonce.into(), ciphertext)?;
    Ok(plaintext)
}

pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    let salt_str = SaltString::generate(&mut OsRng);
    let hash = PasswordHash::generate(Argon2::default(), password, &salt_str)?;
    Ok(hash.to_string())
}

pub fn verify_hashed_password(password: &str, hash: &str) -> bool {
    let Ok(parsed_hash) = PasswordHash::new(hash) else {
        return false;
    };
    Argon2::default().verify_password(password.as_bytes(), &parsed_hash) == Ok(())
}

pub fn generate_secret_key() -> Vec<u8> {
    let key = ChaCha20Poly1305::generate_key(&mut OsRng);
    key.to_vec()
}

pub fn encrypt_key_with_pub_key(key: &[u8], pub_key: &RsaPublicKey) -> Result<Vec<u8>, rsa::Error> {
    let encrypted = pub_key.encrypt(&mut OsRng::default(), Pkcs1v15Encrypt, key)?;
    Ok(encrypted)
}

pub fn decrypt_key_with_priv_key(
    key: &[u8],
    priv_key: &RsaPrivateKey,
) -> Result<Vec<u8>, rsa::Error> {
    let decrypted = priv_key.decrypt(Pkcs1v15Encrypt, key)?;
    Ok(decrypted)
}
