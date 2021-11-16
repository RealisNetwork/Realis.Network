#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;

    use frame_support::inherent::Vec;

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
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    #[pallet::metadata(T::AccountId = "AccountId", TokenId = "T::TokenId")]
    pub enum Event<T: Config> {
        NftDelegated(T::AccountId, T::AccountId, TokenId, u64),
        EndNftDelegation(TokenId),
    }

    #[pallet::error]
    pub enum Error<T> {
        NonExistentNft,
        NotNftOwner,
        NftAlreadyInUse,
    }

    #[pallet::storage]
    #[pallet::getter(fn get_delegated_tokens)]
    pub type DelegatedTokens<T: Config> = StorageValue<_, Vec<(TokenId, u64)>, ValueQuery>;

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
        fn on_finalize(_n: BlockNumberFor<T>) {
            DelegatedTokens::<T>::mutate(|delegated_tokens|
                delegated_tokens
                    .into_iter()
                    .for_each(|(token_id, delegated_time_in_blocks)| {
                        if *delegated_time_in_blocks > 0_u64 {
                            *delegated_time_in_blocks -= 1;
                        } else {
                            Self::drop_delegate_nft(token_id.clone());
                        }
                    }));
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
            let origin = ensure_signed(origin)?;
            let owner = PalletNft::AccountForToken::<T>::get(token_id).ok_or(Error::<T>::NonExistentNft)?;

            ensure!(origin == owner, Error::<T>::NotNftOwner);

            Self::delegate_nft(owner, to, token_id, delegated_time)
        }
    }

    impl<T: Config> Pallet<T> {
        pub fn delegate_nft(
            from: T::AccountId,
            to: T::AccountId,
            token_id: TokenId,
            delegated_time_in_blocks: u64,
        ) -> DispatchResult {
            match PalletNft::Pallet::<T>::get_nft_status(token_id) {
                None => Err(Error::<T>::NonExistentNft)?,
                Some(Status::OnSell | Status::InDelegation) => Err(Error::<T>::NftAlreadyInUse)?,
                Some(Status::Free) => {}
            }

            // TODO maybe can be simplify using append instead of mutate
            DelegatedTokens::<T>::mutate(|delegated_tokens| delegated_tokens.push((token_id, delegated_time_in_blocks)));

            PalletNft::Pallet::<T>::set_nft_status(token_id, Status::InDelegation);

            Self::deposit_event(Event::NftDelegated(from, to, token_id, delegated_time_in_blocks));

            Ok(())
        }


    }
}
