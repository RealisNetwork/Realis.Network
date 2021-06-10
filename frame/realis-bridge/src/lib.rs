// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

use chain_bridge as bridge;
use frame_support::traits::{Currency, EnsureOrigin, ExistenceRequirement::AllowDeath, Get};
use frame_support::{
    decl_error, decl_event, decl_module, dispatch::DispatchResult, ensure,
    traits::ExistenceRequirement,
};
use frame_system::{self as system, ensure_signed};
use pallet_nft as erc721;
use sp_arithmetic::traits::SaturatedConversion;
use sp_core::U256;
use sp_std::prelude::*;

mod mock;
mod tests;

type ResourceId = bridge::ResourceId;

type BalanceOf<T> =
    <<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

pub trait Config: system::Config + bridge::Config + erc721::Config {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
    /// Specifies the origin check provided by the bridge for calls that can only be called by the bridge pallet
    type BridgeOrigin: EnsureOrigin<Self::Origin, Success = Self::AccountId>;

    /// The currency mechanism.
    type Currency: Currency<Self::AccountId>;

    /// Ids can be defined by the runtime and passed in, perhaps from blake2b_128 hashes.
    type HashId: Get<ResourceId>;
    type NativeTokenId: Get<ResourceId>;
    type Erc721Id: Get<ResourceId>;
}

decl_event! {
    pub enum Event<T> where
        <T as frame_system::Config>::Hash,
    {
        Remark(Hash),
    }
}

decl_error! {
    pub enum Error for Module<T: Config>{
        InvalidTransfer,
    }
}

decl_module! {
    pub struct Module<T: Config> for enum Call where origin: T::Origin {
        const HashId: ResourceId = T::HashId::get();
        const NativeTokenId: ResourceId = T::NativeTokenId::get();

        fn deposit_event() = default;

        /// Transfers an arbitrary hash to a (whitelisted) destination chain.
        #[weight = 195_000_000]
        pub fn transfer_hash(origin, hash: T::Hash, dest_id: bridge::ChainId) -> DispatchResult {
            ensure_signed(origin)?;

            let resource_id = T::HashId::get();
            let metadata: Vec<u8> = hash.as_ref().to_vec();
            <bridge::Module<T>>::transfer_generic(dest_id, resource_id, metadata)
        }

        /// Transfers some amount of the native token to some recipient on a (whitelisted) destination chain.
        #[weight = 195_000_000]
        pub fn transfer_native(origin, amount: BalanceOf<T>, recipient: Vec<u8>, dest_id: bridge::ChainId) -> DispatchResult {
            let source = ensure_signed(origin)?;
            ensure!(<bridge::Module<T>>::chain_whitelisted(dest_id), Error::<T>::InvalidTransfer);
            let bridge_id = <bridge::Module<T>>::account_id();
            T::Currency::transfer(&source, &bridge_id, amount, AllowDeath)?;

            let resource_id = T::NativeTokenId::get();
            <bridge::Module<T>>::transfer_fungible(dest_id, resource_id, recipient, U256::from(amount.saturated_into::<u128>()))
        }

        /// Transfers some amount of the realis tokens to some recipient on a (whitelisted) destination chain.
        #[weight = 195_000_000]
        pub fn transfer_realis_tokens(
            origin,
            #[compact] token_id: T::RealisTokenId,
            #[compact] value: T::Balance,
            recipient: Vec<u8>,
            dest_id: bridge::ChainId
        ) -> DispatchResult {
            let source = ensure_signed(origin)?;
            ensure!(<bridge::Module<T>>::chain_whitelisted(dest_id), Error::<T>::InvalidTransfer);
            <pallet_nft::Module<T>>::validate_realis_token_id(token_id)?;
            let bridge_id = <bridge::Module<T>>::account_id();
            <pallet_nft::Module<T>>::do_transfer(&source, &bridge_id, token_id, value, ExistenceRequirement::AllowDeath)?;
            let resource_id = bridge::derive_resource_id(dest_id, &Self::realis_token_acronym(token_id));
            <bridge::Module<T>>::transfer_fungible(dest_id, resource_id, recipient, U256::from(value.saturated_into::<u128>()))
        }

        /// Transfer a non-fungible token (erc721) to a (whitelisted) destination chain.
        #[weight = 195_000_000]
        pub fn transfer_erc721(origin, recipient: Vec<u8>, token_id: U256, dest_id: bridge::ChainId) -> DispatchResult {
            let _source = ensure_signed(origin)?;
            ensure!(<bridge::Module<T>>::chain_whitelisted(dest_id), Error::<T>::InvalidTransfer);
            match <erc721::Module<T>>::all_tokens_in_account(&token_id) {
                Some(_token) => {
                    <erc721::Module<T>>::burn_nft(token_id)?;
                    let resource_id = T::Erc721Id::get();
                    let tid: &mut [u8] = &mut[0; 32];
                    token_id.to_big_endian(tid);
                    <bridge::Module<T>>::transfer_nonfungible(dest_id, resource_id, tid.to_vec(), recipient)
                }
                None => Err(Error::<T>::InvalidTransfer.into())
            }
        }

        //
        // Executable calls. These can be triggered by a bridge transfer initiated on another chain
        //

        /// Executes a simple currency transfer using the bridge account as the source
        #[weight = 195_000_000]
        pub fn transfer(origin, to: T::AccountId, amount: BalanceOf<T>, _r_id: ResourceId) -> DispatchResult {
            let source = T::BridgeOrigin::ensure_origin(origin)?;
            <T as Config>::Currency::transfer(&source, &to, amount, AllowDeath)?;
            Ok(())
        }

        /// This can be called by the bridge to demonstrate an arbitrary call from a proposal.
        #[weight = 195_000_000]
        pub fn remark(origin, hash: T::Hash, _r_id: ResourceId) -> DispatchResult {
            T::BridgeOrigin::ensure_origin(origin)?;
            Self::deposit_event(RawEvent::Remark(hash));
            Ok(())
        }

        /// Allows the bridge to issue new erc721 tokens
        #[weight = 195_000_000]
        pub fn mint_erc721(origin, target_account: T::AccountId,
            token_id: pallet_nft::TokenId,
            rarity: pallet_nft::Rarity,
            socket: pallet_nft::Socket,
            params: pallet_nft::Params, _r_id: ResourceId
            ) -> DispatchResult {
            T::BridgeOrigin::ensure_origin(origin)?;

            let token = pallet_nft::Token {
               token_id,
               rarity,
               socket,
               params
            };

            <erc721::Module<T>>::mint_nft(&target_account, token_id, token)?;
            Ok(())
        }
    }
}

impl<T: Config> Module<T> {
    fn realis_token_acronym(token_id: T::RealisTokenId) -> Vec<u8> {
        let mut acronym = b"NET".to_vec();
        let postfix = U256::from(token_id.saturated_into::<u128>());
        let mut buf = vec![];
        postfix.to_big_endian(&mut buf);
        acronym.append(&mut buf);
        acronym
    }
}
