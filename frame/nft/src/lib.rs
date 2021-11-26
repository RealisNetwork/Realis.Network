#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::dispatch;
pub use pallet::*;
use sp_std::prelude::*;

// Add test modules
#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

// Add benchmarking modules
#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
pub mod weights;
pub use weights::WeightInfoNft;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use sp_runtime::ArithmeticError;
    use sp_std::borrow::ToOwned;

    use realis_primitives::*;

    use super::*;

    #[pallet::pallet]
    #[pallet::generate_store(pub (super) trait Store)]
    pub struct Pallet<T>(PhantomData<T>);

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
        /// Weight information for extrinsics in this pallet.
        type WeightInfoNft: WeightInfoNft;
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
        NftMinted(T::AccountId, TokenId),
        NftBurned(),
        NftTransferred(T::AccountId, T::AccountId, TokenId),
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// Token use now another wallet
        TokenExist,
        /// Not token owner
        NotTokenOwner,
        ///
        NonExistentToken,
        ///
        NotNftMaster,
        /// Got an overflow after adding
        Overflow,
        /// Nft Master was added early
        NftMasterWasAddedEarly,

        CannotTransferNftBecauseThisNftInMarketplace,

        CannotTransferNftBecauseThisNftOnAnotherUser,

        NftAlreadyInUse,
    }

    /// Map where
    /// key - TokenId
    /// value - AccountId to which belong this token
    #[pallet::storage]
    #[pallet::getter(fn account_for_token)]
    pub type AccountForToken<T: Config> = StorageMap<_, Blake2_256, TokenId, T::AccountId>;

    /// Map where
    /// key - AccountId
    /// value - number of tokens that belong to this account
    #[pallet::storage]
    #[pallet::getter(fn total_for_account)]
    pub type TotalForAccount<T: Config> =
        StorageMap<_, Twox64Concat, T::AccountId, u32, ValueQuery>;

    /// Map where (same as VecOfTokensOnAccount by not for Token, instead for Types)
    #[pallet::storage]
    #[pallet::getter(fn token_list)]
    pub type TokensList<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, Vec<(Token, Status)>, ValueQuery>;

    /// Contains vector of all accounts ???
    #[pallet::storage]
    #[pallet::getter(fn nft_masters)]
    pub type NftMasters<T: Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;

    #[pallet::genesis_config]
    pub struct GenesisConfig<T: Config> {
        pub nft_masters: Vec<T::AccountId>,
    }

    #[cfg(feature = "std")]
    impl<T: Config> Default for GenesisConfig<T> {
        fn default() -> Self {
            Self {
                nft_masters: Default::default(),
            }
        }
    }

    #[pallet::genesis_build]
    impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
        fn build(&self) {
            NftMasters::<T>::put(&self.nft_masters);
        }
    }

    #[cfg(feature = "std")]
    impl<T: Config> GenesisConfig<T> {
        /// Direct implementation of `GenesisBuild::build_storage`.
        ///
        /// Kept in order not to break dependency.
        pub fn build_storage(&self) -> Result<sp_runtime::Storage, std::string::String> {
            <Self as GenesisBuild<T>>::build_storage(self)
        }
    }

    #[pallet::hooks]
    impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {}

    /// Dispatchable functions allows users to interact with the pallet and invoke state changes.
    /// These functions materialize as "extrinsics", which are often compared to transactions.
    /// Dispatchable functions must be annotated with a weight and must return a DispatchResult.
    /// Call functions - available from outside
    #[pallet::call]
    impl<T: Config> Pallet<T> {
        /// Create mergeable token and push it to specific account
        /// Token arguments are determined by functions arguments: rarity, socket, params
        #[pallet::weight(T::WeightInfoNft::mint())]
        pub fn mint(
            origin: OriginFor<T>,
            target_account: T::AccountId,
            name: String,
            token_id: TokenId,
            id: u32,
            rarity: Rarity,
            link: String,
        ) -> DispatchResult {
            // Check is signed correct
            let who = ensure_signed(origin)?;
            // Check if account that signed operation have permission for this operation
            ensure!(Self::nft_masters().contains(&who), Error::<T>::NotNftMaster);
            // Create token by grouping up arguments
            let link = "https://ipfs.io/ipfs/".to_owned() + sp_std::str::from_utf8(&link).unwrap();
            let token = Token {
                id: token_id,
                token_type: TokenType::Basic(rarity, String::from(link), id, name),
            };

            // Push token on account
            Self::mint_nft(&target_account, token_id, token)?;
            // Call mint event
            Self::deposit_event(Event::NftMinted(target_account, token_id));

            Ok(())
        }

        // /// Burn mergeable token(only owner)
        // #[pallet::weight(T::WeightInfoNft::burn())]
        // pub fn burn(origin: OriginFor<T>, token_id: TokenId) -> DispatchResult {
        //     // Check is signed correct
        //     let origin = ensure_signed(origin)?;
        //     // Get owner by token_id
        //     let owner = Self::account_for_token(&token_id).ok_or(Error::<T>::NonExistentToken)?;
        //     // Only owner can burn token
        //     ensure!(origin == owner, Error::<T>::NotTokenOwner);
        //     // Burn token
        //     Self::burn_nft(token_id, &owner)?;
        //     // Call burn event
        //     Self::deposit_event(Event::NftBurned());
        //     Ok(())
        // }

        /// Transfer mergeable token(only owner)
        #[pallet::weight(T::WeightInfoNft::transfer())]
        pub fn transfer(
            origin: OriginFor<T>,
            dest_account: T::AccountId,
            token_id: TokenId,
        ) -> DispatchResult {
            // Check is signed correct
            let origin = ensure_signed(origin)?;
            // Get owner by token_id
            let owner = Self::account_for_token(&token_id).ok_or(Error::<T>::NonExistentToken)?;
            // Only owner can transfer token
            ensure!(origin == owner, Error::<T>::NotTokenOwner);
            Self::is_nft_free(token_id)?;
            // Transfer token
            Self::transfer_nft(&dest_account, &owner, token_id)?;
            // Call transfer event
            Self::deposit_event(Event::NftTransferred(origin, dest_account, token_id));
            Ok(())
        }

        /// Add new nft_master
        #[pallet::weight(90_000_000)]
        pub fn add_nft_master(origin: OriginFor<T>, account: T::AccountId) -> DispatchResult {
            // Check is signed correct
            let who = ensure_signed(origin)?;
            // Check if account that signed operation have permission for this operation
            ensure!(Self::nft_masters().contains(&who), Error::<T>::NotNftMaster);
            ensure!(
                !Self::nft_masters().contains(&account),
                Error::<T>::NftMasterWasAddedEarly
            );

            NftMasters::<T>::mutate(|nft_masters| {
                nft_masters.push(account);
            });
            Ok(())
        }

        /// Remove new nft_master
        #[pallet::weight(90_000_000)]
        pub fn remove_nft_master(origin: OriginFor<T>, account: T::AccountId) -> DispatchResult {
            // Check is signed correct
            let who = ensure_signed(origin)?;
            // Check if account that signed operation have permission for this operation
            ensure!(Self::nft_masters().contains(&who), Error::<T>::NotNftMaster);

            NftMasters::<T>::mutate(|nft_masters| {
                let index = nft_masters.iter().position(|token| *token == account);
                nft_masters.remove(index.unwrap())
            });
            Ok(())
        }
    }

    impl<T: Config> Pallet<T> {
        pub fn mint_nft(
            target_account: &T::AccountId,
            token_id: TokenId,
            token: Token,
        ) -> dispatch::result::Result<TokenId, dispatch::DispatchError> {
            ensure!(
                !AccountForToken::<T>::contains_key(token_id),
                Error::<T>::TokenExist
            );
            Self::inc_total_for_account(target_account)?;

            TokensList::<T>::append(&target_account, (token, Status::Free));

            AccountForToken::<T>::insert(token_id, &target_account);

            Ok(token_id)
        }

        pub fn burn_nft(token_id: TokenId, owner: &T::AccountId) -> dispatch::DispatchResult {
            Self::dec_total_for_account(owner)?;

            TokensList::<T>::mutate(&owner, |tuple_tokens| {
                tuple_tokens.retain(|token| token.0.id != token_id);
            });

            AccountForToken::<T>::remove(&token_id);

            Ok(())
        }

        pub fn transfer_nft(
            dest_account: &T::AccountId,
            owner: &T::AccountId,
            token_id: TokenId,
        ) -> dispatch::DispatchResult {
            ensure!(
                *owner != T::AccountId::default(),
                Error::<T>::NonExistentToken
            );

            Self::inc_total_for_account(dest_account)?;
            let token = Self::pop(token_id);
            AccountForToken::<T>::insert(token_id, dest_account);
            TokensList::<T>::append(dest_account, token);

            Ok(())
        }

        pub fn get_nft_status(token_id: TokenId) -> Option<Status> {
            let owner = AccountForToken::<T>::get(token_id).unwrap();
            let tokens = TokensList::<T>::get(&owner);

            tokens
                .iter()
                .find(|(token, _)| token.id == token_id)
                .map(|(_, status)| status.clone())
        }

        pub fn set_nft_status(token_id: TokenId, status: Status) {
            let owner = AccountForToken::<T>::get(token_id).unwrap();

            TokensList::<T>::mutate(owner, |tokens| {
                tokens.into_iter().for_each(|(token, current_status)| {
                    if token.id == token_id {
                        *current_status = status;
                    }
                })
            });
        }

        pub fn pop(token_id: TokenId) -> (Token, Status) {
            let owner = AccountForToken::<T>::get(token_id).unwrap();
            let _result = Self::dec_total_for_account(&owner);

            AccountForToken::<T>::remove(token_id);
            TokensList::<T>::mutate(&owner, |tokens| {
                let index = tokens.iter().position(|token| token.0.id == token_id);
                tokens.remove(index.unwrap())
            })
        }

        pub fn inc_total_for_account(account: &T::AccountId) -> Result<(), ArithmeticError> {
            TotalForAccount::<T>::try_mutate(account, |cnt| {
                cnt.checked_add(1)
                    .map_or(Err(ArithmeticError::Overflow), |new_cnt| {
                        *cnt = new_cnt;
                        Ok(())
                    })
            })
        }

        pub fn dec_total_for_account(account: &T::AccountId) -> Result<(), ArithmeticError> {
            TotalForAccount::<T>::try_mutate(account, |cnt| {
                cnt.checked_sub(1)
                    .map_or(Err(ArithmeticError::Overflow), |new_cnt| {
                        *cnt = new_cnt;
                        Ok(())
                    })
            })
        }

        pub fn is_nft_free(token_id: TokenId) -> DispatchResult {
            match Self::get_nft_status(token_id) {
                None => Err(Error::<T>::NonExistentToken)?,
                Some(Status::OnSell | Status::InDelegation | Status::OnDelegateSell) => {
                    Err(Error::<T>::NftAlreadyInUse)?
                }
                Some(Status::Free) => {}
            }

            Ok(())
        }
    }
}
