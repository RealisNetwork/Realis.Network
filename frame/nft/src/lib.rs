#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame

use frame_support::{decl_module, decl_storage, decl_event, decl_error, ensure, dispatch, traits::Get};
use frame_system::ensure_signed;
use frame_system::ensure_root;
use frame_support::traits::Vec;
// use std::collections::HashSet;
use codec::{Decode, Encode};

pub mod nft;
pub use crate::nft::Nft;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub type TokenId = u32;

#[derive(Encode, Decode, Clone, Eq, PartialEq, PartialOrd, Ord, Debug)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Mythical,
    Legendary,
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, PartialOrd, Ord, Debug)]
pub enum Socket {
    Head,
    Body,
    LegLeft,
    RightLeft,
    ArmLeft,
    ArmRight,
    Weapon,
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, PartialOrd, Ord, Debug)]
pub struct Params {
    strength: u8,
    agility: u8,
    intelligence: u8,
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, PartialOrd, Ord, Debug)]

pub struct Token {
    token_id: TokenId,
    rarity: Rarity,
    socket: Socket,
    params: Params,
}

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Config: frame_system::Config {
    /// Because this pallet emits events, it depends on the runtime's definition of an event.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

// impl <T: Config> Module<T> {
// // show tokens
// // #[weight = 10_000 + T::DbWeight::get().reads_writes(1,1)]
//     fn get_tokens(user_id: T::AccountId) -> Vec<TokenId> {
//     // OwnedTokensArray::get(&user_id)
//         <Module<T>>::tokens_of_owner_by_index(user_id)
//     }
// }

// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
	// A unique name is used to ensure that the pallet's storage items are isolated.
	// This name may be updated, but each pallet in the runtime must use a unique name.
	// ---------------------------------vvvvvvvvvvvvvv
	trait Store for Module<T: Config> as TemplateModule {
		// Learn more about declaring storage items:
		// https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
		Something get(fn something): Option<TokenId>;
		TokensForAccount get(fn tokens_of_owner_by_index): double_map hasher(opaque_blake2_256) T::AccountId, hasher(opaque_blake2_256) TokenId => Vec<Token>;
		// OwnedTokensArray get(fn tokens_of_owner_by_index): map hasher(opaque_blake2_256) T::AccountId => HashSet<token_id>;
        // pub SomeMap get(fn some_map): map hasher(blake2_128_concat) T::AccountId => token_id;
		// Tok
        AccountForToken get(fn account_for_token): map hasher(opaque_blake2_256) TokenId => T::AccountId;
        TotalForAccount get(fn total_for_account): map hasher(blake2_128_concat) T::AccountId => u32;
	}
}

// Pallets use events to inform users when important changes are made.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Config>::AccountId {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(TokenId, AccountId),
		TokenMinted(AccountId, TokenId),
		TokenBurned(TokenId),
		TokenTransferred(TokenId, AccountId),
        TokenBreeded(TokenId),
	}
);

// Errors inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Config> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		///
		TokenExist,
		///
		NotTokenOwner,
		///
		NonExistentToken,
	}
}

// Dispatchable functions allows users to interact with the pallet and invoke state changes.
// These functions materialize as "extrinsics", which are often compared to transactions.
// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
decl_module! {
	pub struct Module<T: Config> for enum Call where origin: T::Origin {
		// Errors must be initialized if they are used by the pallet.
		type Error = Error<T>;

		// Events must be initialized if they are used by the pallet.
		fn deposit_event() = default;

		/// Mint token
		#[weight = 10_000 + T::DbWeight::get().reads_writes(1,1)]
		pub fn mint(origin, target_account: T::AccountId,
		token_id: TokenId,
        rarity: Rarity,
        socket: Socket,
        params: Params,
        /*token_info: Token,*/
        ) ->dispatch::DispatchResult {
		    let _who = ensure_root(origin)?;

		    let token_info = Token {
		        token_id,
		        rarity,
		        socket,
		        params
		    };

            let id_of_token = <Self as Nft<_>>::mint(&target_account, token_info, token_id)?;
		    Self::deposit_event(RawEvent::TokenMinted(target_account.clone(), id_of_token));
            Ok(())

		}

		// ///Burn token(only owner)
		// #[weight = 10_000 + T::DbWeight::get().reads_writes(1,1)]
		// pub fn burn(origin, token_id: TokenId) -> dispatch::DispatchResult {
        //     let who = ensure_signed(origin)?;
        //     ensure!(who == Self::account_for_token(&token_id), Error::<T>::NotTokenOwner);
		// 	let id_of_token = <Self as Nft<_>>::burn(token_id)?;
		//     Self::deposit_event(RawEvent::TokenBurned(id_of_token.clone()));
        //     Ok(())
        // }

        // ///Transfer token(only owner)
        // #[weight = 10_000]
        // pub fn transfer(origin, dest_account: T::AccountId, token_id: TokenId) -> dispatch::DispatchResult {
        //     let who = ensure_signed(origin)?;
        //     ensure!(who == Self::account_for_token(&token_id), Error::<T>::NotTokenOwner);
        //
        //     <Self as Nft<_>>::transfer(&dest_account, &token_id)?;
        //     Self::deposit_event(RawEvent::TokenTransferred(token_id.clone(), dest_account.clone()));
        //     Ok(())
        // }

        // ///Breed tokens(only owner)
        // #[weight = 10_000]
        // pub fn breed_token(origin, token_id: TokenId, token_id2: TokenId) -> dispatch::DispatchResult {
        //     let who = ensure_signed(origin)?;
        //     ensure!(who == Self::account_for_token(&token_id), Error::NotTokenOwner);
        //     ensure!(who == Self::account_for_token(&token_id2), Error::NotTokenOwner);

        //     let another_token = <Self as Nft<_>>::breed_token(token_id, token_id2)?;
        //     Self::deposit_event(RawEvent::TokenBreeded(another_token.clone()));
        // }

		// show tokens
		// #[weight = 10_000 + T::DbWeight::get().reads_writes(1,1)]
		// pub fn get_tokens(origin, user_id: token_id) -> Vec<token_id> {
		//     // OwnedTokensArray::get(&user_id)
		//     Self::tokens_of_owner_by_index(user_id)
		// }

	}
}

impl<T: Config> Nft<T::AccountId> for Module<T> {
    type Token = Token;

    type TokenId = TokenId;

    fn mint(target_account: &T::AccountId, token_info: Self::Token, token_id: Self::TokenId) -> dispatch::result::Result<Self::TokenId, dispatch::DispatchError> {
        // fn mint(target_account: &T::AccountId, token_id: Self::TokenId) -> dispatch::result::Result<Self::TokenId, _> {

        // if AccountForToken::contains_key(&token_id) {
        // Err(Error::<T>::TokenExist)
        // }
        ensure!(!AccountForToken::<T>::contains_key(token_info.token_id),
              Error::<T>::TokenExist
                );

        TokensForAccount::<T>::mutate(&target_account, token_info.token_id, |tokens| {
            match tokens.binary_search(&token_info) {
                Ok(_pos) => {},
                Err(pos) => tokens.insert(pos, token_info)
            }
        });
        // hash_set_of_tokens.insert(token_id);
        TotalForAccount::<T>::mutate(&target_account, |total| *total += 1);
        AccountForToken::<T>::insert(token_id, &target_account);
        // Self::deposit_event(RawEvent::TokenMinted(target_account, token_id));
        Ok(token_id)
    }

    // fn burn(token_id: TokenId) -> dispatch::result::Result<Token, dispatch::DispatchError> {
    //     let owner = Self::owner_of(&tokens_id);
    //     // ensure!(
    //     //     owner != T::AccountId::default(),
    //     //     Error::<T, I>::NonexistentToken
    //     // );
    //
    //     // TokensForAccount::<T>::mutate(|total| *total -= 1);
    //     // TokenBurned::<I>::mutate(|total| *total += 1);
    //     TotalForAccount::<T>::mutate(&owner, |total| *total -= 1);
    //     TokensForAccount::<T>::mutate(&owner, &tokens_id, |tokens| {
    //         let pos = tokens
    //             .binary_search(burn_token)
    //             .expect("We already checked that we have the correct owner; qed");
    //         tokens.remove(pos);
    //     });
    //     AccountForToken::<T>::remove(&tokens_id);
    //
    //     Ok(tokens_id)
    // }
    //
    // fn transfer(dest_account: &T::AccountId, token_info: &TokenId) -> dispatch::DispatchResult
    // {
    //     let owner = Self::owner_of(token_info);
    //     ensure!(
    //             owner != T::AccountId::default(),
    //             Error::<T>::NonExistentToken
    //         );
    //
    //     TotalForAccount::<T>::mutate(&owner, |total| *total -= 1);
    //     TotalForAccount::<T>::mutate(dest_account, |total| *total += 1);
    //     let token = TokensForAccount::<T>::mutate(owner, |tokens| {
    //         let pos = tokens
    //             .binary_search(token_info)
    //             .expect("We already checked that we have the correct owner; qed");
    //         tokens.remove(pos)
    //     });
    //     TokensForAccount::<T>::mutate(dest_account, |tokens| {
    //         match tokens.binary_search(&token) {
    //             Ok(_pos) => {} // should never happen
    //             Err(pos) => tokens.insert(pos, token),
    //         }
    //     });
    //     AccountForToken::<T>::insert(&token_info, &dest_account);
    //
    //     Ok(())
    // }

    // fn breed_token(token_id: TokenId, token_id2: TokenId) -> dispatch::result::Result<another_token, dispatch::DispatchError> {
    //     let owner = Self::owner_of(token_id);
    //     ensure!(
    //             owner != T::AccountId::default(),
    //             Error::<T>::NonExistentToken
    //         );
    //     let another_token = token_id;
    //     burn(token_id);
    //     burn(token_id2);
    //     mint(owner, another_token);
    //     Ok(another_token);
    // }

    fn owner_of(token_id: TokenId) -> T::AccountId {
        Self::account_for_token(token_id)
    }
}