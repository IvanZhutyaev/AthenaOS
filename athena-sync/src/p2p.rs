use crate::protocol::SyncProtocol;
use anyhow::Result;
use async_trait::async_trait;
use libp2p::{
    identity, kad, noise, ping, relay,
    swarm::{NetworkBehaviour, SwarmEvent},
    tcp, yamux, Multiaddr, PeerId, Swarm,
};
use std::collections::HashSet;
use tokio::sync::mpsc;

#[derive(NetworkBehaviour)]
pub struct AthenaBehaviour {
    pub kad: kad::Behaviour<kad::store::MemoryStore>,
    pub ping: ping::Behaviour,
    pub relay: relay::Behaviour,
}

pub struct P2PNode {
    swarm: Swarm<AthenaBehaviour>,
    peers: HashSet<PeerId>,
    message_tx: mpsc::Sender<SyncProtocol>,
    message_rx: mpsc::Receiver<SyncProtocol>,
}

impl P2PNode {
    pub fn new() -> Result<Self> {
        let local_key = identity::Keypair::generate_ed25519();
        let local_peer_id = PeerId::from(local_key.public());

        let transport = tcp::tokio::Transport::default()
            .upgrade(yamux::Config::default())
            .authenticate(noise::Config::new(&local_key)?)
            .multiplex(yamux::Config::default())
            .boxed();

        let behaviour = AthenaBehaviour {
            kad: kad::Behaviour::new(
                local_peer_id,
                kad::store::MemoryStore::new(local_peer_id),
            ),
            ping: ping::Behaviour::new(ping::Config::new()),
            relay: relay::Behaviour::new(local_peer_id, Default::default()),
        };

        let swarm = Swarm::new(transport, behaviour, local_peer_id, Default::default());

        let (message_tx, message_rx) = mpsc::channel(100);

        Ok(Self {
            swarm,
            peers: HashSet::new(),
            message_tx,
            message_rx,
        })
    }

    pub async fn start_listening(&mut self, addr: Multiaddr) -> Result<()> {
        self.swarm.listen_on(addr)?;
        Ok(())
    }

    pub async fn dial(&mut self, addr: Multiaddr) -> Result<()> {
        self.swarm.dial(addr)?;
        Ok(())
    }

    pub async fn run(&mut self) -> Result<()> {
        use futures::StreamExt;
        loop {
            match self.swarm.select_next_some().await {
                SwarmEvent::NewListenAddr { address, .. } => {
                    tracing::info!("Listening on {}", address);
                }
                SwarmEvent::ConnectionEstablished { peer_id, .. } => {
                    tracing::info!("Connected to {}", peer_id);
                    self.peers.insert(peer_id);
                }
                SwarmEvent::ConnectionClosed { peer_id, .. } => {
                    tracing::info!("Disconnected from {}", peer_id);
                    self.peers.remove(&peer_id);
                }
                _ => {}
            }
        }
    }

    pub fn send_message(&self, message: SyncProtocol) -> Result<()> {
        self.message_tx.try_send(message)?;
        Ok(())
    }

    pub async fn receive_message(&mut self) -> Option<SyncProtocol> {
        self.message_rx.recv().await
    }

    /// Запустить P2P узел в фоновой задаче
    pub fn start_background(p2p_node: std::sync::Arc<tokio::sync::RwLock<Option<Self>>>) {
        tokio::spawn(async move {
            loop {
                let mut guard = p2p_node.write().await;
                if let Some(ref mut p2p) = *guard {
                    use futures::StreamExt;
                    match futures::stream::poll_fn(|cx| {
                        std::pin::Pin::new(&mut p2p.swarm).poll_next(cx)
                    })
                    .next()
                    .await
                    {
                        Some(libp2p::swarm::SwarmEvent::NewListenAddr { address, .. }) => {
                            tracing::info!("P2P: Listening on {}", address);
                        }
                        Some(libp2p::swarm::SwarmEvent::ConnectionEstablished { peer_id, .. }) => {
                            tracing::info!("P2P: Connected to {}", peer_id);
                        }
                        Some(libp2p::swarm::SwarmEvent::ConnectionClosed { peer_id, .. }) => {
                            tracing::info!("P2P: Disconnected from {}", peer_id);
                        }
                        Some(_) => {}
                        None => {
                            // Swarm закрыт, выходим
                            tracing::info!("P2P: Swarm closed");
                            break;
                        }
                    }
                } else {
                    // P2P узел удалён, выходим
                    tracing::info!("P2P: Node removed, stopping background task");
                    break;
                }
                // Освобождаем guard перед следующей итерацией
                drop(guard);
                tokio::time::sleep(tokio::time::Duration::from_millis(10)).await;
            }
        });
    }
}

#[async_trait]
pub trait P2PNodeTrait: Send + Sync {
    async fn start_listening(&mut self, addr: Multiaddr) -> Result<()>;
    async fn dial(&mut self, addr: Multiaddr) -> Result<()>;
    fn send_message(&self, message: SyncProtocol) -> Result<()>;
    async fn receive_message(&mut self) -> Option<SyncProtocol>;
}

