use std::sync::Arc;

use async_trait::async_trait;

use russh::{
    client::{Handle, Handler},
    Disconnect,
};

use russh_keys::key::PublicKey;

#[derive(Default)]
struct Client {
    check_server_key: bool,
}

#[async_trait]
impl Handler for Client {
    type Error = russh::Error;

    async fn check_server_key(
        self,
        server_public_key: &PublicKey,
    ) -> Result<(Self, bool), Self::Error> {
        let valid = !self.check_server_key;

        Ok((self, valid))
    }
}

pub struct Session {
    session: Handle<Client>,
}

impl Session {
    pub async fn connect(
        host: &str,
        port: u16,
        user: impl Into<String>,
        password: &str,
    ) -> anyhow::Result<Self> {
        let config = Arc::new(russh::client::Config::default());
        let client = Client::default();

        let mut session = russh::client::connect(config, (host, port), client).await?;

        let _auth = session.authenticate_password(user, password).await?;

        Ok(Self { session })
    }

    async fn close(&mut self) -> anyhow::Result<()> {
        self.session
            .disconnect(Disconnect::ByApplication, "", "English")
            .await?;
        Ok(())
    }
}
