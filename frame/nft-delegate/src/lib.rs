#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;

    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use frame_support::inherent::Vec;
    use frame_support::traits::{ExistenceRequirement, Currency};
    use node_primitives::Balance;

    use realis_primitives::{Status, TokenId};
    use pallet_nft as PalletNft;

    #[pallet::pallet]
    #[pallet::generate_store(pub (super) trait Store)]
    pub struct Pallet<T>(PhantomData<T>);

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config + PalletNft::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        type Currency: Currency<Self::AccountId, Balance = Balance>;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    #[pallet::metadata(T::AccountId = "AccountId", TokenId = "T::TokenId", Balance = "Balance")]
    pub enum Event<T: Config> {
        NftDelegated(T::AccountId, T::AccountId, TokenId, u64),
        EndNftDelegation(TokenId),
        NftSold(T::AccountId, T::AccountId, TokenId, u64, Balance)
    }

    #[pallet::error]
    pub enum Error<T> {
        NonExistentNft,
        NotNftOwner,
        NftAlreadyInUse,
        DelegationTimeTooLow,
        CannotBuyOwnNft,
    }

    #[pallet::storage]
    pub type TokensForAccount<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, Vec<TokenId>>;

    #[pallet::storage]
    pub type DelegatedTokens<T: Config> = StorageValue<_, Vec<(T::AccountId, TokenId, u64)>, ValueQuery>;

    #[pallet::storage]
    pub type DelegateForSale<T: Config> = StorageValue<_, Vec<(TokenId, u64, Balance)>, ValueQuery>;

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_finalize(_n: BlockNumberFor<T>) {
            Self::clean();
            DelegatedTokens::<T>::mutate(|delegated_tokens| {
                // Decrement time in blocks
                delegated_tokens
                    .into_iter()
                    .for_each(|(_, _, delegated_time_in_blocks)| {
                        if *delegated_time_in_blocks > 0_u64 {
                            *delegated_time_in_blocks -= 1;
                        }
                    })
            });
        }
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(90_000_000)]
        pub fn delegate(
            origin: OriginFor<T>,
            to: T::AccountId,
            token_id: TokenId,
            delegated_time: u64,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let owner = PalletNft::AccountForToken::<T>::get(token_id)
                .ok_or(Error::<T>::NonExistentNft)?;
            ensure!(who == owner, Error::<T>::NotNftOwner);
            Self::check_time(delegated_time)?;

            Self::can_delegate_nft(token_id)?;
            Self::delegate_nft(owner, to, token_id, delegated_time);

            Ok(())
        }

        #[pallet::weight(90_000_000)]
        pub fn sell_delegate(
            origin: OriginFor<T>,
            token_id: TokenId,
            delegated_time: u64,
            price: Balance
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let owner = PalletNft::AccountForToken::<T>::get(token_id)
                .ok_or(Error::<T>::NonExistentNft)?;
            ensure!(who == owner, Error::<T>::NotNftOwner);
            ensure!(delegated_time != 0, Error::<T>::DelegationTimeTooLow);

            Self::can_delegate_nft(token_id)?;
            Self::sale_delegate_nft(owner, token_id, delegated_time, price);

            Ok(())
        }

        #[pallet::weight(90_000_000)]
        pub fn buy_delegate(
            origin: OriginFor<T>,
            token_id: TokenId
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let owner = PalletNft::AccountForToken::<T>::get(token_id)
                .ok_or(Error::<T>::NonExistentNft)?;
            ensure!(who != owner, Error::<T>::CannotBuyOwnNft);

            Self::buy_delegate_nft(who, token_id)
        }

        #[pallet::weight(90_000_000)]
        pub fn change_price_delegate(
            origin: OriginFor<T>,
            token_id: TokenId,
            new_price: Balance,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let owner = PalletNft::AccountForToken::<T>::get(token_id)
                .ok_or(Error::<T>::NonExistentNft)?;
            ensure!(who == owner, Error::<T>::NotNftOwner);

            Self::change_price_delegate_nft(token_id, new_price);

            Ok(())
        }

        #[pallet::weight(90_000_000)]
        pub fn change_delegate_time_on_sale(
            origin: OriginFor<T>,
            token_id: TokenId,
            new_time: u64,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let owner = PalletNft::AccountForToken::<T>::get(token_id)
                .ok_or(Error::<T>::NonExistentNft)?;
            ensure!(who == owner, Error::<T>::NotNftOwner);
            Self::check_time(new_time)?;

            Self::change_delegate_nft_time_on_sale(token_id, new_time);

            Ok(())
        }

        #[pallet::weight(90_000_000)]
        pub fn remove_from_sell(
            origin: OriginFor<T>,
            token_id: TokenId,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let owner = PalletNft::AccountForToken::<T>::get(token_id)
                .ok_or(Error::<T>::NonExistentNft)?;
            ensure!(who == owner, Error::<T>::NotNftOwner);

            Self::remove_nft_from_sell(token_id);

            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        pub fn delegate_nft(
            from: T::AccountId,
            to: T::AccountId,
            token_id: TokenId,
            delegated_time_in_blocks: u64,
        ){
            DelegatedTokens::<T>::append((to.clone(), token_id, delegated_time_in_blocks));

            TokensForAccount::<T>::mutate(to.clone(), |delegated_tokens| {
                delegated_tokens.get_or_insert(Vec::new())
                    .push(token_id)
            });

            PalletNft::Pallet::<T>::set_nft_status(token_id, Status::InDelegation);

            Self::deposit_event(Event::NftDelegated(from, to, token_id, delegated_time_in_blocks));
        }

        pub fn clean() {
            DelegatedTokens::<T>::get()
                .into_iter()
                .for_each(|(account_id, token_id, delegated_time_in_blocks)| {
                    if delegated_time_in_blocks == 0_u64 {
                        Self::drop_delegated_nft(account_id, token_id.clone())
                    };
                });
        }

        pub fn drop_delegated_nft(
            account_id: T::AccountId,
            token_id: TokenId,
        ) {
            TokensForAccount::<T>::mutate(account_id, |delegated_tokens| {
                delegated_tokens
                    .as_mut()
                    .map(|delegated_tokens| delegated_tokens.retain(|id| *id != token_id))
            });

            DelegatedTokens::<T>::mutate(|delegated_tokens| {
                delegated_tokens.retain(|(_, id, _)| *id != token_id);
            });

            PalletNft::Pallet::<T>::set_nft_status(token_id, Status::Free);

            Self::deposit_event(Event::EndNftDelegation(token_id));
        }

        pub fn sale_delegate_nft(
            _who: T::AccountId,
            token_id: TokenId,
            delegated_time: u64,
            price: Balance,
        ) {
            DelegateForSale::<T>::append((token_id, delegated_time, price));

            PalletNft::Pallet::<T>::set_nft_status(token_id, Status::OnDelegateSell);
        }

        pub fn buy_delegate_nft(
            buyer: T::AccountId,
            token_id: TokenId,
        ) -> DispatchResult {
            let owner = PalletNft::AccountForToken::<T>::get(token_id)
                .ok_or(Error::<T>::NonExistentNft)?;

            Self::can_buy_nft(token_id)?;

            let (_, delegated_time_in_blocks, price) = DelegateForSale::<T>::get()
                .into_iter()
                .find(|(id, _, _)| *id == token_id)
                .ok_or(Error::<T>::NonExistentNft)?;

            <T as pallet::Config>::Currency::transfer(
                &buyer,
                &owner,
                price,
                ExistenceRequirement::KeepAlive,
            )?;

            DelegateForSale::<T>::mutate(|delegated_tokens| {
                delegated_tokens
                    .retain(|(id, _, _)| *id != token_id)
            });

            Self::delegate_nft(owner, buyer, token_id, delegated_time_in_blocks);

            Ok(())
        }

        pub fn change_price_delegate_nft(
            token_id: TokenId,
            new_price: Balance,
        ) {
            DelegateForSale::<T>::mutate(|delegated_tokens| {
                delegated_tokens
                    .into_iter()
                    .find(|(id, _, _)| *id == token_id)
                    .map(|(_, _, price)| *price = new_price)
            });
        }

        pub fn change_delegate_nft_time_on_sale(
            token_id: TokenId,
            new_time: u64,
        ) {
            DelegateForSale::<T>::mutate(|delegated_tokens| {
                delegated_tokens
                    .into_iter()
                    .find(|(id, _, _)| *id == token_id)
                    .map(|(_, time, _)| *time = new_time)
            });
        }

        pub fn remove_nft_from_sell(token_id: TokenId) {
            DelegateForSale::<T>::mutate(|delegated_tokens| {
                delegated_tokens
                    .retain(|(id, _, _)| *id != token_id)
            });

            PalletNft::Pallet::<T>::set_nft_status(token_id, Status::Free);
        }

        pub fn can_delegate_nft(token_id: TokenId) -> DispatchResult {
            match PalletNft::Pallet::<T>::get_nft_status(token_id) {
                None => Err(Error::<T>::NonExistentNft)?,
                Some(Status::OnSell | Status::InDelegation | Status::OnDelegateSell) => Err(Error::<T>::NftAlreadyInUse)?,
                Some(Status::Free) => {}
            }

            Ok(())
        }

        pub fn can_buy_nft(token_id: TokenId) -> DispatchResult {
            match PalletNft::Pallet::<T>::get_nft_status(token_id) {
                None => Err(Error::<T>::NonExistentNft)?,
                Some(Status::OnSell | Status::InDelegation) => Err(Error::<T>::NftAlreadyInUse)?,
                Some(Status::Free | Status::OnDelegateSell) => {}
            }

            Ok(())
        }

        pub fn check_time(time: u64) -> DispatchResult {
            ensure!(time != 0, Error::<T>::DelegationTimeTooLow);
            Ok(())
        }
    }
}
