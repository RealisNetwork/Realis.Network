// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

use chainbridge as bridge;
use frame_support::traits::{Currency, EnsureOrigin, ExistenceRequirement::AllowDeath, Get};
use frame_support::{decl_error, decl_event, decl_module, dispatch::DispatchResult, ensure};
use frame_system::{self as system, ensure_signed};
use sp_arithmetic::traits::SaturatedConversion;
use sp_core::U256;
use sp_std::prelude::*;

mod mock;
mod tests;

type ResourceId = bridge::ResourceId;

type BalanceOf<T> =
    <<T as Trait>::Currency as Currency<<T as frame_system::Trait>::AccountId>>::Balance;

pub trait Trait: system::Trait + bridge::Trait {
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
    /// Specifies the origin check provided by the bridge for calls that can only be called by the bridge pallet
    type BridgeOrigin: EnsureOrigin<Self::Origin, Success = Self::AccountId>;

    /// The currency mechanism.
    type Currency: Currency<Self::AccountId>;

    /// Ids can be defined by the runtime and passed in, perhaps from blake2b_128 hashes.
    type NativeTokenId: Get<ResourceId>;
}

decl_event! {
    pub enum Event<T> where
        AccountId = <T as frame_system::Trait>::AccountId,
    {
        Who(AccountId),
    }
}

decl_error! {
    pub enum Error for Module<T: Trait>{
        InvalidTransfer,
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        const NativeTokenId: ResourceId = T::NativeTokenId::get();

        fn deposit_event() = default;

        /// Transfers some amount of the native token to some recipient on a (whitelisted) destination chain.
        #[weight = 195_000_000]
        pub fn transfer_native(origin, amount: BalanceOf<T>, recipient: Vec<u8>, dest_id: bridge::ChainId) -> DispatchResult {
            let source = ensure_signed(origin)?;
            ensure!(<bridge::Module<T>>::chain_whitelisted(dest_id), Error::<T>::InvalidTransfer);
            let bridge_id = <bridge::Module<T>>::account_id();
            T::Currency::transfer(&source, &bridge_id, amount.into(), AllowDeath)?;

            let resource_id = T::NativeTokenId::get();
            <bridge::Module<T>>::transfer_fungible(dest_id, resource_id, recipient, U256::from(amount.saturated_into::<u128>()))
        }

        //
        // Executable calls. These can be triggered by a bridge transfer initiated on another chain
        //

        /// Executes a simple currency transfer using the bridge account as the source
        #[weight = 195_000_000]
        pub fn transfer(origin, to: T::AccountId, amount: BalanceOf<T>, r_id: ResourceId) -> DispatchResult {
            let source = T::BridgeOrigin::ensure_origin(origin)?;
            <T as Trait>::Currency::transfer(&source, &to, amount.into(), AllowDeath)?;
            Ok(())
        }
    }
}
