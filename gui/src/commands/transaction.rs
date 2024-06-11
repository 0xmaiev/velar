use anyhow::anyhow;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig, signature::Signature, transaction::Transaction,
};
use std::sync::Arc;

use crate::Error;

/// Submits a [`Transaction`] to the solana network and returns it's [`Signature`].
///
/// # Errors
///
/// This function will return an error if there is an error submitting the [`Transaction`].
pub async fn submit_transaction(
    rpc_client: Arc<RpcClient>,
    tx: Transaction,
) -> Result<Signature, Error> {
    match rpc_client.send_transaction(&tx).await {
        Ok(sig) => Ok(sig),
        Err(e) => Err(Error::Client(e)),
    }
}

/// Confirms that the given transaction [`Signature`] is committed and returns a boolean
/// representing whether or not the transaction succeeded.
///
/// # Errors
///
/// This function will return an error if there is an error submitting the [`Transaction`].
pub async fn confirm_transaction(
    rpc_client: Arc<RpcClient>,
    signature: Signature,
) -> Result<bool, Error> {
    match rpc_client
        .confirm_transaction_with_commitment(&signature, CommitmentConfig::confirmed())
        .await
    {
        Ok(res) => Ok(res.value),
        Err(e) => Err(Error::Client(e)),
    }
}
