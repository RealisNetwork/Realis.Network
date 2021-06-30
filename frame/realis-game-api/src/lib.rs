#![cfg_attr(not(feature = "std"), no_std)]

// 1. Imports and Dependencies
pub use frame_support::traits::Currency;

pub use pallet::*;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

mod benchmarking;
pub mod weights;

pub use weights::WeightInfoOf;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_support::PalletId;
    use frame_support::traits::{Currency, ExistenceRequirement, WithdrawReasons};
    use frame_support::traits::Imbalance;
    use frame_system::pallet_prelude::*;
    use sp_runtime::traits::{AccountIdConversion, Saturating};
    use super::*;

    use pallet_nft as NFT;

    type BalanceOf<T> = <<T as Config>::ApiCurrency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::config]
    pub trait Config: frame_system::Config + pallet_nft::Config + pallet_staking::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        type PalletId: Get<PalletId>;

        type ApiCurrency: Currency<Self::AccountId, Balance = Self::Balance>;

        type StakingPoolId: From<<Self as pallet_staking::Config>::PalletId>;

        type WeightInfoOf: WeightInfoOf;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        /// Event documentation should end with an array that provides descriptive names for event
        /// parameters. [something, who]
        TokenMinted,
        TokenTransferred,
        TokenBurned,
        FundsTransferred,
        Balance(T::AccountId, BalanceOf<T>),
    }

    #[pallet::error]
    pub enum Error<T> {
        /// Error names should be descriptive.
        NoneValue,
        ///
        TokenExist,
        ///
        NotTokenOwner,
        ///
        NonExistentToken,
        ///
        NotNftMaster,
    }

    // #[pallet::storage]
    // pub(crate) type NftMasters<T: Config> = StorageValue<_, T::AccountId, ValueQuery>;

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub nft_masters: Vec<T::AccountId>,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                nft_masters: Default::default(),
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            NFT::NftMasters::<T>::put(&self.nft_masters);
        }
    }

    #[cfg(feature = "std")]
    impl<T: Config> GenesisConfig<T> {
        /// Direct implementation of `GenesisBuild::build_storage`.
        ///
        /// Kept in order not to break dependency.
        pub fn build_storage(&self) -> Result<sp_runtime::Storage, String> {
            <Self as GenesisBuild<T>>::build_storage(self)
        }
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    // 7. Extrinsics
    // Functions that are callable from outside the runtime.
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(T::WeightInfoOf::mint_basic_nft())]
        pub fn mint_basic_nft(
            origin: OriginFor<T>,
            target_account: T::AccountId,
            token_id: pallet_nft::TokenId,
            type_token: pallet_nft::Types,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let nft_master = NFT::NftMasters::<T>::get();
            ensure!(nft_master.contains(&who), Error::<T>::NotNftMaster);

            ensure!(
                !NFT::AccountForToken::<T>::contains_key(token_id),
                Error::<T>::TokenExist
            );

            NFT::Pallet::<T>::mint_basic_nft(&target_account, token_id, type_token)?;
            Self::deposit_event(Event::<T>::TokenMinted);
            Ok(())
        }

        #[pallet::weight(T::WeightInfoOf::burn_basic_nft())]
        pub fn burn_basic_nft(
            origin: OriginFor<T>,
            token_id: pallet_nft::TokenId,
        ) -> DispatchResult {
            let who = ensure_signed(origin.clone())?;
            let nft_master = NFT::NftMasters::<T>::get();
            ensure!(nft_master.contains(&who), Error::<T>::NotNftMaster);
            NFT::Pallet::<T>::burn_basic(origin, token_id)?;
            Self::deposit_event(Event::<T>::TokenBurned);
            Ok(())
        }

        #[pallet::weight(T::WeightInfoOf::transfer_basic_nft())]
        pub fn transfer_basic_nft(
            origin: OriginFor<T>,
            dest_account: T::AccountId,
            token_id: pallet_nft::TokenId,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let nft_master = NFT::NftMasters::<T>::get();
            ensure!(nft_master.contains(&who), Error::<T>::NotNftMaster);

            NFT::Pallet::<T>::transfer_basic_nft(token_id, None, &dest_account)?;
            Self::deposit_event(Event::<T>::TokenTransferred);
            Ok(())
        }

        #[pallet::weight(T::WeightInfoOf::transfer_from_pallet())]
        pub fn transfer_from_pallet(
            origin: OriginFor<T>,
            dest: T::AccountId,
            #[pallet::compact] value: T::Balance,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let nft_master = NFT::NftMasters::<T>::get();
            ensure!(nft_master.contains(&who), Error::<T>::NotNftMaster);
            let pallet_id = Self::account_id();
            <T as Config>::ApiCurrency::transfer(
                &pallet_id,
                &dest,
                value,
                ExistenceRequirement::KeepAlive,
            )?;
            Self::deposit_event(Event::<T>::FundsTransferred);
            Ok(())
        }

        #[pallet::weight(T::WeightInfoOf::transfer_to_pallet())]
        pub fn transfer_to_pallet(
            origin: OriginFor<T>,
            from: T::AccountId,
            #[pallet::compact] value: T::Balance,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let nft_master = NFT::NftMasters::<T>::get();
            ensure!(nft_master.contains(&who), Error::<T>::NotNftMaster);
            let pallet_id = Self::account_id();
            <T as Config>::ApiCurrency::transfer(
                &from,
                &pallet_id,
                value,
                ExistenceRequirement::KeepAlive,
            )?;
            Self::deposit_event(Event::<T>::FundsTransferred);
            Ok(())
        }

        #[pallet::weight(T::WeightInfoOf::transfer_from_ptp())]
        pub fn transfer_from_ptp(
            origin: OriginFor<T>,
            from: T::AccountId,
            to: T::AccountId,
            #[pallet::compact] value: T::Balance,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let nft_master = NFT::NftMasters::<T>::get();
            ensure!(nft_master.contains(&who), Error::<T>::NotNftMaster);
            <T as Config>::ApiCurrency::transfer(&from, &to, value, ExistenceRequirement::KeepAlive)?;
            Self::deposit_event(Event::<T>::FundsTransferred);
            Ok(())
        }

        #[pallet::weight(T::WeightInfoOf::spend_in_game())]
        pub fn spend_in_game(
            origin: OriginFor<T>,
            from: T::AccountId,
            #[pallet::compact] amount: T::Balance,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let nft_master = NFT::NftMasters::<T>::get();
            ensure!(nft_master.contains(&who), Error::<T>::NotNftMaster);
            let imbalance = <T as Config>::ApiCurrency::withdraw(
                &from,
                amount,
                WithdrawReasons::all(),
                ExistenceRequirement::KeepAlive,
            )?;
            let pallet_id_staking = Self::account_id_staking();
            let pallet_id_game_api = Self::account_id();
            // for fees, 80% to treasury, 20% to author
            let (to_game_api, to_staking) = imbalance.ration(80, 20);
            <T as Config>::ApiCurrency::resolve_creating(&pallet_id_game_api, to_game_api);
            <T as Config>::ApiCurrency::resolve_creating(&pallet_id_staking, to_staking);
            Ok(())
        }

        #[pallet::weight(90_000_000)]
        pub fn balance_pallet(origin: OriginFor<T>) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let nft_master = NFT::NftMasters::<T>::get();
            ensure!(nft_master.contains(&who), Error::<T>::NotNftMaster);
            let account_id = Self::account_id();
            let balance = <T as Config>::ApiCurrency::free_balance(&account_id)
                .saturating_sub(<T as Config>::ApiCurrency::minimum_balance());
            Self::deposit_event(Event::Balance(account_id, balance));
            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        pub fn account_id() -> T::AccountId {
            <T as Config>::PalletId::get().into_account()
        }

        pub fn account_id_staking() -> T::AccountId {
            <T as pallet_staking::Config>::PalletId::get().into_account()
        }
    }
}
