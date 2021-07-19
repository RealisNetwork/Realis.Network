use jsonrpc_core::Result;
use jsonrpc_derive::rpc;
use sp_runtime::DispatchResult;

pub use realis_game_api_rpc_runtime_api::GameApi as GameRuntimeApi;

pub type Weight = u64;

#[rpc]
pub trait GameApi<BlockHash, BlockNumber, AccountId, Balance, Hash> {
    #[rpc(name = "mint_basic_nft")]
    fn mint_basic_nft(
        &self,
        call_request: AccountId,
        at: Option<BlockHash>,
    ) -> Result<DispatchResult>;

    #[rpc(name = "transfer_from_pallet")]
    fn transfer_from_pallet(
        &self,
        dest: AccountId,
        value: Balance,
    ) -> Result<DispatchResult>;
}
