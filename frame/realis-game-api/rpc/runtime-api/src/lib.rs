
#![cfg_attr(not(feature = "std"), no_std)]

use codec::Codec;
use sp_std::vec::Vec;
use realis_primitives::*;
use sp_runtime::DispatchResult;

sp_api::decl_runtime_apis! {
	/// The API to interact with contracts without using executive.
	pub trait GameApi<AccountId, Balance, BlockNumber, Hash> where
		AccountId: Codec,
		Balance: Codec,
		BlockNumber: Codec,
		Hash: Codec,
	{
		/// Perform a call from a specified account to a given contract.
        ///
        /// See `pallet_contracts::Pallet::call`.
		fn mint_basic_nft(
			origin: AccountId,
			target_account: AccountId,
			token_id: TokenId,
			basic: Basic,
		) -> DispatchResult;

	}
}