// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::{
    decl_error, decl_event, decl_module, decl_storage, dispatch::DispatchResult, ensure,
    traits::Get,
};
use frame_system::{ensure_root, ensure_signed};
use sp_core::U256;
use sp_runtime::RuntimeDebug;
use sp_std::prelude::*;
use realis_primitives::TokenId;

mod mock;
mod tests;

#[derive(PartialEq, Eq, Clone, Encode, Decode, RuntimeDebug)]
pub struct Erc721Token {
    pub id: TokenId,
    pub metadata: Vec<u8>,
}

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use frame_support::traits::{Currency, ExistenceRequirement};
    use sp_runtime::traits::{AtLeast32BitUnsigned, AccountIdConversion};
    use sp_core::H160;
    use frame_support::PalletId;

    type BalanceOf<T> =
    <<T as Config>::BridgeCurrency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::pallet]
    #[pallet::generate_store(pub (super) trait Store)]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::config]
    pub trait Config: frame_system::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        /// Some identifier for this token type, possibly the originating ethereum address.
        /// This is not explicitly used for anything, but may reflect the bridge's notion of resource ID.
        type BridgeCurrency: Currency<Self::AccountId, Balance = Self::Balance>;

        type Balance: Member + Parameter + AtLeast32BitUnsigned + Default + Copy;

        type PalletId: Get<PalletId>;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    pub enum Event<T: Config> {
        TransferTokenToBSC(T::AccountId, H160, BalanceOf<T>),
        TransferNftToBSC(T::AccountId, H160, TokenId),

        TransferTokenToRealis(H160, T::AccountId, BalanceOf<T>),
        TransferNftToRealis(H160, T::AccountId, TokenId),
    }

    #[pallet::error]
    pub enum Error<T> {
        /// ID not recognized
        TokenIdDoesNotExist,
        /// Already exists with an owner
        TokenAlreadyExists,
        /// Origin is not owner
        NotOwner,
    }

    #[pallet::genesis_config]
    pub struct GenesisConfig {}

    #[cfg(feature = "std")]
    impl Default for GenesisConfig {
        fn default() -> Self {
            Self {}
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig {
        fn build(&self) {

        }
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight((10000, Pays::No))]
        pub fn transfer_token_to_bsc(
            origin: OriginFor<T>,
            from: T::AccountId,
            to: H160,
            #[pallet::compact] value: T::Balance,
        ) -> DispatchResult {
            ensure_signed(origin)?;
            let pallet_id = Self::account_id();
            <T as Config>::BridgeCurrency::transfer(
                &from,
                &pallet_id,
                value,
                ExistenceRequirement::KeepAlive,
            )?;
            Self::deposit_event(Event::<T>::TransferTokenToBSC(from, to, value));
            Ok(())
        }

        #[pallet::weight((10000, Pays::No))]
        pub fn transfer_token_to_realis(
            origin: OriginFor<T>,
            from: H160,
            to: T::AccountId,
            #[pallet::compact] value: T::Balance,
        ) -> DispatchResult {
            ensure_signed(origin)?;
            let pallet_id = Self::account_id();
            <T as Config>::BridgeCurrency::transfer(
                &pallet_id,
                &to,
                value,
                ExistenceRequirement::KeepAlive,
            )?;
            Self::deposit_event(Event::<T>::TransferTokenToRealis(from, to, value));
            Ok(())
        }

        // #[pallet::weight((10000, Pays::No))]
        // fn transfer_nft_to_bsc(
        //     origin: OriginFor<T>,
        //     dest: H160,
        //     token_id: TokenId
        // ) -> DispatchResult {
        //     let who = ensure_signed(origin)?;
        //     // TODO implement logic
        //     let pallet_id = Self::account_id();
        //     //Nft::transfer(origin, pallet_id, token_id);
        //
        //     Self::deposit_event(Event::<T>::TransferNftToBSC(who, dest, token_id));
        //     Ok(())
        // }
        //
        // #[pallet::weight((10000, Pays::No))]
        // fn transfer_nft_to_realis(
        //     origin: OriginFor<T>,
        //     from: H160,
        //     token_id: TokenId
        // ) -> DispatchResult {
        //     let who = ensure_signed(origin)?;
        //     // TODO implement logic
        //     let pallet_id = Self::account_id();
        //     //Nft::transfer(Origin::signed(pallet_id), who, token_id);
        //
        //     Self::deposit_event(Event::<T>::TransferNftToRealis(from, who, token_id));
        //     Ok(())
        // }
    }

    impl<T: Config> Pallet<T> {
        pub fn account_id() -> T::AccountId {
            <T as Config>::PalletId::get().into_account()
        }
    }
}