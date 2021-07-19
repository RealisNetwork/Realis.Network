#![cfg_attr(not(feature = "std"), no_std)]

use codec::Codec;
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

        fn burn_basic_nft(origin: AccountId, token_id: TokenId) -> DispatchResult;

        fn transfer_basic_nft(
            origin: AccountId,
            dest_account: AccountId,
            token_id: TokenId,
        ) -> DispatchResult;

        fn transfer_from_pallet(
            origin: AccountId,
            dest: AccountId,
            value: Balance,
        ) -> DispatchResult;

        fn transfer_to_pallet(
            origin: AccountId,
            from: AccountId,
            value: Balance,
        ) -> DispatchResult;

        fn transfer_from_ptp(
            origin: AccountId,
            from: AccountId,
            to: AccountId,
            value: Balance,
        ) -> DispatchResult;

        fn spend_in_game(
            origin: AccountId,
            from: AccountId,
            amount: Balance,
        ) -> DispatchResult;

    }
}
