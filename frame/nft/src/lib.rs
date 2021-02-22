#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame

use frame_support::{decl_module, decl_storage, decl_event, decl_error, ensure, dispatch, traits::Get};
use frame_system::ensure_signed;
use frame_system::ensure_root;
use frame_support::traits::Vec;
// use std::collections::HashSet;

pub mod nft;
pub use crate::nft::Nft;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

type token_id = u32;

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Config: frame_system::Config {
    /// Because this pallet emits events, it depends on the runtime's definition of an event.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
}

impl <T: Config> Module<T> {
// show tokens
// #[weight = 10_000 + T::DbWeight::get().reads_writes(1,1)]
    fn get_tokens(user_id: T::AccountId) -> Vec<token_id> {
    // OwnedTokensArray::get(&user_id)
        <Module<T>>::tokens_of_owner_by_index(user_id)
    }
}

// The pallet's runtime storage items.
// https://substrate.dev/docs/en/knowledgebase/runtime/storage
decl_storage! {
	// A unique name is used to ensure that the pallet's storage items are isolated.
	// This name may be updated, but each pallet in the runtime must use a unique name.
	// ---------------------------------vvvvvvvvvvvvvv
	trait Store for Module<T: Config> as TemplateModule {
		// Learn more about declaring storage items:
		// https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
		Something get(fn something): Option<token_id>;
		TokensForAccount get(fn tokens_of_owner_by_index): map hasher(opaque_blake2_256) T::AccountId => Vec<token_id>;
		// OwnedTokensArray get(fn tokens_of_owner_by_index): map hasher(opaque_blake2_256) T::AccountId => HashSet<token_id>;
        // pub SomeMap get(fn some_map): map hasher(blake2_128_concat) T::AccountId => token_id;
		// Tok
        AccountForToken get(fn account_for_token): map hasher(opaque_blake2_256) token_id => T::AccountId;
        TotalForAccount get(fn total_for_account): map hasher(blake2_128_concat) T::AccountId => u32;
	}
}

// Pallets use events to inform users when important changes are made.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event!(
	pub enum Event<T> where AccountId = <T as frame_system::Config>::AccountId {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(token_id, AccountId),
		TokenMinted(AccountId, token_id),
		TokenBurned(token_id),
		TokenTransferred(token_id, AccountId),
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

		/// An example dispatchable that takes a singles value as a parameter, writes the value to
		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
		#[weight = 10_000 + T::DbWeight::get().writes(1)]
		pub fn do_something(origin, something: token_id) -> dispatch::DispatchResult {
			// Check that the extrinsic was signed and get the signer.
			// This function will return an error if the extrinsic is not signed.
			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
			let who = ensure_signed(origin)?;

			// Update storage.
			Something::put(something);

			// Emit an event.
			Self::deposit_event(RawEvent::SomethingStored(something, who));
			// Return a successful DispatchResult
			Ok(())
		}

		/// An example dispatchable that may throw a custom error.
		#[weight = 10_000 + T::DbWeight::get().reads_writes(1,1)]
		pub fn cause_error(origin) -> dispatch::DispatchResult {
			let _who = ensure_signed(origin)?;

			// Read a value from storage.
			match Something::get() {
				// Return an error if the value has not been set.
				None => Err(Error::<T>::NoneValue)?,
				Some(old) => {
					// Increment the value read from storage; will error in the event of overflow.
					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
					// Update the value in storage with the incremented result.
					Something::put(new);
					Ok(())
				},
			}
		}

		/// mint token
		#[weight = 10_000 + T::DbWeight::get().reads_writes(1,1)]
		pub fn mint(origin, target_account: T::AccountId, token_info: token_id) ->dispatch::DispatchResult {
		    let _who = ensure_signed(origin)?;

            let id_of_token = <Self as Nft<_>>::mint(&target_account, token_info)?;
		    Self::deposit_event(RawEvent::TokenMinted(target_account.clone(), id_of_token));
            Ok(())

		}

		///burn token
		#[weight = 10_000 + T::DbWeight::get().reads_writes(1,1)]
		pub fn burn(origin, token_id: token_id) -> dispatch::DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(who == Self::account_for_token(&token_id), Error::<T>::NotTokenOwner);
			let id_of_token = <Self as Nft<_>>::burn(token_id)?;
		    Self::deposit_event(RawEvent::TokenBurned(id_of_token.clone()));
            Ok(())
        }

        #[weight = 10_000]
        pub fn transfer(origin, dest_account: T::AccountId, token_id: token_id) -> dispatch::DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(who == Self::account_for_token(&token_id), Error::<T>::NotTokenOwner);

            <Self as Nft<_>>::transfer(&dest_account, &token_id)?;
            Self::deposit_event(RawEvent::TokenTransferred(token_id.clone(), dest_account.clone()));
            Ok(())
        }

		// show tokens
		// #[weight = 10_000 + T::DbWeight::get().reads_writes(1,1)]
		// pub fn get_tokens(origin, user_id: token_id) -> Vec<token_id> {
		//     // OwnedTokensArray::get(&user_id)
		//     Self::tokens_of_owner_by_index(user_id)
		// }

	}
}

impl<T: Config> Nft<T::AccountId> for Module<T> {
    type TokenId = token_id;

    fn mint(target_account: &T::AccountId, token_info: Self::TokenId) -> dispatch::result::Result<Self::TokenId, dispatch::DispatchError> {
    // fn mint(target_account: &T::AccountId, token_id: Self::TokenId) -> dispatch::result::Result<Self::TokenId, _> {

        // if AccountForToken::contains_key(&token_id) {
        // Err(Error::<T>::TokenExist)
        // }

        ensure!(!AccountForToken::<T>::contains_key(&token_info),
              Error::<T>::TokenExist
                );

        TokensForAccount::<T>::mutate(target_account, |tokens| {
            match tokens.binary_search(&token_info) {
                Ok(_pos) => {},
                Err(pos) => tokens.insert(pos, token_info)
            }
        });
        // hash_set_of_tokens.insert(token_id);
        TotalForAccount::<T>::mutate(&target_account, |total| *total += 1);
        AccountForToken::<T>::insert(&token_info, &target_account);
        // Self::deposit_event(RawEvent::TokenMinted(target_account, token_id));
        Ok(token_info)
    }

    fn burn(tokens_id: token_id) -> dispatch::result::Result<token_id, dispatch::DispatchError> {
        let owner = Self::owner_of(&tokens_id);
        // ensure!(
        //     owner != T::AccountId::default(),
        //     Error::<T, I>::NonexistentToken
        // );

        let burn_token = (&tokens_id/*<T as Config<T>>::token_info::default(), */);

        // TokensForAccount::<T>::mutate(|total| *total -= 1);
        // TokenBurned::<I>::mutate(|total| *total += 1);
        TotalForAccount::<T>::mutate(&owner, |total| *total -= 1);
        TokensForAccount::<T>::mutate(owner, |tokens| {
            let pos = tokens
                .binary_search(&burn_token)
                .expect("We already checked that we have the correct owner; qed");
            tokens.remove(pos);
        });
        AccountForToken::<T>::remove(&tokens_id);

        Ok(tokens_id)
    }

    fn transfer(dest_account: &T::AccountId, token_info: &token_id) -> dispatch::DispatchResult
    {
        let owner = Self::owner_of(token_info);
        ensure!(
                owner != T::AccountId::default(),
                Error::<T>::NonExistentToken
            );

        TotalForAccount::<T>::mutate(&owner, |total| *total -= 1);
        TotalForAccount::<T>::mutate(dest_account, |total| *total += 1);
        let token = TokensForAccount::<T>::mutate(owner, |tokens| {
            let pos = tokens
                .binary_search(token_info)
                .expect("We already checked that we have the correct owner; qed");
            tokens.remove(pos)
        });
        TokensForAccount::<T>::mutate(dest_account, |tokens| {
            match tokens.binary_search(&token) {
                Ok(_pos) => {} // should never happen
                Err(pos) => tokens.insert(pos, token),
            }
        });
        AccountForToken::<T>::insert(&token_info, &dest_account);

        Ok(())
    }

    fn owner_of(token_id: &token_id) -> T::AccountId {
        Self::account_for_token(token_id)
    }
}




// #![cfg_attr(not(feature = "std"), no_std)]
//
// use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch, traits::Get, traits::Currency, StorageValue, StorageMap, ensure,  };
// use frame_system::ensure_signed;
// use sp_runtime::{traits::{Hash, Zero}};
// use codec::{Encode, Decode};
// // use std::cmp;
// #[cfg(test)]
// mod mock;
//
// #[cfg(test)]
// mod tests;
//
// // type TokenId = token_id;
// // pub enum TokenRarity {
// // 	A,
// // 	B,
// // 	C
// // }
//
// // #[derive(Decode, Clone, PartialEq, Eq)]
// // pub struct Token {
// // 	id: TokenId,
// // 	rarity: TokenRarity
// // }
//
// /// Configure the pallet by specifying the parameters and types on which it depends.
// pub trait Config: frame_system::Config {
// 	/// Because this pallet emits events, it depends on the runtime's definition of an event.
// 	type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
// }
//
//
// // The pallet's runtime storage items.
// // https://substrate.dev/docs/en/knowledgebase/runtime/storage
// decl_storage! {
// 	// A unique name is used to ensure that the pallet's storage items are isolated.
// 	// This name may be updated, but each pallet in the runtime must use a unique name.
// 	// ---------------------------------vvvvvvvvvvvvvv
// 	trait Store for Module<T: Config> as TemplateModule {
// 		MyU32: token_id;
//         MyBool get(my_bool_getter): bool;
// 		// pub TokensArray: Vec<Token>
// 		// // Learn more about declaring storage items:
// 		// // https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
// 		// Something get(fn something): Option<token_id>;
// 		// getAllTokens get(fn some):
// 	}
// }
//
// // Pallets use events to inform users when important changes are made.
// // https://substrate.dev/docs/en/knowledgebase/runtime/events
// decl_event!(
// 	pub enum Event<T> where AccountId = <T as frame_system::Config>::AccountId {
// 		/// Event documentation should end with an array that provides descriptive names for event
// 		/// parameters. [something, who]
// 		SomethingStored(token_id, AccountId),
// 	}
// );
//
// // Errors inform users that something went wrong.
// decl_error! {
// 	pub enum Error for Module<T: Config> {
// 		/// Error names should be descriptive.
// 		NoneValue,
// 		/// Errors should have helpful documentation associated with them.
// 		StorageOverflow,
// 	}
// }
//
// // Dispatchable functions allows users to interact with the pallet and invoke state changes.
// // These functions materialize as "extrinsics", which are often compared to transactions.
// // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
// decl_module! {
// 	pub struct Module<T: Config> for enum Call where origin: T::Origin {
// 		// Errors must be initialized if they are used by the pallet.
// 		type Error = Error<T>;
//
// 		// Events must be initialized if they are used by the pallet.
// 		fn deposit_event() = default;
//
// 		/// An example dispatchable that takes a singles value as a parameter, writes the value to
// 		/// storage and emits an event. This function must be dispatched by a signed extrinsic.
// 		#[weight = 10_000 + T::DbWeight::get().writes(1)]
// 		pub fn do_something(origin, something: token_id) -> dispatch::DispatchResult {
// 			// Check that the extrinsic was signed and get the signer.
// 			// This function will return an error if the extrinsic is not signed.
// 			// https://substrate.dev/docs/en/knowledgebase/runtime/origin
// 			let who = ensure_signed(origin)?;
//
// 			// Update storage.
// 			Something::put(something);
//
// 			// Emit an event.
// 			Self::deposit_event(RawEvent::SomethingStored(something, who));
// 			// Return a successful DispatchResult
// 			Ok(())
// 		}
//
// 		/// An example dispatchable that may throw a custom error.
// 		#[weight = 10_000 + T::DbWeight::get().reads_writes(1,1)]
// 		pub fn cause_error(origin) -> dispatch::DispatchResult {
// 			let _who = ensure_signed(origin)?;
//
// 			// Read a value from storage.
// 			match Something::get() {
// 				// Return an error if the value has not been set.
// 				None => Err(Error::<T>::NoneValue)?,
// 				Some(old) => {
// 					// Increment the value read from storage; will error in the event of overflow.
// 					let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
// 					// Update the value in storage with the incremented result.
// 					Something::put(new);
// 					Ok(())
// 				},
// 			}
// 		}
// 	}
// }
//
// // // #[derive(Encode, Decode, Clone, PartialEq)]
// // #[derive(Encode, Decode, Clone, PartialEq, Eq)]
// // #[cfg_attr(feature = "std", derive(Debug))]
// // pub enum rarity {
// //     A,
// //     B,
// //     C
// // }
// // #[derive(Encode, Decode, Clone, PartialEq, Eq, Default)]
// // pub struct Token /*<Hash , Balance> */ {
// //     id: token_id,
// //     rarity: rarity
// //     // dna: Hash,
// //     // price: Balance,
// //     // gen: u64,
// // }
// //
// // pub trait Config: frame_system::Config {
// //     // type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
// // }
// //
// // decl_storage! {
// // 	trait Store for Module<T: Config> as Tokens {
// // 		/// The lookup table for names.
// // 		AllTokens get(fn token): map hasher(blake2_128_concat) T::AccountId => Vec<Token>;
// // 		// AllTokens: map hasher(twox_64_concat) => Vec<Token>;
// // 	}
// // }
// //
// // decl_module! {
// //     pub struct Module<T: Config> for enum Call where origin: T::Origin {}
// // }
//
//
// //
// // decl_event!(
// //     pub enum Event<T>
// //     where
// //         <T as frame_system::Config>::AccountId,
// //         <T as system::Config>::Hash,
// //         <T as pallet_balances::Config>::Balance
// //     {
// //         Created(AccountId, Hash),
// //         PriceSet(AccountId, Hash, Balance),
// //         Transferred(AccountId, AccountId, Hash),
// //         Bought(AccountId, AccountId, Hash, Balance),
// //     }
// // );
// //
// // decl_storage! {
// //     trait Store for Module<T: Trait> as KittyStorage {
// //
// //         OwnedKitty get(fn kitty): map hasher(opaque_blake2_256) T::Hash => Kitty<T::Hash, T::Balance>;
// //         // Kitties get(fn kitty): map T::Hash => Kitty<T::Hash, T::Balance>;
// //         KittyOwner get(fn owner_of): map hasher(opaque_blake2_256) T::Hash => Option<T::AccountId>;
// //
// //         AllKittiesArray get(fn kitty_by_index): map u64 => T::Hash;
// //         AllKittiesCount get(fn all_kitties_count): u64;
// //         AllKittiesIndex: map T::Hash => u64;
// //
// //         OwnedKittiesArray get(kitty_of_owner_by_index): map hasher(opaque_blake2_256) (T::AccountId, u64) => T::Hash;
// //         OwnedKittiesCount get(owned_kitty_count): map hasher(opaque_blake2_256) T::AccountId => u64;
// //         OwnedKittiesIndex: map T::Hash => u64;
// //
// //         Nonce: u64;
// //     }
// // }
// //
// // decl_module! {
// //     pub struct Module<T: Trait> for enum Call where origin: T::Origin {
//
//     //     fn deposit_event<T>() = default;
//     //
//     //     fn create_kitty(origin) -> Result {
//     //         let sender = ensure_signed(origin)?;
//     //         let nonce = <Nonce<T>>::get();
//     //         let random_hash = (<system::Module<T>>::random_seed(), &sender, nonce)
//     //             .using_encoded(<T as system::Trait>::Hashing::hash);
//     //
//     //         let new_kitty = Kitty {
//     //             id: random_hash,
//     //             dna: random_hash,
//     //             price: <T::Balance as TryFrom<u64>>::try_from(0).unwrap(),
//     //             gen: 0,
//     //         };
//     //
//     //         Self::mint(sender, random_hash, new_kitty)?;
//     //
//     //         <Nonce<T>>::mutate(|n| *n += 1);
//     //
//     //         Ok(())
//     //     }
//     //
//     //     fn set_price(origin, kitty_id: T::Hash, new_price: T::Balance) -> Result {
//     //         let sender = ensure_signed(origin)?;
//     //
//     //         ensure!(<Kitties<T>>::exists(kitty_id), "This cat does not exist");
//     //
//     //         let owner = Self::owner_of(kitty_id).ok_or("No owner for this kitty")?;
//     //         ensure!(owner == sender, "You do not own this cat");
//     //
//     //         let mut kitty = Self::kitty(kitty_id);
//     //         kitty.price = new_price;
//     //
//     //         <Kitties<T>>::insert(kitty_id, kitty);
//     //
//     //         Self::deposit_event(RawEvent::PriceSet(sender, kitty_id, new_price));
//     //
//     //         Ok(())
//     //     }
//     //
//     //     fn transfer(origin, to: T::AccountId, kitty_id: T::Hash) -> Result {
//     //         let sender = ensure_signed(origin)?;
//     //
//     //         let owner = Self::owner_of(kitty_id).ok_or("No owner for this kitty")?;
//     //         ensure!(owner == sender, "You do not own this kitty");
//     //
//     //         Self::transfer_from(sender, to, kitty_id)?;
//     //
//     //         Ok(())
//     //     }
//     //
//     //     fn buy_kitty(origin, kitty_id: T::Hash, max_price: T::Balance) -> Result {
//     //         let sender = ensure_signed(origin)?;
//     //
//     //         ensure!(<Kitties<T>>::exists(kitty_id), "This cat does not exist");
//     //
//     //         let owner = Self::owner_of(kitty_id).ok_or("No owner for this kitty")?;
//     //         ensure!(owner != sender, "You can't buy your own cat");
//     //
//     //         let mut kitty = Self::kitty(kitty_id);
//     //
//     //         let kitty_price = kitty.price;
//     //         ensure!(!kitty_price.is_zero(), "The cat you want to buy is not for sale");
//     //         ensure!(kitty_price <= max_price, "The cat you want to buy costs more than your max price");
//     //
//     //         <balances::Module<T> as Currency<_>>::transfer(&sender, &owner, kitty_price)?;
//     //
//     //         Self::transfer_from(owner.clone(), sender.clone(), kitty_id)
//     //             .expect("`owner` is shown to own the kitty; \
//     //             `owner` must have greater than 0 kitties, so transfer cannot cause underflow; \
//     //             `all_kitty_count` shares the same type as `owned_kitty_count` \
//     //             and minting ensure there won't ever be more than `max()` kitties, \
//     //             which means transfer cannot cause an overflow; \
//     //             qed");
//     //
//     //         kitty.price = <T::Balance as As<u64>>::sa(0);
//     //         <Kitties<T>>::insert(kitty_id, kitty);
//     //
//     //         Self::deposit_event(RawEvent::Bought(sender, owner, kitty_id, kitty_price));
//     //
//     //         Ok(())
//     //     }
//     //
//     //     fn breed_kitty(origin, kitty_id_1: T::Hash, kitty_id_2: T::Hash) -> Result{
//     //         let sender = ensure_signed(origin)?;
//     //
//     //         ensure!(<Kitties<T>>::exists(kitty_id_1), "This cat 1 does not exist");
//     //         ensure!(<Kitties<T>>::exists(kitty_id_2), "This cat 2 does not exist");
//     //
//     //         let nonce = <Nonce<T>>::get();
//     //         let random_hash = (<system::Module<T>>::random_seed(), &sender, nonce)
//     //             .using_encoded(<T as system::Trait>::Hashing::hash);
//     //
//     //         let kitty_1 = Self::kitty(kitty_id_1);
//     //         let kitty_2 = Self::kitty(kitty_id_2);
//     //
//     //         let mut final_dna = kitty_1.dna;
//     //
//     //         for (i, (dna_2_element, r)) in kitty_2.dna.as_ref().iter().zip(random_hash.as_ref().iter()).enumerate() {
//     //             if r % 2 == 0 {
//     //                 final_dna.as_mut()[i] = *dna_2_element;
//     //             }
//     //         }
//     //
//     //         let new_kitty = Kitty {
//     //             id: random_hash,
//     //             dna: final_dna,
//     //             price: <T::Balance as As<u64>>::sa(0),
//     //             gen: cmp::max(kitty_1.gen, kitty_2.gen) + 1,
//     //         };
//     //
//     //         Self::mint(sender, random_hash, new_kitty)?;
//     //
//     //         <Nonce<T>>::mutate(|n| *n += 1);
//     //
//     //         Ok(())
//     //     }
//     // }
// // }
//
// // impl<T: Trait> Module<T> {
//     // fn mint(to: T::AccountId, kitty_id: T::Hash, new_kitty: Kitty<T::Hash, T::Balance>) -> Result {
//     //     ensure!(!<KittyOwner<T>>::exists(kitty_id), "Kitty already exists");
//     //
//     //     let owned_kitty_count = Self::owned_kitty_count(&to);
//     //
//     //     let new_owned_kitty_count = owned_kitty_count.checked_add(1)
//     //         .ok_or("Overflow adding a new kitty to account balance")?;
//     //
//     //     let all_kitties_count = Self::all_kitties_count();
//     //
//     //     let new_all_kitties_count = all_kitties_count.checked_add(1)
//     //         .ok_or("Overflow adding a new kitty to total supply")?;
//     //
//     //     <Kitties<T>>::insert(kitty_id, new_kitty);
//     //     <KittyOwner<T>>::insert(kitty_id, &to);
//     //
//     //     <AllKittiesArray<T>>::insert(all_kitties_count, kitty_id);
//     //     <AllKittiesCount<T>>::put(new_all_kitties_count);
//     //     <AllKittiesIndex<T>>::insert(kitty_id, all_kitties_count);
//     //
//     //     <OwnedKittiesArray<T>>::insert((to.clone(), owned_kitty_count), kitty_id);
//     //     <OwnedKittiesCount<T>>::insert(&to, new_owned_kitty_count);
//     //     <OwnedKittiesIndex<T>>::insert(kitty_id, owned_kitty_count);
//     //
//     //     Self::deposit_event(RawEvent::Created(to, kitty_id));
//     //
//     //     Ok(())
//     // }
//
//     //fn transfer_from(from: T::AccountId, to: T::AccountId, kitty_id: T::Hash) -> Result {
//     //     let owner = Self::owner_of(kitty_id).ok_or("No owner for this kitty")?;
//     //
//     //     ensure!(owner == from, "'from' account does not own this kitty");
//     //
//     //     let owned_kitty_count_from = Self::owned_kitty_count(&from);
//     //     let owned_kitty_count_to = Self::owned_kitty_count(&to);
//     //
//     //     let new_owned_kitty_count_to = owned_kitty_count_to.checked_add(1)
//     //         .ok_or("Transfer causes overflow of 'to' kitty balance")?;
//     //
//     //     let new_owned_kitty_count_from = owned_kitty_count_from.checked_sub(1)
//     //         .ok_or("Transfer causes underflow of 'from' kitty balance")?;
//     //
//     //     // "Swap and pop"
//     //     let kitty_index = <OwnedKittiesIndex<T>>::get(kitty_id);
//     //     if kitty_index != new_owned_kitty_count_from {
//     //         let last_kitty_id = <OwnedKittiesArray<T>>::get((from.clone(), new_owned_kitty_count_from));
//     //         <OwnedKittiesArray<T>>::insert((from.clone(), kitty_index), last_kitty_id);
//     //         <OwnedKittiesIndex<T>>::insert(last_kitty_id, kitty_index);
//     //     }
//     //
//     //     <KittyOwner<T>>::insert(&kitty_id, &to);
//     //     <OwnedKittiesIndex<T>>::insert(kitty_id, owned_kitty_count_to);
//     //
//     //     <OwnedKittiesArray<T>>::remove((from.clone(), new_owned_kitty_count_from));
//     //     <OwnedKittiesArray<T>>::insert((to.clone(), owned_kitty_count_to), kitty_id);
//     //
//     //     <OwnedKittiesCount<T>>::insert(&from, new_owned_kitty_count_from);
//     //     <OwnedKittiesCount<T>>::insert(&to, new_owned_kitty_count_to);
//     //
//     //     Self::deposit_event(RawEvent::Transferred(from, to, kitty_id));
//     //
//     //     Ok(())
//     // }
// //}
//
//
// ///////////////////////
//
// // #![cfg_attr(not(feature = "std"), no_std)]
// //
// // /// Edit this file to define custom logic or remove it if it is not needed.
// // /// Learn more about FRAME and the core library of Substrate FRAME pallets:
// // /// https://substrate.dev/docs/en/knowledgebase/runtime/frame
// //
// // use frame_support::{decl_module, decl_storage, decl_event, decl_error, dispatch, traits::Get};
// // use frame_system::ensure_signed;
// //
// // // ACTION: Add `support::StorageValue` and `support::ensure` to the imports
// // use codec::{Encode, Decode};
// //
// // #[derive(Encode, Decode, Default, Clone, PartialEq)]
// // #[cfg_attr(feature = "std", derive(Debug))]
// // pub struct Kitty<Hash, Balance> {
// //     id: Hash,
// //     dna: Hash,
// //     price: Balance,
// //     gen: u64,
// // }
// //
// // pub trait Config: pallet_balances::Config {}
// //
// // decl_storage! {
// //     trait Store for Module<T: Trait> as KittyStorage {
// //         // ACTION: Add two new kitty storage items:
// //         //         - `Kitties` which maps a `T::Hash` to a `Kitty<T::Hash, T::Balance>`
// //         //         - `KittyOwner` which maps a `T::Hash` to an `Option<T::AccountId>`
// //
// //         // ACTION: Update `OwnedKitty` to store a `T::Hash`
// //         OwnedKitty get(fn kitty_of_owner): map hasher(opaque_blake2_256) T::AccountId => Kitty<T::Hash, T::Balance>;
// //
// //         // ACTION: Add a `u64` value named `Nonce`
// //     }
// // }
// //
// // decl_module! {
// //     pub struct Module<T: Trait> for enum Call where origin: T::Origin {
// //
// //         fn create_kitty(origin) -> Result {
// //             let sender = ensure_signed(origin)?;
// //
// //             // ACTION: Generate a `random_hash` using:
// //             //         - `<system::Module<T>>::random_seed()`
// //             //         - `sender`
// //             //         - `Nonce`
// //
// //             // ACTION: `ensure` our `random_hash` does not collide with an existing token
// //
// //             // ACTION: Update our Kitty to use this `random_hash` as the `id` and the `dna`
// //             let new_kitty = Kitty {
// //                 id: <T as system::Config>::Hashing::hash_of(&0),
// //                 dna: <T as system::Config>::Hashing::hash_of(&0),
// //                 price: <T::Balance as TryFrom<u64>>::try_from(0).unwrap(),
// //                 gen: 0,
// //             };
// //
// //             //TODO ensure!(!<Kitties<T>>::exists(new_id), "This new id already exists");
// //
// //             // ACTION: `insert` the storage for `Kitties`, should point from our kitty's id to the `Kitty` object
// //             // ACTION: `insert` the storage for `KittyOwner`, should point from our kitty's id to the owner
// //             // ACTION: Update the `OwnedKitty` storage below to store the kitty's id rather than the `Kitty` object
// //             <OwnedKitty<T>>::insert(&sender, new_kitty);
// //
// //             // ACTION: `mutate` the nonce to increment it by 1
// //             //   HINT: You can pass the closure `(|n| *n += 1)` into `mutate`
// //
// //             Ok(())
// //         }
// //     }
// // }
