#![allow(dead_code)]
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

pub const DEFAULT_SOLANA_DIR: &str = "/.config/solana";

pub const DEFAULT_SOLANA_WALLET: &str = "id.json";
