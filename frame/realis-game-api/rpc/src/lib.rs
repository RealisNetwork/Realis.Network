use std::sync::Arc;

use codec::Codec;
use jsonrpc_core::{Error, ErrorCode, Result};
use jsonrpc_derive::rpc;
use serde::{Deserialize, Serialize};
use sp_api::ProvideRuntimeApi;
use sp_blockchain::HeaderBackend;
use sp_core::{Bytes, H256};
use sp_rpc::number::NumberOrHex;
use sp_runtime::{generic::BlockId, traits::{Block as BlockT, Header as HeaderT}, DispatchResult};
use std::convert::{TryFrom, TryInto};
use realis_primitives::*;

pub use realis_game_api_rpc_runtime_api::GameApi as GameRuntimeApi;

const RUNTIME_ERROR: i64 = 1;
const CONTRACT_DOESNT_EXIST: i64 = 2;
const CONTRACT_IS_A_TOMBSTONE: i64 = 3;

pub type Weight = u64;


#[rpc]
pub trait GameApi<BlockHash, BlockNumber, AccountId, Balance, Hash> {
    #[rpc(name = "mint_basic_nft")]
    fn mint_basic_nft(
        &self,
        call_request: AccountId,
        at: Option<BlockHash>,
    ) -> Result<DispatchResult>;
}