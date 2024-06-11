mod procgen;
mod resource;

pub use procgen::*;

use anchor_lang::AccountDeserialize;
pub use realm::{Character, Direction, Position};
use solana_account_decoder::UiAccountEncoding;
use solana_client::{nonblocking::pubsub_client::PubsubClient, rpc_config::RpcAccountInfoConfig};
use solana_sdk::{commitment_config::CommitmentConfig, pubkey::Pubkey};
use std::sync::Arc;
use std::sync::RwLock;
use tokio::sync::broadcast::channel;
use tokio::sync::broadcast::Receiver;
use tokio_stream::StreamExt;

#[cfg(feature = "localnet")]
pub const PUBSUB_RPC_URL: &str = "ws://127.0.0.1:8900";
#[cfg(feature = "localnet")]
pub const JSON_RPC_URL: &str = "http://127.0.0.1:8899";

#[cfg(feature = "devnet")]
pub const PUBSUB_RPC_URL: &str =
    "wss://cypher-develope-1013.devnet.rpcpool.com/1a4c1f68-bf8e-4b64-9d41-8e5b4032ef21";
#[cfg(feature = "devnet")]
pub const JSON_RPC_URL: &str =
    "https://cypher-develope-1013.devnet.rpcpool.com/1a4c1f68-bf8e-4b64-9d41-8e5b4032ef21";

#[cfg(feature = "mainnet")]
pub const PUBSUB_RPC_URL: &str = "wss://glint.rpcpool.com/e9c697d4-c929-4dcb-83fe-9ee0d4d0c168";
#[cfg(feature = "mainnet")]
pub const JSON_RPC_URL: &str = "https://glint.rpcpool.com/e9c697d4-c929-4dcb-83fe-9ee0d4d0c168";

pub const TILE_SIZE: usize = 1;

#[derive(Debug, Clone)]
pub struct PlayerContext {
    player: Pubkey,
    state: Arc<RwLock<Character>>,
}

impl Default for PlayerContext {
    fn default() -> Self {
        Self {
            player: Pubkey::default(),
            state: Arc::default(),
        }
    }
}

impl PlayerContext {
    pub fn new(owner: Pubkey) -> Self {
        let (player, _) = Character::derive(&owner);
        Self {
            player,
            ..Default::default()
        }
    }

    pub async fn subscribe(&self, pubsub_client: Arc<PubsubClient>) -> Receiver<String> {
        let (tx, rx) = channel(u16::MAX as usize);
        let player = self.player.clone();
        let state = self.state.clone();

        tokio::spawn(async move {
            let (mut stream, unsub) = pubsub_client
                .account_subscribe(
                    &player,
                    Some(RpcAccountInfoConfig {
                        encoding: Some(UiAccountEncoding::Base64),
                        commitment: Some(CommitmentConfig::processed()),
                        ..Default::default()
                    }),
                )
                .await
                .unwrap();

            loop {
                tokio::select! {
                    Some(account) = stream.next() => {
                        if let Some(decoded) = account.value.data.decode() {
                            let decoded_account = Character::try_deserialize(&mut decoded.as_ref()).unwrap();

                            *state.write().unwrap() = decoded_account;
                        }

                    }
                }
            }

            unsub().await;
        });

        rx
    }

    pub fn position(&self) -> Position {
        self.state.read().unwrap().position
    }
}
