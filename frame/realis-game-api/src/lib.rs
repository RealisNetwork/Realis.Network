#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame

use frame_support::{decl_module, decl_storage, decl_event, decl_error, ensure, dispatch, traits::{
    ExistenceRequirement, ExistenceRequirement::AllowDeath, StoredMap, WithdrawReasons, OnNewAccount, Get, OnUnbalanced,
}, Parameter, PalletId};
use sp_runtime::{traits::{AtLeast32BitUnsigned, Bounded, CheckedAdd, CheckedSub, Member, Saturating, StaticLookup,
                          StoredMapError, Zero,}, RuntimeDebug};
use frame_system::{ensure_signed, split_inner, RefCount, ensure_root};
use sp_std::prelude::*;
use pallet_nft::{Token, Params, Socket, Rarity, TokenId};
use pallet_nft as NFT;
use pallet_staking;
// use std::collections::HashSet;
use codec::{Decode, Encode, EncodeLike};

// extern crate clap;
// #[macro_use]
// extern crate prettytable;
// extern crate reqwest;
// extern crate serde;
// #[macro_use]
// extern crate serde_derive;
// extern crate serde_json;

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Config: frame_system::Config + pallet_nft::Config + pallet_balances::Config {
    /// Because this pallet emits events, it depends on the runtime's definition of an event.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;

    type PalletId: Get<PalletId>;
}


decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Config>::AccountId {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		TokenMinted(AccountId, TokenId),
		TokenTransferred(TokenId, AccountId),
        TokenBreeded(TokenId),
        // TokensTransferred(TokenId, AccountId, TokenId, AccountId),
	}
);

decl_error! {
	pub enum Error for Module<T: Config> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		///
		TokenExist,
		///
		NotTokenOwner,
		///
		NonExistentToken,
		///
		NotNftMaster
	}
}

pub struct DealWithTransactions;
impl OnUnbalanced<PositiveImbalance> for DealWithTransactions {
    fn on_unbalanceds<B>(mut fees_then_tips: impl Iterator<Item=NegativeImbalance>) {
        if let Some(value) = fees_then_tips.next() {
            // for fees, 80% to treasury, 20% to author
            let mut split = value.ration(80, 20);
            if let Some(value) = fees_then_tips.next() {
                // for tips, if any, 80% to treasury, 20% to author (though this can be anything)
                value.ration_merge_into(80, 20, &mut split);
            }
            T::account_id()::on_unbalanced(split.1);
            pallet_staking::Module::<T>::account_id().on_unbalanced(split.0);
        }
    }
}
decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
        const PalletId: PalletId = T::PalletId::get();
        		// Errors must be initialized if they are used by the pallet.
		type Error = Error<T>;

		// Events must be initialized if they are used by the pallet.
		fn deposit_event() = default;

        #[weight = 10_000]
        pub fn mint_nft(origin, target_account: <T as frame_system::Config>::AccountId, token_id: pallet_nft::TokenId) -> dispatch::DispatchResult {
           pallet_nft::Module::<T>::mint_basic_nft(&target_account, token_id)
           Ok(())
        }

        #[weight = 10_000]
        pub fn transfer_nft(origin, dest_account: T::AccountId, token_id: pallet_nft::TokenId) {
            NFT::Module::<T>::transfer_basic_nft(&dest_account, token_id)
        }

        #[weight = 10_000]
        pub fn transfer_from_pallet(T::account_id(), dest: <T::Lookup as StaticLookup>::Source, value: <T as pallet_balances::Config>::Balance) -> dispatch::DispatchResultWithPostInfo {
            pallet_support::Curency::transfer(T::account_id(), &dest, value, KeepAlive)
		}

        #[weight = 10_000]
        pub fn transfer_from_ptop(from: <T::Lookup as StaticLookup>::Source, to: <T::Lookup as StaticLookup>::Source, value: <T as pallet_balances::Config>::Balance) -> dispatch::DispatchResultWithPostInfo {
            pallet_support::Curency::transfer(from, to, value, KeepAlive)
		}

        #[weight = 10_000]
        pub fn burn_nft(origin, pallet_nft::TokenId) -> dispatch::DispatchResult {
            NFT::Module::<T>::burn_basic_nft(TokenId)
        }

        #[weight = 10_000]
        pub fn spend_in_game (T::account_id(), pallet_staking::Module::<T>::account_id()) {

        }
    }
}

impl<T: Config> Module<T> {
    pub fn account_id() -> T::AccountId {
        T::PalletId::get().into_account()
    }
}
// fn get_from_json() -> Token {
//     let client = reqwest::Client::new();
//     let mut request = client.get(URL);

//     let mut resp = request.send().unwrap();

//     assert!(resp.status().is_success());
//     return resp;
// }