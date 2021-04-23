#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame

use frame_support::{decl_module, decl_storage, decl_event, decl_error, ensure, dispatch, traits::Get};
use frame_system::ensure_signed;
use frame_system::ensure_root;
use frame_support::traits::Vec;
// use pallet_balances;
use pallet_nft::{Token, Params, Socket, Rarity, TokenId};
use pallet_nft as NFT;
// use std::collections::HashSet;
use codec::{Decode, Encode, EncodeLike};
use sp_runtime::{
	RuntimeDebug, DispatchResult, DispatchError,
	traits::{
		StaticLookup
	},
};

decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Config>::AccountId {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(TokenId, AccountId),
		TokenMinted(AccountId, TokenId),
		TokenBurned(Token),
		TokenTransferred(TokenId, AccountId),
        TokenBreeded(TokenId),
        // TokensTransferred(TokenId, AccountId, TokenId, AccountId),
	}
);

pub trait Config: frame_system::Config + pallet_nft::Config + pallet_balances::Config{
    /// Because this pallet emits events, it depends on the runtime's definition of an event.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
        		// Errors must be initialized if they are used by the pallet.
		// type Error = Error<T>;

		// Events must be initialized if they are used by the pallet.
		// fn deposit_event() = default;

        #[weight = 10_000]
        pub fn mint_nft(
            origin, 
            target_account: <T as frame_system::Config>::AccountId,
            // token_id: TokenId,
            // rarity: Rarity,
            // socket: Socket,
            // params: Params,
        ) {
            /*dispatch::DispatchResult*/ 
            // let token_id = 3;
            // let rarity = NFT::Rarity::Legendary;
            // let socket = NFT::Socket::Head;
            // let params = NFT::Params{strength: 2,
            //     agility: 2,
            //     intelligence: 9 };
        //    return NFT::Module::<T>::mint(origin, target_account, token_id, rarity, socket, params);
        }

        #[weight = 10_000]
        fn breed_nft(origin, target_account: <T as frame_system::Config>::AccountId) -> dispatch::DispatchResult {
            let token_id = 3;
            let rarity = NFT::Rarity::Legendary;
            let socket = NFT::Socket::Head;
            let params = NFT::Params{strength: 2,
                agility: 2,
                intelligence: 9 };
            NFT::Module::<T>::burn(origin.clone(), 1);
            NFT::Module::<T>::burn(origin.clone(), 2);
            return NFT::Module::<T>::mint(origin, target_account, token_id, rarity, socket, params);
        }

        #[weight = 10_000]
        fn transfer_nft(origin, dest_account: T::AccountId) {
            let json_nft = 123;
            return NFT::Module::<T>::transfer(origin, dest_account, json_nft);
        }


        #[weight = 10_000]
        fn transfer(origin, dest: <T::Lookup as StaticLookup>::Source, value: T::Balance) -> dispatch::DispatchResultWithPostInfo {
            pallet_balances::Pallet::<T>::transfer(origin, dest, value)
		}
    }
}