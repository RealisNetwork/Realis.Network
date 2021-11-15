#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::{PhantomData, IsType};

    #[pallet::pallet]
    #[pallet::generate_store(pub (super) trait Store)]
    pub struct Pallet<T>(PhantomData<T>);

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    #[pallet::event]
    #[pallet::generate_deposit(pub (super) fn deposit_event)]
    #[pallet::metadata(T::AccountId = "AccountId", RealisTokenId = "T::RealisTokenId")]
    pub enum Event<T: Config> {}


    #[pallet::error]
    pub enum Error<T> {}

    // #[pallet::genesis_config]
    // pub struct GenesisConfig<T: Config> {}
    //
    // #[cfg(feature = "std")]
    // impl<T: Config> Default for GenesisConfig<T> {
    //     fn default() -> Self {
    //         Self {}
    //     }
    // }
    //
    // #[pallet::genesis_build]
    // impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
    //     fn build(&self) {}
    // }
    //
    // #[cfg(feature = "std")]
    // impl<T: Config> GenesisConfig<T> {
    //     /// Direct implementation of `GenesisBuild::build_storage`.
    //     ///
    //     /// Kept in order not to break dependency.
    //     pub fn build_storage(&self) -> Result<sp_runtime::Storage, std::string::String> {
    //         <Self as GenesisBuild<T>>::build_storage(self)
    //     }
    // }

    #[pallet::call]
    impl<T: Config> Pallet<T> {}

    impl<T: Config> Pallet<T> {}
}
