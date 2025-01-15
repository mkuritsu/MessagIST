use rsa::pkcs8::DecodePrivateKey;
use rsa::pkcs8::DecodePublicKey;
use rsa::pkcs8::EncodePrivateKey;
use rsa::pkcs8::EncodePublicKey;
use rsa::RsaPrivateKey;
use rsa::RsaPublicKey;

pub fn separate_cipher_nonce<'a>(data: &'a [u8]) -> (&'a [u8], &'a [u8]) {
    let len = data.len();
    let nonce_start = len - 12;
    let ciphertext = &data[..nonce_start];
    let nonce = &data[nonce_start..];
    (ciphertext, nonce)
}

pub fn join_cipher_nonce(ciphertext: &[u8], nonce: &[u8]) -> Vec<u8> {
    let mut vec = ciphertext.to_vec();
    vec.extend(nonce.to_vec());
    vec
}

pub fn public_key_to_bytes(key: RsaPublicKey) -> rsa::pkcs8::Result<Vec<u8>> {
    Ok(key.to_public_key_der()?.as_bytes().to_vec())
}

pub fn public_key_from_bytes(bytes: &[u8]) -> rsa::pkcs8::Result<RsaPublicKey> {
    Ok(RsaPublicKey::from_public_key_der(bytes)?)
}

pub fn private_key_to_bytes(key: RsaPrivateKey) -> rsa::pkcs8::Result<Vec<u8>> {
    Ok(key.to_pkcs8_der()?.as_bytes().to_vec())
}

pub fn private_key_from_bytes(bytes: &[u8]) -> rsa::pkcs8::Result<RsaPrivateKey> {
    Ok(RsaPrivateKey::from_pkcs8_der(bytes)?)
}
