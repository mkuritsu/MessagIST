use std::path::Path;

use cryptolib::{
    DecodePrivateKey, DecodePublicKey, EncodePrivateKey, EncodePublicKey, LineEnding,
    RsaPrivateKey, RsaPublicKey,
};
use rand::rngs::OsRng;

pub async fn gen_key_pair(id: &str) -> anyhow::Result<(RsaPrivateKey, RsaPublicKey)> {
    let prk_path = format!("{}.priv", id);
    let puk_path = format!("{}.pub", id);
    let prk_path = Path::new(&prk_path);
    let puk_path = Path::new(&puk_path);
    let prk = if prk_path.exists() {
        let prk_pem = tokio::fs::read_to_string(prk_path).await?;
        RsaPrivateKey::from_pkcs8_pem(&prk_pem)?
    } else {
        let prk = RsaPrivateKey::new(&mut OsRng, 2048)?;
        let prk_pem = prk.to_pkcs8_pem(LineEnding::default())?;
        tokio::fs::write(&prk_path, prk_pem).await?;
        prk
    };
    let puk = if puk_path.exists() {
        let puk_pem = tokio::fs::read_to_string(puk_path).await?;
        RsaPublicKey::from_public_key_pem(&puk_pem)?
    } else {
        let puk = RsaPublicKey::from(&prk);
        let puk_pem = puk.to_public_key_pem(LineEnding::default())?;
        tokio::fs::write(&puk_path, puk_pem).await?;
        puk
    };
    Ok((prk, puk))
}
