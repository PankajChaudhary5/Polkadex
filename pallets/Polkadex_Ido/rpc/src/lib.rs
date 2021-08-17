
use sp_runtime::{
    generic::BlockId,
    traits::{Block as BlockT},
};
use std::sync::Arc;
use jsonrpc_core::{Result, Error as RpcError, ErrorCode};
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use jsonrpc_derive::rpc;
pub use polkadex_ido_runtime_api::PolkadexIdoRuntimeApi;
use codec::Codec;
use pallet_polkadex_ido_primitives::FundingRoundWithPrimitives;

const RUNTIME_ERROR: i64 = 1;

#[rpc]
pub trait PolkadexIdoRpcApi<BlockHash,AccountId,Hash> {
    #[rpc(name = "polkadexido_getRoundsByInvestor")]
    fn get_rounds_by_investor( &self,account : AccountId, at: Option<BlockHash>) -> Result<Vec<(Hash, FundingRoundWithPrimitives<AccountId>)>>;
    #[rpc(name = "polkadexido_getRoundsByCreator")]
    fn get_rounds_by_creator( &self,account : AccountId, at: Option<BlockHash>) -> Result<Vec<(Hash, FundingRoundWithPrimitives<AccountId>)>>;
}

/// A struct that implements the `SumStorageApi`.
pub struct PolkadexIdoRpc<C, M> {
    client: Arc<C>,
    _marker: std::marker::PhantomData<M>,
}

impl<C, M> PolkadexIdoRpc<C, M> {
    /// Create new `SumStorage` instance with the given reference to the client.
    pub fn new(client: Arc<C>) -> Self {
        Self { client, _marker: Default::default() }
    }
}


impl<C, Block, AccountId, Hash> PolkadexIdoRpcApi<<Block as BlockT>::Hash,  AccountId, Hash>
for PolkadexIdoRpc<C, Block>
    where
        Block: BlockT,
        C: Send + Sync + 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
        C::Api: PolkadexIdoRuntimeApi<Block, AccountId, Hash>,
        AccountId : Codec,
        Hash : Codec
{
    fn get_rounds_by_investor(
        &self,
        account : AccountId,
        at: Option<<Block as BlockT>::Hash>
    ) -> Result<Vec<(Hash, FundingRoundWithPrimitives<AccountId>)>> {

        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(||
            // If the block hash is not supplied assume the best block.
            self.client.info().best_hash
        ));

        let runtime_api_result = api.rounds_by_investor(&at,account);
        runtime_api_result.map_err(runtime_error_into_rpc_err)
    }

    fn get_rounds_by_creator(
        &self,
        account : AccountId,
        at: Option<<Block as BlockT>::Hash>
    ) -> Result<Vec<(Hash, FundingRoundWithPrimitives<AccountId>)>> {

        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(||
            // If the block hash is not supplied assume the best block.
            self.client.info().best_hash
        ));

        let runtime_api_result = api.rounds_by_creator(&at, account);
        runtime_api_result.map_err(runtime_error_into_rpc_err)
    }
}

/// Converts a runtime trap into an RPC error.
fn runtime_error_into_rpc_err(err: impl std::fmt::Debug) -> RpcError {
    RpcError {
        code: ErrorCode::ServerError(RUNTIME_ERROR),
        message: "Runtime error".into(),
        data: Some(format!("{:?}", err).into()),
    }
}

