//////////
// SSH //
////////

use std::{collections::HashMap, net::SocketAddr, sync::Arc};

use async_trait::async_trait;
use russh::{server::{Auth, Msg, Session}, Channel, ChannelId};
use tokio::sync::Mutex;

#[derive(Clone)]
pub struct Server;

impl russh::server::Server for Server {
    type Handler = SshSession;

    fn new_client(&mut self, client_ip: Option<SocketAddr>) -> Self::Handler {
        let mut ssh = SshSession::default();
        ssh.ip = client_ip;
        ssh
    }
}

pub struct SshSession {
    clients: Arc<Mutex<HashMap<ChannelId, Channel<Msg>>>>,
    ip: Option<SocketAddr>
}

impl Default for SshSession {
    fn default() -> Self {
        Self {
            clients: Arc::new(Mutex::new(HashMap::new())),
            ip: None,
        }
    }
}

#[async_trait]
impl russh::server::Handler for SshSession {
    type Error = anyhow::Error;

    #[allow(unused_variables)]
    async fn auth_password(&mut self, user: &str, password: &str) -> Result<Auth, Self::Error> {
        //info!("credentials: {}, {}", user, password);
        Ok(Auth::Accept)
    }

    #[allow(unused_variables)]
    async fn auth_publickey(
        &mut self,
        user: &str,
        public_key: &russh_keys::key::PublicKey,
    ) -> Result<Auth, Self::Error> {
        //info!("credentials: {}, {:?}", user, public_key);
        Ok(Auth::Accept)
    }

    #[allow(unused_variables)]
    async fn channel_open_session(
        &mut self,
        channel: Channel<Msg>,
        _session: &mut Session,
    ) -> Result<bool, Self::Error> {
        {
            let mut clients = self.clients.lock().await;
            clients.insert(channel.id(), channel);
        }
        Ok(true)
    }

    #[allow(unused_variables)]
    async fn subsystem_request(
        &mut self,
        channel_id: ChannelId,
        name: &str,
        session: &mut Session,
    ) -> Result<(), Self::Error> {
        Ok(())
    }
}