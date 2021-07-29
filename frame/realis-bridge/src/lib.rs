// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use frame_support::{dispatch::DispatchResult, ensure, traits::Get};
use frame_system::ensure_signed;
pub use realis_primitives::{TokenId, TokenType};
use sp_runtime::RuntimeDebug;
use sp_std::prelude::*;

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
    use frame_support::traits::{Currency, ExistenceRequirement};
    use frame_support::PalletId;
    use frame_system::pallet_prelude::*;
    use pallet_nft as Nft;
    use realis_primitives::{Token, TokenType};
    use sp_core::H160;
    use sp_runtime::traits::Zero;
    use sp_runtime::traits::{AccountIdConversion, Saturating};

    pub type BalanceOf<T> = <<T as Config>::BridgeCurrency as Currency<
        <T as frame_system::Config>::AccountId,
    >>::Balance;

    #[pallet::pallet]
    #[pallet::generate_store(pub (super) trait Store)]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::config]
    pub trait Config: frame_system::Config + Nft::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        /// Some identifier for this token type, possibly the originating ethereum address.
        /// This is not explicitly used for anything, but may reflect the bridge's notion of resource ID.
        type BridgeCurrency: Currency<Self::AccountId, Balance = Self::Balance>;

        type PalletId: Get<PalletId>;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    pub enum Event<T: Config> {
        ///Token was tranfered to BEP-20 on BSC
        TransferTokenToBSC(T::AccountId, H160, BalanceOf<T>),
        ///NFT was tranfered to BEP-721 on BSC
        TransferNftToBSC(T::AccountId, H160, TokenId),

        ///Token was tranfered to Realis.Network from BEP-20 on BSC
        TransferTokenToRealis(T::AccountId, BalanceOf<T>),
        ///NFT was tranfered to Realis.Network from BEP-721 on BSC
        TransferNftToRealis(T::AccountId, TokenId),
        Balance(T::AccountId, BalanceOf<T>),
    }

    #[pallet::error]
    pub enum Error<T> {
        /// ID not recognized
        TokenIdDoesNotExist,
        /// Already exists with an owner
        TokenAlreadyExists,
        /// Origin is not owner
        NotOwner,
        /// Not enought balance
        InsufficientBalance,
        /// Not Nft master
        NotNftMaster,
        /// Haven`t permission at this token
        NotTokenOwner,
        /// No such token exists
        NonExistentToken,
        /// Token wasnt transferred to BEP-20
        TokensWasntTransfered,
        /// NFT wasnt trasnferred to BEP-721
        NFTWasntTransfered,
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
        fn build(&self) {}
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(10000)]
        pub fn transfer_token_to_bsc(
            origin: OriginFor<T>,
            from: T::AccountId,
            to: H160,
            #[pallet::compact] value: T::Balance,
        ) -> DispatchResult {
            ensure_signed(origin)?;
            let zero = T::Balance::zero();
            if value == zero {
                return Err(sp_runtime::DispatchError::Other("InsufficientBalance"));
            }

            Self::deposit_event(Event::<T>::TransferTokenToBSC(from, to, value));
            Ok(())
        }

        #[pallet::weight(10000)]
        pub fn transfer_token_to_realis(
            origin: OriginFor<T>,
            to: T::AccountId,
            #[pallet::compact] value: T::Balance,
        ) -> DispatchResult {
            ensure_signed(origin)?;
            let zero = T::Balance::zero();
            if value == zero {
                return Err(sp_runtime::DispatchError::Other("InsufficientBalance"));
            }
            let pallet_id = Self::account_id();
            <T as Config>::BridgeCurrency::transfer(
                &pallet_id,
                &to,
                value,
                ExistenceRequirement::KeepAlive,
            )?;

            Self::deposit_event(Event::<T>::TransferTokenToRealis(to, value));
            Ok(())
        }

        #[pallet::weight(90000000)]
        pub fn balance_pallet(origin: OriginFor<T>) -> DispatchResult {
            let _who = ensure_signed(origin)?;
            let account_id = Self::account_id();
            let balance = <T as Config>::BridgeCurrency::free_balance(&account_id)
                .saturating_sub(<T as Config>::BridgeCurrency::minimum_balance());
            Self::deposit_event(Event::Balance(account_id, balance));
            Ok(())
        }

        #[pallet::weight(10000)]
        pub fn transfer_nft_to_bsc(
            origin: OriginFor<T>,
            from: T::AccountId,
            dest: H160,
            token_id: TokenId,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            // Only owner can transfer token
            ensure!(who == from, Error::<T>::NotTokenOwner);

            Self::deposit_event(Event::<T>::TransferNftToBSC(from, dest, token_id));
            Ok(())
        }

        #[pallet::weight(10000)]
        pub fn transfer_nft_to_realis(
            origin: OriginFor<T>,
            to: T::AccountId,
            token_id: TokenId,
            token_type: u8,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let nft_master = Nft::NftMasters::<T>::get();
            ensure!(nft_master.contains(&who), Error::<T>::NotNftMaster);
            let token = Token {
                id: token_id,
                token_type: TokenType::Basic(token_type),
            };
            Nft::Pallet::<T>::mint_basic_nft(&who, token_id, token)?;

            Self::deposit_event(Event::<T>::TransferNftToRealis(to, token_id));
            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        pub fn account_id() -> T::AccountId {
            <T as Config>::PalletId::get().into_account()
        }

        pub fn transfer_token_to_bsc_success(from: T::AccountId, value: T::Balance) {
            let pallet_id = Self::account_id();
            <T as Config>::BridgeCurrency::transfer(
                &from,
                &pallet_id,
                value,
                ExistenceRequirement::KeepAlive,
            );
        }

        pub fn transfer_token_to_bsc_error() -> crate::Error<T> {
            return Error::<T>::TokensWasntTransfered;
        }

        pub fn transfer_nft_to_bsc_success(from: T::AccountId, token_id: TokenId) {
            Nft::Pallet::<T>::burn_basic_nft(token_id, Some(from));
        }

        pub fn transfer_nft_to_bsc_error() -> crate::Error<T> {
            return Error::<T>::NFTWasntTransfered;
        }
    }
}
