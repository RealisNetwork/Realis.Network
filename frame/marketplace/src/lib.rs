#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::dispatch;
pub use pallet::*;
use sp_std::prelude::*;

#[frame_support::pallet]
pub mod pallet {
    use super::*;
    use frame_support::pallet_prelude::*;
    use frame_support::traits::{Currency, ExistenceRequirement};
    use frame_system::pallet_prelude::*;
    use node_primitives::Balance;
    use pallet_nft as Nft;
    use realis_primitives::*;
    use realis_primitives::constants::COMMISSION;
    use frame_support::sp_runtime::traits::AccountIdConversion;

    #[pallet::pallet]
    #[pallet::generate_store(pub (super) trait Store)]
    pub struct Pallet<T>(PhantomData<T>);

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config + Nft::Config + pallet_staking::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        type Currency: Currency<Self::AccountId, Balance = Balance>;
    }

    // Pallets use events to inform users when important changes are made.
    // https://substrate.dev/docs/en/knowledgebase/runtime/events
    #[pallet::event]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    #[pallet::metadata(T::AccountId = "AccountId", TokenBalance = "Balance")]
    pub enum Event<T: Config> {
        /// This TokenId on sale in Marketplace!
        NftForSale(TokenId, Balance, TokenId),
        /// This TokenId was buyed in Marketplace!
        NftBuyed(<T as frame_system::Config>::AccountId, TokenId),
        /// This TokenId have a new price
        ChangePriceNft(TokenId, Balance),
        /// User remove NFT from Marketplace
        RemoveFromMarketplaceNft(TokenId),
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        CannotForSaleThisNft,
        CannotSellAgainNft,
        CannotChangePriceNft,
        NonExistentToken,
        NotTokenOwner,
    }

    #[pallet::storage]
    #[pallet::getter(fn nft_for_sale_in_account)]
    pub(super) type NFTForSaleInAccount<T: Config> = StorageMap<
        _,
        Blake2_256,
        <T as frame_system::Config>::AccountId,
        Vec<(TokenId, Rarity, Balance)>,
    >;

    #[pallet::storage]
    pub(super) type AllNFTForSale<T: Config> = StorageValue<_, Vec<(TokenId, Rarity, Balance)>, ValueQuery>;

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(90_000_000)]
        pub fn sell_nft(origin: OriginFor<T>, token_id: TokenId, price: Balance) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let owner = pallet_nft::AccountForToken::<T>::get(token_id)
                .ok_or(Error::<T>::NonExistentToken)?;
            ensure!(who == owner, Error::<T>::NotTokenOwner);
            let (token, status) = Nft::TokensList::<T>::get(who.clone())
                .into_iter()
                .find(|(token, _)| token.id == token_id)
                .ok_or(Error::<T>::NonExistentToken)?;

            ensure!(status == Status::Free, Error::<T>::CannotSellAgainNft);
            let TokenType::Basic(rarity, _, _, _) = token.token_type;
            Self::sell(owner.clone(), token_id, rarity, price);
            // Call sell event
            Self::deposit_event(Event::NftForSale(token_id, price, token_id));
            Ok(())
        }

        #[pallet::weight(90_000_000)]
        pub fn buy_nft(origin: OriginFor<T>, token_id: TokenId) -> DispatchResult {
            let who = ensure_signed(origin)?;
            // if token_in_storage[0].1 == Status::InDelegation || token_in_storage[0].1 == Status::OnSell {
            //     pallet::DispatchError::Other("CannotForSaleThisNft");
            // }
            Self::buy(who.clone(), token_id)?;

            // Call mint event
            Self::deposit_event(Event::NftBuyed(who, token_id));
            Ok(())
        }

        #[pallet::weight(90_000_000)]
        pub fn change_price_nft(
            origin: OriginFor<T>,
            token_id: TokenId,
            price: Balance,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let owner = pallet_nft::AccountForToken::<T>::get(token_id)
                .ok_or(Error::<T>::NonExistentToken)?;
            ensure!(who == owner, Error::<T>::CannotChangePriceNft);

            Self::change_price(who.clone(), token_id, price);

            // Call mint event
            Self::deposit_event(Event::ChangePriceNft(token_id, price));
            Ok(())
        }

        #[pallet::weight(90_000_000)]
        pub fn remove_from_marketplace_nft(
            origin: OriginFor<T>,
            token_id: TokenId,
        ) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let owner = pallet_nft::AccountForToken::<T>::get(token_id)
                .ok_or(Error::<T>::NonExistentToken)?;
            ensure!(who == owner, Error::<T>::NotTokenOwner);
            // if token_in_storage[0].1 == Status::InDelegation || token_in_storage[0].1 == Status::OnSell {
            //     pallet::DispatchError::Other("CannotForSaleThisNft");
            // }
            Self::remove(who.clone(), token_id);

            // Call mint event
            Self::deposit_event(Event::RemoveFromMarketplaceNft(token_id));
            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        pub fn sell(
            seller: <T as frame_system::Config>::AccountId,
            token_id: TokenId,
            rarity: Rarity,
            price: Balance,
        ) {
            NFTForSaleInAccount::<T>::mutate(seller.clone(), |tokens| {
                tokens
                    .get_or_insert(Vec::default())
                    .push((token_id, rarity, price.clone()));
            });

            AllNFTForSale::<T>::mutate(|tokens| {
                tokens.push((token_id, rarity, price));
            });

            Nft::Pallet::<T>::set_nft_status(token_id, Status::OnSell);
        }

        pub fn buy(
            buyer: <T as frame_system::Config>::AccountId,
            token_id: TokenId,
        ) -> dispatch::result::Result<(), dispatch::DispatchError> {
            let owner = pallet_nft::AccountForToken::<T>::get(token_id).unwrap();

            let (_, _, price) = NFTForSaleInAccount::<T>::get(&owner)
                .unwrap()
                .into_iter()
                .find(|(id, _, _)| *id == token_id)
                .ok_or(Error::<T>::NonExistentToken)?;

            let to_blockchain = price * COMMISSION / 100;
            let to_seller = price - to_blockchain;

            let staking = Self::account_id_staking();
            <T as pallet::Config>::Currency::transfer(
                &buyer,
                &staking,
                to_blockchain,
                ExistenceRequirement::KeepAlive,
            )?;

            <T as pallet::Config>::Currency::transfer(
                &buyer,
                &owner,
                to_seller,
                ExistenceRequirement::KeepAlive,
            )?;

            NFTForSaleInAccount::<T>::mutate(&owner, |tokens| {
                tokens.as_mut().unwrap().retain(|(id, _, _)| *id == token_id)
            });

            AllNFTForSale::<T>::mutate(|tokens| {
                tokens.retain(|(id, _, _)| *id == token_id)
            });

            let (token, _) = Nft::Pallet::<T>::pop(token_id);

            Nft::AccountForToken::<T>::insert(token_id, buyer.clone());
            Nft::Pallet::<T>::inc_total_for_account(&buyer.clone())?;
            Nft::TokensList::<T>::append(buyer, (token, Status::Free));

            Ok(())
        }

        pub fn change_price(
            owner: <T as frame_system::Config>::AccountId,
            token_id: TokenId,
            new_price: Balance,
        ) {
            NFTForSaleInAccount::<T>::mutate(&owner, |tokens| {
                tokens.as_mut().unwrap().into_iter().find(|(id, _, _)| *id == token_id)
                    .map(|(_, _, price)| *price = new_price.clone());
            });
            AllNFTForSale::<T>::mutate(|tokens| {
                tokens.into_iter().find(|(id, _, _)| *id == token_id)
                    .map(|(_, _, price)| *price = new_price);
            });
        }

        pub fn remove(
            owner: <T as frame_system::Config>::AccountId,
            token_id: TokenId,
        ) {
            NFTForSaleInAccount::<T>::mutate(&owner, |tokens| {
                tokens.as_mut().unwrap().retain(|(id, _, _)| *id != token_id);
            });

            AllNFTForSale::<T>::mutate(|tokens| {
                tokens.retain(|(id, _, _)| *id != token_id);
            });


            Nft::Pallet::<T>::set_nft_status(token_id, Status::Free);
        }

        pub fn account_id_staking() -> T::AccountId {
            <T as pallet_staking::Config>::PalletId::get().into_account()
        }
    }
}
