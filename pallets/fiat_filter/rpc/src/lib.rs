// use frame_support::Parameter;
use jsonrpc_core::{Error as RpcError, ErrorCode, Result};
use jsonrpc_derive::rpc;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_core::Bytes;
use sp_rpc::number::NumberOrHex;
use sp_runtime::codec::{Codec, Decode};
use sp_runtime::generic::BlockId;
use sp_runtime::traits::{Block as BlockT, MaybeDisplay};
use std::convert::TryInto;
use std::sync::Arc;

pub use fiat_filter_rpc_runtime_api::FiatFeeRuntimeApi;

// TODO: rpc method that accepts a scale-encoded call
// returns fee in fiat usd_cent as u32
#[rpc]
pub trait FiatFeeApi<Balance> {
    #[rpc(name = "get_call_fee_dock")]
    fn get_call_fee_dock(&self, encoded_xt: Bytes) -> Result<Balance>;
}

/// Error type of this RPC api.
pub enum FiatFeeRpcError {
    /// The transaction was not decodable.
    DecodeError,
    /// The call to runtime failed.
    RuntimeError,
}
impl From<FiatFeeRpcError> for i64 {
    fn from(e: FiatFeeRpcError) -> i64 {
        match e {
            FiatFeeRpcError::RuntimeError => 1,
            FiatFeeRpcError::DecodeError => 2,
        }
    }
}

/// A struct that implements the FiatFeeApi
pub struct FiatFeeServer<Client, Block> {
    client: Arc<Client>,
    _marker_block: std::marker::PhantomData<Block>,
}
impl<Client, Block> FiatFeeServer<Client, Block> {
    /// Create new `SumStorage` instance with the given reference to the client.
    pub fn new(client: Arc<Client>) -> Self {
        Self {
            client,
            _marker_block: Default::default(),
        }
    }
}
impl<Client, Block, Balance> FiatFeeApi<Balance> for FiatFeeServer<Client, Block>
where
    Block: BlockT,
    Client: Send + Sync + 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
    Client::Api: FiatFeeRuntimeApi<Block, Balance>,
    Balance: Codec + MaybeDisplay + Copy + TryInto<NumberOrHex>,
    // Call: Parameter,
{
    // -> Result<RuntimeDispatchInfo<Balance>>
    fn get_call_fee_dock(&self, encoded_xt: Bytes) -> Result<Balance> {
        let api = self.client.runtime_api();
        // automatically pick the latest/best block
        let at = BlockId::<Block>::hash(self.client.info().best_hash);

        // decode extrinsic
        let uxt: Block::Extrinsic = Decode::decode(&mut &*encoded_xt).map_err(|e| RpcError {
            code: ErrorCode::ServerError(FiatFeeRpcError::DecodeError.into()),
            message: "Unable to query dispatch info.".into(),
            data: Some(format!("{:?}", e).into()),
        })?;

        // call runtime api method get_call_fee_dock()
        api.get_call_fee_dock(&at, uxt).map_err(|e| RpcError {
            code: ErrorCode::ServerError(FiatFeeRpcError::RuntimeError.into()),
            message: "Unable to query dispatch info.".into(),
            data: Some(format!("{:?}", e).into()),
        })
    }

    // fn get_call_fee_dock(&self, call: Call) -> Result<u32> {
    //     let api = self.client.runtime_api();
    //     // automatically pick the latest/best block
    //     let at = BlockId::<Block>::hash(self.client.info().best_hash);

    //     // TODO actually call runtime api method
    //     Ok(7)
    // }
}
