#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame
use frame_support::{
    decl_error, decl_event, decl_module, decl_storage, dispatch, ensure,
    traits::{
        ExistenceRequirement, ExistenceRequirement::AllowDeath, Get, OnNewAccount, OnUnbalanced,
        StoredMap, WithdrawReasons,
    },
    PalletId, Parameter,
};
use sp_std::prelude::*;
// use std::collections::HashSet;
use codec::{Decode, Encode, EncodeLike};

// pub struct DealWithTransactions;
// impl OnUnbalanced<PositiveImbalance> for DealWithTransactions {
//     fn on_unbalanceds<B>(mut fees_then_tips: impl Iterator<Item=NegativeImbalance>) {
//         if let Some(value) = fees_then_tips.next() {
//             // for fees, 80% to treasury, 20% to author
//             let mut split = value.ration(80, 20);
//             if let Some(value) = fees_then_tips.next() {
//                 // for tips, if any, 80% to treasury, 20% to author (though this can be anything)
//                 value.ration_merge_into(80, 20, &mut split);
//             }
//             T::account_id()::on_unbalanced(split.1);
//             pallet_staking::Module::<T>::account_id().on_unbalanced(split.0);
//         }
//     }
// }

// 1. Imports and Dependencies
pub use frame_system::pallet::*;
#[frame_support::pallet]
pub mod pallet {
    use codec::Codec;
    use frame_support::pallet_prelude::*;
    use frame_support::traits::{Currency, ExistenceRequirement};
    use frame_support::PalletId;
    use frame_system::pallet_prelude::*;
    use pallet_nft as NFT;
    use pallet_nft::Error::KeepAlive;
    use pallet_nft::{Params, Rarity, Socket, Token, TokenId};
    use pallet_staking::*;
    use sp_runtime::traits::{AccountIdConversion, AtLeast32BitUnsigned};
    use sp_std::fmt::Debug;
    use frame_support::dispatch::{GetDispatchInfo, Dispatchable};

    // 2. Declaration of the Pallet type
    // This is a placeholder to implement traits and methods.
    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    // 3. Runtime Configuration Trait
    // All types and constants go here.
    // Use #[pallet::constant] and #[pallet::extra_constants]
    // to pass in values to metadata.
    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_nft::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        #[pallet::constant]
        type PalletId: Get<PalletId>;

        type Currency: Currency<Self::AccountId, Balance = Self::Balance>;
    }

    // 5. Runtime Events
    // Can stringify event types to metadata.
    #[pallet::event]
    #[pallet::metadata(T::AccountId = "AccountId")]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Event documentation should end with an array that provides descriptive names for event
        /// parameters. [something, who]
        TokenMinted,
        TokenTransferred,
        TokenBurned,
        FundTransferred,
    }

    #[pallet::error]
    pub enum Error<T> {
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
        NotNftMaster,
    }

    // 6. Hooks
    // Define some logic that should be executed
    // regularly in some context, for e.g. on_initialize.
    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    // 7. Extrinsics
    // Functions that are callable from outside the runtime.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(90_000_000)]
        pub fn mint_nft(
            origin: OriginFor<T>,
            target_account: T::AccountId,
            token_id: pallet_nft::TokenId,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            NFT::Module::<T>::mint_basic_nft(&target_account, token_id);
            Self::deposit_event(Event::<T>::TokenMinted);
            Ok(())
        }

        #[pallet::weight(60_000_000)]
        pub fn transfer_nft(
            origin: OriginFor<T>,
            dest_account: T::AccountId,
            token_id: pallet_nft::TokenId,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            NFT::Module::<T>::transfer_basic_nft(&dest_account, token_id);
            Self::deposit_event(Event::<T>::TokenTransferred);
            Ok(())
        }

        // #[pallet::weight(50_000_000)]
        // pub fn transfer_from_pallet(origin: OriginFor<T>, pallet_id: T::account_id(), dest: <T::Lookup as StaticLookup>::Source, value: <T as pallet_balances::Config>::Balance) -> DispatchResultWithPostInfo {
        //     frame_support::Curency::transfer(T::account_id(), &dest, value, KeepAlive)
        // }

        #[pallet::weight(30_000_000)]
        pub fn transfer_from_ptop(
            origin: OriginFor<T>,
            from: T::AccountId,
            to: T::AccountId,
            value: T::Balance,
        ) -> DispatchResult {
            T::Currency::transfer(&from, &to, value, ExistenceRequirement::KeepAlive);
            Self::deposit_event(Event::<T>::FundTransferred);
            Ok(())
        }

        #[pallet::weight(90_000_000)]
        pub fn burn_nft(origin: OriginFor<T>, token_id: pallet_nft::TokenId) -> DispatchResult {
            let who = ensure_signed(origin)?;
            NFT::Module::<T>::burn_basic_nft(token_id);
            Self::deposit_event(Event::<T>::TokenBurned);
            Ok(())
        }

        // #[pallet::weight(90_000_000)]
        // pub fn spend_in_game (origin: OriginFor<T>, pallet_id: ::account_id(), pallet_id_staking: pallet_staking::Module::<T>::account_id()) -> DispatchResultWithPostInfo {
        //
        //     }
    }

    impl<T: Config> Pallet<T> {
        pub fn account_id() -> T::AccountId {
            T::PalletId::get().into_account()
        }
    }
}
