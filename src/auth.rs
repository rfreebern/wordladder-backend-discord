use ed25519_dalek::{VerifyingKey, Signature};
use serde::{Deserialize};
use base64::{Engine as _, engine::general_purpose::URL_SAFE};

#[derive(Deserialize)]
pub struct Claims {
    pub guild_id: String,
    pub user_id: String,
    pub exp: u64,
}

pub fn verify(token: &str) -> Result<Claims, fastly::Error> {
    let key_hex = std::env::var("DISCORD_PUBLIC_KEY")
        .map_err(|_| fastly::Error::msg("DISCORD_PUBLIC_KEY not set"))?;
    
    let key_bytes = hex::decode(&key_hex)
        .map_err(|_| fastly::Error::msg("Invalid hex in public key"))?;
    
    let key_array: [u8; 32] = key_bytes.try_into()
        .map_err(|_| fastly::Error::msg("Invalid key length"))?;
    
    let pk = VerifyingKey::from_bytes(&key_array)
        .map_err(|_| fastly::Error::msg("Invalid public key"))?;
    
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        return Err(fastly::Error::msg("Invalid JWT format"));
    }
    
    let msg = format!("{}.{}", parts[0], parts[1]);
    let sig = Signature::from_slice(&URL_SAFE.decode(parts[2])
        .map_err(|_| fastly::Error::msg("Invalid signature encoding"))?)
        .map_err(|_| fastly::Error::msg("Invalid signature"))?;
    
    pk.verify_strict(msg.as_bytes(), &sig)
        .map_err(|_| fastly::Error::msg("Signature verification failed"))?;
    
    let body = URL_SAFE.decode(parts[1])
        .map_err(|_| fastly::Error::msg("Invalid body encoding"))?;
    
    let claims: Claims = serde_json::from_slice(&body)
        .map_err(|_| fastly::Error::msg("Invalid JSON in token"))?;
    
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    if claims.exp < now {
        return Err(fastly::Error::msg("Token expired"));
    }
    
    Ok(claims)
}
