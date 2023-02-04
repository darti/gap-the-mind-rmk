use std::sync::Arc;

use async_trait::async_trait;
use log::info;
use russh::client::Handler;
use russh_keys::key::PublicKey;

#[derive(Default)]
struct Client {
    check_server_key: bool,
}

#[async_trait]
impl Handler for Client {
    type Error = anyhow::Error;

    async fn check_server_key(
        self,
        server_public_key: &PublicKey,
    ) -> Result<(Self, bool), Self::Error> {
        let valid = !self.check_server_key;

        Ok((self, valid))
    }
}

pub async fn init_connect(host: &str, port: u16, user: &str, password: &str) -> anyhow::Result<()> {
    let config = Arc::new(russh::client::Config::default());
    let client = Client::default();

    let mut session = russh::client::connect(config, (host, port), client)
        .await
        .unwrap();

    session.authenticate_password(user, password).await.unwrap();
    info!("Authenticated");

    Ok(())
}
