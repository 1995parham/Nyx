use anyhow::{Context, Result};
use base64::{engine::general_purpose, Engine as _};
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey};

pub struct KeyPair {
    pub public_key: RsaPublicKey,
    pub private_key: RsaPrivateKey,
}

pub fn generate_key_pair() -> Result<KeyPair> {
    let mut rng = rand::thread_rng();
    let private_key = RsaPrivateKey::new(&mut rng, 2048)
        .context("Failed to generate RSA private key")?;
    let public_key = RsaPublicKey::from(&private_key);

    Ok(KeyPair {
        public_key,
        private_key,
    })
}

pub fn encrypt_content(content: &str, public_key: &RsaPublicKey) -> Result<String> {
    let mut rng = rand::thread_rng();
    let encrypted_data = public_key
        .encrypt(&mut rng, Pkcs1v15Encrypt, content.as_bytes())
        .context("Failed to encrypt content")?;

    Ok(general_purpose::STANDARD.encode(encrypted_data))
}

pub fn decrypt_content(encrypted_content: &str, private_key: &RsaPrivateKey) -> Result<String> {
    let encrypted_data = general_purpose::STANDARD
        .decode(encrypted_content)
        .context("Failed to decode base64 encrypted content")?;

    let decrypted_data = private_key
        .decrypt(Pkcs1v15Encrypt, &encrypted_data)
        .context("Failed to decrypt content")?;

    String::from_utf8(decrypted_data).context("Failed to convert decrypted data to string")
}

pub fn serialize_private_key(private_key: &RsaPrivateKey) -> Result<String> {
    use rsa::pkcs8::EncodePrivateKey;
    let private_key_pem = private_key
        .to_pkcs8_pem(rsa::pkcs8::LineEnding::LF)
        .context("Failed to serialize private key")?;
    Ok(private_key_pem.to_string())
}

pub fn deserialize_private_key(private_key_pem: &str) -> Result<RsaPrivateKey> {
    use rsa::pkcs8::DecodePrivateKey;
    RsaPrivateKey::from_pkcs8_pem(private_key_pem)
        .context("Failed to deserialize private key")
}