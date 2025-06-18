mod ssh;

use std::sync::Arc;
use rand_core::OsRng;
use russh::*;
use ssh::server::SshServer;

#[tokio::main]
async fn main() {
    let host: &str = "0.0.0.0";
    let port: u16 = 2222;

    env_logger::builder()
        .filter_level(log::LevelFilter::Info)
        .init();

    let config = server::Config {
        inactivity_timeout: Some(std::time::Duration::from_secs(3600)),
        auth_rejection_time: std::time::Duration::from_secs(3),
        auth_rejection_time_initial: Some(std::time::Duration::from_secs(0)),
        keys: vec![
            keys::PrivateKey::random(&mut OsRng, keys::Algorithm::Ed25519).unwrap(),
        ],
        preferred: Preferred {
            // kex: std::borrow::Cow::Owned(vec![kex::DH_GEX_SHA256]),
            ..Preferred::default()
        },
        ..Default::default()
    };

    let config = Arc::new(config);
    let server = SshServer::new();

    log::info!("SSH server running at {}:{}", host, port);
    server.run(config, (host, port)).await.unwrap();
}
