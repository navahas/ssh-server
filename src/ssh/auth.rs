use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::Mutex;
use russh::keys::PublicKey;

#[derive(Debug, Clone, Default)]
pub struct AuthLog {
    entries: Arc<Mutex<HashSet<String>>>,
}

impl AuthLog {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn record_key(&self, user: &str, key: &PublicKey) -> bool {
        let fingerprint = key.fingerprint(Default::default()).to_string();
        let entry = format!("{user}:{fingerprint}");

        let mut entries = self.entries.lock().await;
        if entries.insert(entry.clone()) {
            log::info!("New public key login attempt: {entry}");
            true // New key, can persist to DB
        } else {
            false // Already logged
        }
    }

    pub async fn all_entries(&self) -> Vec<String> {
        let entries = self.entries.lock().await;
        entries.iter().cloned().collect()
    }
}
