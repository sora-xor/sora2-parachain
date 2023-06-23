//! A collection of node-specific RPC methods.
//! Substrate provides the `sc-rpc` crate, which defines the core RPC layer
//! used by Substrate nodes. This file extends those RPC definitions with
//! capabilities that are specific to this project's runtime configuration.

#![warn(missing_docs)]

use std::sync::Arc;

use parachain_template_runtime::{opaque::Block, AccountId, Balance, BlockNumber, Index as Nonce};

use beefy_light_client_rpc::{BeefyLightClientAPIServer, BeefyLightClientClient};
use sc_client_api::AuxStore;
pub use sc_rpc::{DenyUnsafe, SubscriptionTaskExecutor};
use sc_transaction_pool_api::TransactionPool;
use sp_api::ProvideRuntimeApi;
use sp_block_builder::BlockBuilder;
use sp_blockchain::{Error as BlockChainError, HeaderBackend, HeaderMetadata};

/// A type representing all RPC extensions.
pub type RpcExtension = jsonrpsee::RpcModule<()>;

/// Dependencies for BEEFY
pub struct BeefyDeps {
    /// Receives notifications about finality proof events from BEEFY.
    pub beefy_finality_proof_stream:
        beefy_gadget::communication::notification::BeefyVersionedFinalityProofStream<Block>,
    /// Receives notifications about best block events from BEEFY.
    pub beefy_best_block_stream:
        beefy_gadget::communication::notification::BeefyBestBlockStream<Block>,
    /// Executor to drive the subscription manager in the BEEFY RPC handler.
    pub subscription_executor: sc_rpc::SubscriptionTaskExecutor,
}

/// Full client dependencies
pub struct FullDeps<C, P, B> {
    /// The client instance to use.
    pub client: Arc<C>,
    /// The backend instance to use.
    pub backend: Arc<B>,
    /// Transaction pool instance.
    pub pool: Arc<P>,
    /// Whether to deny unsafe calls
    pub deny_unsafe: DenyUnsafe,
    /// BEEFY specific dependencies.
    pub beefy: BeefyDeps,
}

/// Instantiate all RPC extensions.
pub fn create_full<C, P, B>(
    deps: FullDeps<C, P, B>,
) -> Result<RpcExtension, Box<dyn std::error::Error + Send + Sync>>
where
    C: ProvideRuntimeApi<Block>
        + HeaderBackend<Block>
        + AuxStore
        + HeaderMetadata<Block, Error = BlockChainError>
        + Send
        + Sync
        + 'static,
    C::Api: beefy_light_client_rpc::BeefyLightClientRuntimeAPI<Block, beefy_light_client::BitField>,
    C::Api: pallet_transaction_payment_rpc::TransactionPaymentRuntimeApi<Block, Balance>,
    C::Api: substrate_frame_rpc_system::AccountNonceApi<Block, AccountId, Nonce>,
    C::Api: mmr_rpc::MmrRuntimeApi<Block, <Block as sp_runtime::traits::Block>::Hash, BlockNumber>,
    C::Api: leaf_provider_rpc::LeafProviderRuntimeAPI<Block>,
    C::Api: sp_beefy::BeefyApi<Block>,
    C::Api: BlockBuilder<Block>,
    P: TransactionPool + Sync + Send + 'static,
    B: sc_client_api::Backend<Block> + Send + Sync + 'static,
    B::State: sc_client_api::StateBackend<sp_runtime::traits::HashFor<Block>>,
{
    use beefy_gadget_rpc::{Beefy, BeefyApiServer};
    use bridge_channel_rpc::{BridgeChannelAPIServer, BridgeChannelClient};
    use leaf_provider_rpc::{LeafProviderAPIServer, LeafProviderClient};
    use mmr_rpc::{Mmr, MmrApiServer};
    use pallet_transaction_payment_rpc::{TransactionPayment, TransactionPaymentApiServer};
    use substrate_frame_rpc_system::{System, SystemApiServer};

    let mut module = RpcExtension::new(());
    let FullDeps { client, pool, deny_unsafe, beefy, backend } = deps;

    // Default RPC:
    module.merge(System::new(client.clone(), pool, deny_unsafe).into_rpc())?;
    module.merge(TransactionPayment::new(client.clone()).into_rpc())?;

    // Beefy and MMR:
    module.merge(Mmr::new(client.clone()).into_rpc())?;
    module.merge(
        Beefy::<Block>::new(
            beefy.beefy_finality_proof_stream,
            beefy.beefy_best_block_stream,
            beefy.subscription_executor,
        )?
        .into_rpc(),
    )?;
    if let Some(storage) = backend.offchain_storage() {
        module.merge(<BridgeChannelClient<_, _> as BridgeChannelAPIServer<
            bridge_types::types::BridgeOffchainData<
                parachain_template_runtime::BlockNumber,
                parachain_template_runtime::BridgeMaxMessagesPerCommit,
                parachain_template_runtime::BridgeMaxMessagePayloadSize,
            >,
        >>::into_rpc(BridgeChannelClient::new(storage)))?;
    }

    module.merge(LeafProviderClient::new(client.clone()).into_rpc())?;
    module.merge(BeefyLightClientClient::new(client).into_rpc())?;
    Ok(module)
}
