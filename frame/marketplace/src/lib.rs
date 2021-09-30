#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::dispatch;
pub use pallet::*;
use sp_std::prelude::*;
#[allow(unused_imports)]
use sp_std::vec;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use pallet_nft as Nft;
    use realis_primitives::*;

    use super::*;
    use frame_support::traits::{Currency, ExistenceRequirement};
    use node_primitives::Balance;
    use sp_runtime::ArithmeticError;

    #[pallet::pallet]
    #[pallet::generate_store(pub (super) trait Store)]
    pub struct Pallet<T>(PhantomData<T>);

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config + Nft::Config + realis_game_api::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        type Currency: Currency<Self::AccountId, Balance = Balance>;
    }

    // Pallets use events to inform users when important changes are made.
    // https://substrate.dev/docs/en/knowledgebase/runtime/events
    #[pallet::event]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    #[pallet::metadata(
        T::AccountId = "AccountId",
        TokenBalance = "Balance",
        RealisTokenId = "T::RealisTokenId"
    )]
    pub enum Event<T: Config> {
        /// This TokenId on sale in Marketplace!
        NftForSale(TokenId, Balance, Vec<(Token, Status)>),
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
        CannotChangePriceNft
    }

    #[pallet::storage]
    #[pallet::getter(fn nft_for_sale_in_account)]
    pub(super) type NFTForSaleInAccount<T: Config> =
        StorageMap<_, Blake2_256, <T as frame_system::Config>::AccountId, Vec<(TokenId, Balance)>>;

    #[pallet::storage]
    pub(super) type AllNFTForSale<T: Config> = StorageValue<_, Vec<(TokenId, Balance)>>;

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    #[pallet::call]
    impl<T: Config> Pallet<T> {
        #[pallet::weight(90_000_000)]
        pub fn sell_nft(origin: OriginFor<T>, token_id: TokenId, price: Balance) -> DispatchResult {
            let who = ensure_signed(origin)?;
            let owner = pallet_nft::AccountForToken::<T>::get(token_id).unwrap();
            let tokens = Nft::TokensList::<T>::get(who.clone()).unwrap();
            for token in tokens {
                if token.0.id == token_id {
                    ensure!(
                        token.1 == Status::Free,
                        Error::<T>::CannotSellAgainNft
                    );
                };
            }
            let old_token = Self::sell(owner, token_id, price).unwrap();
            // Call mint event
            Self::deposit_event(Event::NftForSale(token_id, price, old_token));
            Ok(())
        }

        #[pallet::weight(90_000_000)]
        pub fn buy_nft(origin: OriginFor<T>, token_id: TokenId) -> DispatchResult {
            let who = ensure_signed(origin)?;
            // if token_in_storage[0].1 == Status::InDelegation || token_in_storage[0].1 == Status::OnSell {
            //     pallet::DispatchError::Other("CannotForSaleThisNft");
            // }
            Self::buy(who.clone(), token_id).unwrap();

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
            let tokens = Nft::TokensList::<T>::get(who.clone()).unwrap();
            let owner = pallet_nft::AccountForToken::<T>::get(token_id).unwrap();
            for token in tokens {
                if token.0.id == token_id {
                    ensure!(
                        who == owner,
                        Error::<T>::CannotChangePriceNft
                    );
                };
            }
            Self::change_price(who.clone(), token_id, price).unwrap();

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
            // if token_in_storage[0].1 == Status::InDelegation || token_in_storage[0].1 == Status::OnSell {
            //     pallet::DispatchError::Other("CannotForSaleThisNft");
            // }
            Self::remove(who.clone(), token_id).unwrap();

            // Call mint event
            Self::deposit_event(Event::RemoveFromMarketplaceNft(token_id));
            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        fn sell(
            seller: <T as frame_system::Config>::AccountId,
            token_id: TokenId,
            price: Balance,
        ) -> dispatch::result::Result<Vec<(Token, Status)>, dispatch::DispatchError> {
            NFTForSaleInAccount::<T>::mutate(seller.clone(), |tokens| {
                tokens.get_or_insert(Vec::default()).push((token_id, price));
            });

            AllNFTForSale::<T>::mutate(|tokens| {
                tokens.get_or_insert(Vec::default()).push((token_id, price));
            });

            let mut old_token = vec![];

            Nft::TokensList::<T>::mutate(&seller, |tuple_tokens| {
                tuple_tokens
                    .as_mut()
                    .unwrap()
                    .iter()
                    .for_each(|tuple_tokens| {
                        if tuple_tokens.0.id == token_id {
                            old_token.push((tuple_tokens.0.clone(), Status::OnSell));
                        }
                    });
            });

            Nft::TokensList::<T>::mutate(&seller, |tokens| {
                let tokens_mut = tokens.as_mut().unwrap();
                let index = tokens_mut.iter().position(|token| token.0.id == token_id);
                tokens_mut.remove(index.unwrap());
            });

            Nft::TokensList::<T>::mutate(&seller, |tokens| {
                tokens
                    .get_or_insert(Vec::default())
                    .push(old_token[0].clone());
            });
            Ok(old_token)
        }

        fn buy(
            buyer: <T as frame_system::Config>::AccountId,
            token_id: TokenId,
        ) -> dispatch::result::Result<(), dispatch::DispatchError> {
            let owner = pallet_nft::AccountForToken::<T>::get(token_id).unwrap();

            let mut balance = vec![];

            NFTForSaleInAccount::<T>::mutate(&owner, |tuple_tokens| {
                tuple_tokens
                    .as_mut()
                    .unwrap()
                    .iter()
                    .for_each(|tuple_tokens| {
                        if tuple_tokens.0 == token_id {
                            balance.push(tuple_tokens.1);
                        }
                    });
            });

            let five_percent = balance[0] / 100 * 5;

            let amount = balance[0] / 100 * 95;

            <T as pallet::Config>::Currency::transfer(&buyer, &owner, amount, ExistenceRequirement::KeepAlive)?;

            let pallet_id: T::AccountId = realis_game_api::Pallet::<T>::account_id();

            <T as pallet::Config>::Currency::transfer(&buyer, &pallet_id, five_percent, ExistenceRequirement::KeepAlive)?;

            NFTForSaleInAccount::<T>::mutate(&owner, |tokens| {
                let tokens_mut = tokens.as_mut().unwrap();
                let index = tokens_mut.iter().position(|token| token.0 == token_id);
                tokens_mut.remove(index.unwrap())
            });

            AllNFTForSale::<T>::mutate(|tokens| {
                let tokens_mut = tokens.as_mut().unwrap();
                let index = tokens_mut.iter().position(|token| token.0 == token_id);
                tokens_mut.remove(index.unwrap())
            });

            let mut old_token = vec![];

            Nft::TokensList::<T>::mutate(&owner, |tuple_tokens| {
                tuple_tokens
                    .as_mut()
                    .unwrap()
                    .iter()
                    .for_each(|tuple_tokens| {
                        if tuple_tokens.0.id == token_id {
                            old_token.push((tuple_tokens.0.clone(), Status::Free));
                        }
                    });
            });

            Nft::TokensList::<T>::mutate(&owner, |tokens| {
                let tokens_mut = tokens.as_mut().unwrap();
                let index = tokens_mut.iter().position(|token| token.0.id == token_id);
                tokens_mut.remove(index.unwrap())
            });

            Nft::AccountForToken::<T>::insert(token_id, buyer.clone());
            Self::dec_total_for_account(&owner)?;
            Self::inc_total_for_account(&buyer.clone())?;

            Nft::TokensList::<T>::mutate(buyer, |tokens| {
                tokens
                    .get_or_insert(Vec::default())
                    .push(old_token[0].clone());
            });

            Ok(())
        }

        fn change_price(
            owner: <T as frame_system::Config>::AccountId,
            token_id: TokenId,
            new_price: Balance,
        ) -> dispatch::result::Result<(), dispatch::DispatchError> {
            let mut old_token = vec![];

            NFTForSaleInAccount::<T>::mutate(&owner, |tokens| {
                tokens.as_mut().unwrap().iter().for_each(|tuple_tokens| {
                    if tuple_tokens.0 == token_id {
                        old_token.push((tuple_tokens.0.clone(), new_price));
                    }
                });
            });
            AllNFTForSale::<T>::mutate(|tokens| {
                let tokens_mut = tokens.as_mut().unwrap();
                let index = tokens_mut.iter().position(|token| token.0 == token_id);
                tokens_mut.remove(index.unwrap());
            });

            AllNFTForSale::<T>::mutate(|tokens| {
                tokens
                    .get_or_insert(Vec::default())
                    .push(old_token[0].clone());
            });

            NFTForSaleInAccount::<T>::mutate(&owner, |tokens| {
                let tokens_mut = tokens.as_mut().unwrap();
                let index = tokens_mut.iter().position(|token| token.0 == token_id);
                tokens_mut.remove(index.unwrap());
            });

            NFTForSaleInAccount::<T>::mutate(&owner, |tokens| {
                tokens
                    .get_or_insert(Vec::default())
                    .push(old_token[0].clone());
            });
            Ok(())
        }

        fn remove(
            owner: <T as frame_system::Config>::AccountId,
            token_id: TokenId,
        ) -> dispatch::result::Result<(), dispatch::DispatchError> {
            NFTForSaleInAccount::<T>::mutate(&owner, |tokens| {
                let tokens_mut = tokens.as_mut().unwrap();
                let index = tokens_mut.iter().position(|token| token.0 == token_id);
                tokens_mut.remove(index.unwrap())
            });

            AllNFTForSale::<T>::mutate(|tokens| {
                let tokens_mut = tokens.as_mut().unwrap();
                let index = tokens_mut.iter().position(|token| token.0 == token_id);
                tokens_mut.remove(index.unwrap())
            });

            let mut old_token = vec![];

            Nft::TokensList::<T>::mutate(&owner, |tokens| {
                tokens.as_mut().unwrap().iter().for_each(|tuple_tokens| {
                    if tuple_tokens.0.id == token_id {
                        old_token.push((tuple_tokens.0.clone(), Status::Free));
                    }
                });
            });

            Nft::TokensList::<T>::mutate(&owner, |tokens| {
                let tokens_mut = tokens.as_mut().unwrap();
                let index = tokens_mut.iter().position(|token| token.0.id == token_id);
                tokens_mut.remove(index.unwrap())
            });

            Nft::TokensList::<T>::mutate(owner, |tokens| {
                tokens
                    .get_or_insert(Vec::default())
                    .push(old_token[0].clone());
            });

            Ok(())
        }

        fn inc_total_for_account(account: &T::AccountId) -> Result<(), ArithmeticError> {
            Nft::TotalForAccount::<T>::try_mutate(account, |cnt| {
                cnt.checked_add(1)
                    .map_or(Err(ArithmeticError::Overflow), |new_cnt| {
                        *cnt = new_cnt;
                        Ok(())
                    })
            })
        }

        fn dec_total_for_account(account: &T::AccountId) -> Result<(), ArithmeticError> {
            Nft::TotalForAccount::<T>::try_mutate(account, |cnt| {
                cnt.checked_sub(1)
                    .map_or(Err(ArithmeticError::Overflow), |new_cnt| {
                        *cnt = new_cnt;
                        Ok(())
                    })
            })
        }
    }
}
