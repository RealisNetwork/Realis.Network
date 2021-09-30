// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::{dispatch::DispatchResult, ensure, traits::Get};
use frame_system::ensure_signed;
pub use realis_primitives::{TokenId, TokenType, Status};
use sp_std::prelude::*;

mod benchmarking;
mod mock;
mod tests;

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_support::traits::{Currency, ExistenceRequirement};
    use frame_support::PalletId;
    use frame_system::pallet_prelude::*;
    use pallet_nft as Nft;
    use realis_primitives::Rarity;
    use realis_primitives::Rarity::Common;
    use realis_primitives::String;
    use realis_primitives::TokenType::Basic;
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
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    pub enum Event<T: Config> {
        ///Token was tranfered to BEP-20 on BSC
        TransferTokenToBSC(T::AccountId, H160, BalanceOf<T>, BalanceOf<T>),
        ///NFT was tranfered to BEP-721 on BSC
        TransferNftToBSC(T::AccountId, H160, TokenId, u8, Rarity),

        ///Token was tranfered to Realis.Network from BEP-20 on BSC
        TransferTokenToRealis(H160, T::AccountId, BalanceOf<T>),
        ///NFT was tranfered to Realis.Network from BEP-721 on BSC
        TransferNftToRealis(H160, T::AccountId, TokenId, u8, Rarity),
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
        CannotTransferNftBecauseThisNftOnAnotherUser

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
        #[pallet::weight(10000)]
        pub fn transfer_token_to_bsc(
            origin: OriginFor<T>,
            from: T::AccountId,
            to: H160,
            #[pallet::compact] value: T::Balance,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let balance = <T as Config>::BridgeCurrency::free_balance(&who);
            ensure!(balance > value, Error::<T>::InsufficientBalance);

            Self::deposit_event(Event::<T>::TransferTokenToBSC(
                from.clone(),
                to,
                value,
                balance,
            ));
            Ok(())
        }

        #[pallet::weight(10000)]
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

            Self::deposit_event(Event::<T>::TransferTokenToRealis(from, to, value));
            Ok(())
        }

        #[pallet::weight(90000000)]
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

        #[pallet::weight(10000)]
        #[allow(irrefutable_let_patterns)]
        pub fn transfer_nft_to_bsc(
            origin: OriginFor<T>,
            from: T::AccountId,
            dest: H160,
            token_id: TokenId,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            // Only owner can transfer token
            ensure!(who == from, Error::<T>::NotTokenOwner);
            let token = Nft::TokensList::<T>::get(from.clone()).unwrap();

            let tokens = Nft::TokensList::<T>::get(from.clone()).unwrap();
            for token in tokens {
                if token.0.id == token_id {
                    ensure!(
                        token.1 == Status::OnSell,
                        Error::<T>::CannotTransferNftBecauseThisNftInMarketplace
                    );
                    ensure!(
                        token.1 == Status::InDelegation,
                        Error::<T>::CannotTransferNftBecauseThisNftOnAnotherUser
                    );
                };
            }

            let mut value: u8 = 1;

            let mut rarity: Rarity = Common;

            for t in token {
                if t.0.id == token_id {
                    if let Basic(v, t, _) = t.0.token_type {
                        value = v;
                        rarity = t;
                    }
                }
            }

            Self::deposit_event(Event::<T>::TransferNftToBSC(
                from, dest, token_id, value, rarity,
            ));
            Ok(())
        }

        #[pallet::weight(10000)]
        pub fn transfer_nft_to_realis(
            origin: OriginFor<T>,
            from: H160,
            to: T::AccountId,
            token_id: TokenId,
            token_type: u8,
            rarity: Rarity,
            link: String,
        ) -> DispatchResult {
            let who = ensure_signed(origin.clone())?;
            ensure!(
                Self::bridge_masters().contains(&who),
                Error::<T>::NotBridgeMaster
            );

            Nft::Pallet::<T>::mint(origin, to.clone(), token_id, rarity, token_type, link)?;

            Self::deposit_event(Event::<T>::TransferNftToRealis(
                from, to, token_id, token_type, rarity,
            ));
            Ok(())
        }

        #[pallet::weight(10000)]
        pub fn transfer_token_to_bsc_success(
            origin: OriginFor<T>,
            from: T::AccountId,
            value: T::Balance,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(
                Self::bridge_masters().contains(&who),
                Error::<T>::NotBridgeMaster
            );
            let pallet_id = Self::account_id();
            <T as Config>::BridgeCurrency::transfer(
                &from,
                &pallet_id,
                value,
                ExistenceRequirement::KeepAlive,
            )?;
            Ok(())
        }

        #[pallet::weight(10000)]
        #[allow(path_statements)]
        pub fn transfer_token_to_bsc_error(origin: OriginFor<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(
                Self::bridge_masters().contains(&who),
                Error::<T>::NotBridgeMaster
            );
            Error::<T>::TokensWasntTransfered;
            Ok(())
        }

        #[pallet::weight(10000)]
        #[allow(unused_must_use)]
        pub fn transfer_nft_to_bsc_success(
            origin: OriginFor<T>,
            from: T::AccountId,
            token_id: TokenId,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(
                Self::bridge_masters().contains(&who),
                Error::<T>::NotBridgeMaster
            );
            Nft::Pallet::<T>::burn_nft(token_id, &from);
            Ok(())
        }

        #[pallet::weight(10000)]
        #[allow(path_statements)]
        pub fn transfer_nft_to_bsc_error(origin: OriginFor<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(
                Self::bridge_masters().contains(&who),
                Error::<T>::NotBridgeMaster
            );
            Error::<T>::NFTWasntTransfered;
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
