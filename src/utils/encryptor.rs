extern crate bcrypt;
use bcrypt::{DEFAULT_COST, hash, verify};
use tokio::task;

pub async fn encrypt(password: String) -> Result<String, String> {
    let hashed = task::spawn_blocking(move || {
        hash(password, DEFAULT_COST)
    }).await
    .map_err(|e| e.to_string())?  
    .map_err(|e| e.to_string())?; 

    Ok(hashed)
}

pub async fn valid_password(password: String, password_hash: String) -> Result<bool, String> {
    let valid = task::spawn_blocking(move || {
        verify(password, password_hash.as_str())
    }).await
    .map_err(|e| e.to_string())?
    .map_err(|e| e.to_string())?;

    Ok(valid)
}
