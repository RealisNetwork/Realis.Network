use jsonrpc_derive::rpc;
use std::sync::Arc;
use sp_runtime::DispatchResult;
use sp_rpc::number::NumberOrHex;
use sp_runtime::{
    generic::BlockId,
    traits::{Block as BlockT, Header as HeaderT},
};
use jsonrpc_core::{Error, ErrorCode, Result};
use serde::{Deserialize, Serialize};
use std::convert::{TryFrom, TryInto};
pub use realis_game_api_rpc_runtime_api::RealisGameApi as GameRuntimeApi;
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use codec::Codec;
use sp_core::{Bytes, H256};

pub type Weight = u64;

const RUNTIME_ERROR: i64 = 1;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(deny_unknown_fields)]
pub struct CallRequest<AccountId> {
    origin: AccountId,
    dest: AccountId,
    value: NumberOrHex,
}

#[rpc]
pub trait GameApi<BlockHash, BlockNumber, AccountId, Balance, Hash> {
    #[rpc(name = "realisGameApi_mintBasicNft")]
    fn mint_basic_nft(
        &self,
        call_request: CallRequest<AccountId>,
        at: Option<BlockHash>,
    ) -> Result<DispatchResult>;

    #[rpc(name = "realisGameApi_transferFromPallet")]
    fn transfer_from_pallet(
        &self,
        call_request: CallRequest<AccountId>,
        at: Option<BlockHash>,
    ) -> Result<DispatchResult>;
}

pub struct Contracts<C, B> {
    client: Arc<C>,
    _marker: std::marker::PhantomData<B>,
}

impl<C, B> Contracts<C, B> {
    /// Create new `Contracts` with the given reference to the client.
    pub fn new(client: Arc<C>) -> Self {
        Contracts {
            client,
            _marker: Default::default(),
        }
    }
}

impl<C, Block, AccountId, Balance, Hash>
GameApi<
    <Block as BlockT>::Hash,
    <<Block as BlockT>::Header as HeaderT>::Number,
    AccountId,
    Balance,
    Hash,
> for Contracts<C, Block>
    where
        Block: BlockT,
        C: Send + Sync + 'static + ProvideRuntimeApi<Block> + HeaderBackend<Block>,
        C::Api: GameRuntimeApi<
            Block,
            AccountId,
            Balance,
            <<Block as BlockT>::Header as HeaderT>::Number,
            Hash,
        >,
        AccountId: Codec,
        Balance: Codec + TryFrom<NumberOrHex>,
        Hash: Codec,
{
    fn mint_basic_nft(
        &self,
        call_request: CallRequest<AccountId>,
        at: Option<<Block as BlockT>::Hash>,
    ) -> DispatchResult {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(||
            // If the block hash is not supplied assume the best block.
            self.client.info().best_hash));

        let CallRequest {
            origin,
            dest,
            value,
        } = call_request;

        let value: Balance = decode_hex(value, "balance")?;

        let exec_result = api
            .call(&at, origin, dest, value)
            .map_err(runtime_error_into_rpc_err)?;

        Ok(exec_result)
    }

    fn transfer_from_pallet(
        &self,
        call_request: CallRequest<AccountId>,
        at: Option<<Block as BlockT>::Hash>,
    ) -> DispatchResult {
        let api = self.client.runtime_api();
        let at = BlockId::hash(at.unwrap_or_else(||
            // If the block hash is not supplied assume the best block.
            self.client.info().best_hash));

        let CallRequest {
            origin,
            dest,
            value,
        } = call_request;

        let value: Balance = decode_hex(value, "balance")?;

        let exec_result = api
            .call(&at, origin, dest, value)
            .map_err(runtime_error_into_rpc_err)?;

        Ok(exec_result)
    }
}
fn runtime_error_into_rpc_err(err: impl std::fmt::Debug) -> Error {
    Error {
        code: ErrorCode::ServerError(RUNTIME_ERROR),
        message: "Runtime error".into(),
        data: Some(format!("{:?}", err).into()),
    }
}

fn decode_hex<H: std::fmt::Debug + Copy, T: TryFrom<H>>(from: H, name: &str) -> Result<T> {
    from.try_into().map_err(|_| Error {
        code: ErrorCode::InvalidParams,
        message: format!("{:?} does not fit into the {} type", from, name),
        data: None,
    })
}