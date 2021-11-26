// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{dispatch::DispatchResult, ensure, traits::Get};
use frame_system::ensure_signed;
pub use realis_primitives::{Status, TokenId, TokenType};
use sp_std::prelude::*;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
mod mock;
mod tests;

pub mod weights;
pub use weights::WeightInfoBridge;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_support::traits::{Currency, ExistenceRequirement};
    use frame_support::PalletId;
    use frame_system::pallet_prelude::*;
    use pallet_nft as Nft;
    pub use realis_game_api::*;
    use sp_core::H160;
    use sp_runtime::traits::{AccountIdConversion, Saturating};

    pub type BalanceOf<T> = <<T as Config>::BridgeCurrency as Currency<
        <T as frame_system::Config>::AccountId,
    >>::Balance;

    #[pallet::pallet]
    #[pallet::generate_store(pub (super) trait Store)]
    pub struct Pallet<T>(PhantomData<T>);

    #[pallet::config]
    pub trait Config: frame_system::Config + Nft::Config + pallet_balances::Config {
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        /// Some identifier for this token type, possibly the originating ethereum address.
        /// This is not explicitly used for anything, but may reflect the bridge's notion of resource ID.
        type BridgeCurrency: Currency<Self::AccountId, Balance = Self::Balance>;

        type PalletId: Get<PalletId>;

        type WeightInfoBridge: WeightInfoBridge;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    pub enum Event<T: Config> {
        ///Token was tranfered to BEP-20 on BSC
        SendTokensToBsc(T::AccountId, H160, BalanceOf<T>, BalanceOf<T>),
        ///NFT was tranfered to BEP-721 on BSC
        TransferNftToBSC(T::AccountId, H160, TokenId),

        ///Token was tranfered to Realis.Network from BEP-20 on BSC
        GetTokensToRealis(H160, T::AccountId, BalanceOf<T>),
        ///NFT was tranfered to Realis.Network from BEP-721 on BSC
        TransferNftToRealis(H160, T::AccountId, TokenId),
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
        NotBridgeMaster,
        /// Bridge Master was added early
        BridgeMasterWasAddedEarly,
        /// Haven`t permission at this token
        NotTokenOwner,
        /// No such token exists
        NonExistentToken,
        /// Token wasnt transferred to BEP-20
        TokensWasntTransfered,
        /// NFT wasnt trasnferred to BEP-721
        NFTWasntTransfered,
        CannotTransferNftBecauseThisNftInMarketplace,
        CannotTransferNftBecauseThisNftOnAnotherUser,
    }

    #[pallet::storage]
    #[pallet::getter(fn bridge_masters)]
    pub type BridgeMasters<T: Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub bridge_masters: Vec<T::AccountId>,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                bridge_masters: Default::default(),
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            BridgeMasters::<T>::put(&self.bridge_masters);
        }
    }

    #[cfg(feature = "std")]
    impl<T: Config> GenesisConfig<T> {
        /// Direct implementation of `GenesisBuild::build_storage`.
        ///
        /// Kept in order not to break dependency.
        #[cfg(feature = "std")]
        pub fn build_storage(&self) -> Result<sp_runtime::Storage, std::string::String> {
            #[cfg(feature = "std")]
            <Self as GenesisBuild<T>>::build_storage(self)
        }
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(T::WeightInfoBridge::transfer_token_to_bsc())]
        pub fn transfer_token_to_bsc(
            origin: OriginFor<T>,
            to: H160,
            #[pallet::compact] value: T::Balance,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let balance = <T as Config>::BridgeCurrency::free_balance(&who);
            ensure!(balance > value, Error::<T>::InsufficientBalance);

            let pallet_id = Self::account_id();
            <T as Config>::BridgeCurrency::transfer(
                &who,
                &pallet_id,
                value,
                ExistenceRequirement::KeepAlive,
            )?;

            Self::deposit_event(Event::<T>::SendTokensToBsc(who, to, value, balance));
            Ok(())
        }

        #[pallet::weight(T::WeightInfoBridge::transfer_token_to_realis())]
        pub fn transfer_token_to_realis(
            origin: OriginFor<T>,
            from: H160,
            to: T::AccountId,
            #[pallet::compact] value: T::Balance,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(
                Self::bridge_masters().contains(&who),
                Error::<T>::NotBridgeMaster
            );
            let pallet_id = Self::account_id();
            <T as Config>::BridgeCurrency::transfer(
                &pallet_id,
                &to,
                value,
                ExistenceRequirement::AllowDeath,
            )?;

            Self::deposit_event(Event::<T>::GetTokensToRealis(from, to, value));
            Ok(())
        }

        #[pallet::weight(10000)]
        pub fn balance_pallet(origin: OriginFor<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(
                Self::bridge_masters().contains(&who),
                Error::<T>::NotBridgeMaster
            );
            let account_id = Self::account_id();
            let balance = <T as Config>::BridgeCurrency::free_balance(&account_id)
                .saturating_sub(<T as Config>::BridgeCurrency::minimum_balance());
            Self::deposit_event(Event::Balance(account_id, balance));
            Ok(())
        }

        #[pallet::weight(T::WeightInfoBridge::transfer_nft_to_bsc())]
        #[allow(irrefutable_let_patterns)]
        pub fn transfer_nft_to_bsc(
            origin: OriginFor<T>,
            to: H160,
            value: TokenId,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;

            let tokens = Nft::TokensList::<T>::get(who.clone());
            for token in tokens {
                if token.0.id == value {
                    ensure!(
                        token.1 != Status::OnSell,
                        Error::<T>::CannotTransferNftBecauseThisNftInMarketplace
                    );
                    ensure!(
                        token.1 != Status::InDelegation,
                        Error::<T>::CannotTransferNftBecauseThisNftOnAnotherUser
                    );
                    ensure!(
                        token.1 != Status::OnDelegateSell,
                        Error::<T>::CannotTransferNftBecauseThisNftOnAnotherUser
                    );
                };
            }

            let pallet_id = Self::account_id();

            Nft::Pallet::<T>::transfer_nft(&pallet_id, &who, value)?;

            Self::deposit_event(Event::<T>::TransferNftToBSC(who.clone(), to, value));
            Ok(())
        }

        #[pallet::weight(T::WeightInfoBridge::transfer_nft_to_realis())]
        pub fn transfer_nft_to_realis(
            origin: OriginFor<T>,
            from: H160,
            to: T::AccountId,
            token_id: TokenId,
        ) -> DispatchResult {
            let who = ensure_signed(origin.clone())?;
            ensure!(
                Self::bridge_masters().contains(&who),
                Error::<T>::NotBridgeMaster
            );

            let pallet_id = Self::account_id();

            Nft::Pallet::<T>::transfer_nft(&to, &pallet_id, token_id)?;

            Self::deposit_event(Event::<T>::TransferNftToRealis(from, to.clone(), token_id));
            Ok(())
        }

        #[pallet::weight(10000)]
        pub fn add_bridge_master(origin: OriginFor<T>, account: T::AccountId) -> DispatchResult {
            // Check is signed correct
            let who = ensure_signed(origin)?;
            // Check if account that signed operation have permission for this operation
            ensure!(
                Self::bridge_masters().contains(&who),
                Error::<T>::NotBridgeMaster
            );
            ensure!(
                !Self::bridge_masters().contains(&account),
                Error::<T>::BridgeMasterWasAddedEarly
            );

            BridgeMasters::<T>::mutate(|bridge_masters| {
                bridge_masters.push(account);
            });
            Ok(())
        }

        /// Remove bridge_master
        #[pallet::weight(10000)]
        pub fn remove_bridge_master(origin: OriginFor<T>, account: T::AccountId) -> DispatchResult {
            // Check is signed correct
            let who = ensure_signed(origin)?;
            // Check if account that signed operation have permission for this operation
            ensure!(
                Self::bridge_masters().contains(&who),
                Error::<T>::NotBridgeMaster
            );

            BridgeMasters::<T>::mutate(|bridge_masters| {
                let index = bridge_masters.iter().position(|token| *token == account);
                bridge_masters.remove(index.unwrap())
            });
            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        pub fn account_id() -> T::AccountId {
            <T as Config>::PalletId::get().into_account()
        }
    }
}
