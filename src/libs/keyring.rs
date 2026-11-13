use keyring;

use crate::types::{DbConfig, config::StoredDbConfig};

const SERVICE_NAME : &str = "sync_flaplock";
const DB_KEY: &str = "sync_config";

pub fn save_config(cfg: &DbConfig) -> Result<(), String> {
    let stored = StoredDbConfig {
        host: cfg.host.clone(),
        port: cfg.port,
        // name: cfg.name.clone(),
        username: cfg.username.clone(),
        password: cfg.password.clone(),
    };
    let json = serde_json::to_string(&stored).map_err(|e| e.to_string())?;
    keyring::Entry::new(SERVICE_NAME, DB_KEY)
        .and_then(|e| e.set_password(&json))
        .map_err(|e| format!("Gagal menyimpan konfigurasi: {e}"))
}

pub fn load_config() -> Result<Option<DbConfig>, String> {
    let json = match keyring::Entry::new(SERVICE_NAME, DB_KEY).and_then(|e| e.get_password()) {
        Ok(v) => v,
        Err(keyring::Error::NoEntry) => return Ok(None),
        Err(e) => return Err(e.to_string()),
    };
    let s: StoredDbConfig = serde_json::from_str(&json).map_err(|e| e.to_string())?;
    Ok(Some(DbConfig {
        host: s.host,
        port: s.port,
        // name: s.name,
        username: s.username,
        password: s.password,
    }))
}

pub fn delete_config() -> Result<(), String> {
    match keyring::Entry::new(SERVICE_NAME, DB_KEY).and_then(|e| e.delete_credential()) {
        Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
        Err(e) => Err(format!("Gagal menghapus konfigurasi: {e}")),
    }
}