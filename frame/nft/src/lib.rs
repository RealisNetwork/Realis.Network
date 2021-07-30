#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::dispatch;
pub use pallet::*;
use sp_runtime::traits::AtLeast32BitUnsigned;
use sp_std::prelude::*;

// Add test modules
#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

// Add benchmarking modules
mod benchmarking;
pub mod weights;
pub use weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {
    use frame_support::pallet_prelude::*;
    use frame_system::pallet_prelude::*;
    use sp_runtime::ArithmeticError;

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
        // type TokenId;
        type Balance: Member + Parameter + AtLeast32BitUnsigned + Default + Copy;
        /// Weight information for extrinsics in this pallet.
        type WeightInfo: WeightInfo;
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
        /// Event documentation should end with an array that provides descriptive names for event
        /// parameters. [something, who]
        SomethingStored(TokenId, T::AccountId),
        TokenMinted(T::AccountId, TokenId),
        TokenBurned(),
        BasicTokenBurned(TokenId),
        TokenTransferred(TokenId, T::AccountId),
        TokenBreeded(TokenId),
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        /// Error names should be descriptive.
        NoneValue,
        /// Errors should have helpful documentation associated with them.
        StorageOverflow,
        /// Token use now another wallet
        TokenExist,
        /// Not token owner
        NotTokenOwner,
        ///
        NonExistentToken,
        ///
        NotNftMaster,
        ///
        InvalidTokenId,
        /// Transfer amount should be non-zero
        AmountZero,
        /// Account balance must be greater than or equal to the transfer amount
        BalanceLow,
        /// Vesting balance too high to send value
        VestingBalance,
        /// Got an overflow after adding
        Overflow,
        /// Balance too low to send value
        InsufficientBalance,
        /// Value too low to create account due to existential deposit
        ExistentialDeposit,
    }
    /// Map where
    ///	key - AccountId
    /// value - vector vector of TokenId and Token that belong specific account for each account
    #[pallet::storage]
    #[pallet::getter(fn tokens_of_owner_by_index)]
    pub type VecOfTokensOnAccount<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, Vec<Token>>;

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
    pub(crate) type TotalForAccount<T: Config> =
        StorageMap<_, Twox64Concat, T::AccountId, u32, ValueQuery>;

    /// Map where (same as VecOfTokensOnAccount by not for Token, instead for Types)
    #[pallet::storage]
    #[pallet::getter(fn tokens_with_types)]
    pub type TokensWithTypes<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, Vec<Token>>;

    /// Contains vector of all accounts ???
    #[pallet::storage]
    #[pallet::getter(fn nft_masters)]
    pub type NftMasters<T: Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;

    // #[pallet::storage]
    // pub(crate) type SystemAccount<T: Config> = StorageMap<_, Blake2_128Concat, T::RealisTokenId, T::AccountId, AccountInfo<T::Index, AccountData<<T as Config>::Balance>>>;

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
        pub fn build_storage(&self) -> Result<sp_runtime::Storage, String> {
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
        #[pallet::weight(T::WeightInfo::mint())]
        pub fn mint(
            origin: OriginFor<T>,
            target_account: T::AccountId,
            token_id: TokenId,
            rarity: Rarity,
            socket: Socket,
            params: Params,
        ) -> DispatchResult {
            // Check is signed correct
            let who = ensure_signed(origin)?;
            // Check if account that signed operation have permission for this operation
            ensure!(Self::nft_masters().contains(&who), Error::<T>::NotNftMaster);
            // Create token by grouping up arguments
            let token = Token {
                id: token_id,
                token_type: TokenType::Mergeable(Mergeable {
                    rarity,
                    socket,
                    params,
                }),
            };

            // Push token on account
            Self::mint_nft(&target_account, token_id, token)?;
            // Call mint event
            Self::deposit_event(Event::TokenMinted(target_account, token_id));

            Ok(())
        }

        /// Create token(basic token) and push it to specific account
        /// Token arguments are determined by functions arguments: type
        #[pallet::weight(T::WeightInfo::mint_basic())]
        pub fn mint_basic(
            origin: OriginFor<T>,
            target_account: T::AccountId,
            token_id: TokenId,
            basic: Basic,
        ) -> DispatchResult {
            // Check is signed correct
            let who = ensure_signed(origin)?;
            // Check if account that signed operation have permission for this operation
            ensure!(Self::nft_masters().contains(&who), Error::<T>::NotNftMaster);
            // Create token by grouping up arguments
            let token = Token {
                id: token_id,
                token_type: TokenType::Basic(basic),
            };
            // Push token on account
            Self::mint_basic_nft(&target_account, token_id, token)?;
            // Call mint event
            Self::deposit_event(Event::TokenMinted(target_account, token_id));
            Ok(())
        }

        /// Burn mergeable token(only owner)
        #[pallet::weight(T::WeightInfo::burn())]
        pub fn burn(origin: OriginFor<T>, token_id: TokenId) -> DispatchResult {
            // Check is signed correct
            let origin = ensure_signed(origin)?;
            // Get owner by token_id
            let owner = Self::account_for_token(&token_id).ok_or(Error::<T>::NonExistentToken)?;
            // Only owner can burn token
            ensure!(origin == owner, Error::<T>::NotTokenOwner);
            // Burn token
            Self::burn_nft(token_id, &owner)?;
            // Call burn event
            Self::deposit_event(Event::TokenBurned());
            Ok(())
        }

        /// Burn basic token(only owner)
        #[pallet::weight(T::WeightInfo::burn_basic())]
        pub fn burn_basic(origin: OriginFor<T>, token_id: TokenId) -> DispatchResult {
            // Check is signed correct
            let origin = ensure_signed(origin)?;
            // Get owner by token_id
            let owner = Self::account_for_token(&token_id).ok_or(Error::<T>::NonExistentToken)?;
            // Only owner can burn token
            ensure!(origin == owner, Error::<T>::NotTokenOwner);
            // Burn token
            Self::burn_basic_nft(token_id, Some(owner))?;
            // Call burn event
            Self::deposit_event(Event::TokenBurned());
            Ok(())
        }

        /// Transfer mergeable token(only owner)
        #[pallet::weight(T::WeightInfo::transfer())]
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
            // Transfer token
            Self::transfer_nft(&dest_account, &owner, token_id)?;
            // Call transfer event
            Self::deposit_event(Event::TokenTransferred(token_id, dest_account));
            Ok(())
        }

        /// Transfer basic token(only owner)
        #[pallet::weight(T::WeightInfo::transfer_basic())]
        pub fn transfer_basic(
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
            // Transfer token
            Self::transfer_basic_nft(token_id, Some(owner), &dest_account)?;
            // Call transfer event
            Self::deposit_event(Event::TokenTransferred(token_id, dest_account));
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

            VecOfTokensOnAccount::<T>::mutate(&target_account, |tokens| {
                tokens.get_or_insert(Vec::default()).push(token);
            });

            AccountForToken::<T>::insert(token_id, &target_account);

            Ok(token_id)
        }

        pub fn mint_basic_nft(
            target_account: &T::AccountId,
            token_id: TokenId,
            basic_tokens: Token,
        ) -> dispatch::result::Result<TokenId, dispatch::DispatchError> {
            ensure!(
                !AccountForToken::<T>::contains_key(token_id),
                Error::<T>::TokenExist
            );

            Self::inc_total_for_account(target_account)?;

            TokensWithTypes::<T>::mutate(&target_account, |tokens| {
                tokens.get_or_insert(Vec::default()).push(basic_tokens);
            });

            AccountForToken::<T>::insert(token_id, &target_account);

            Ok(token_id)
        }

        pub fn burn_nft(token_id: TokenId, owner: &T::AccountId) -> dispatch::DispatchResult {
            Self::dec_total_for_account(owner)?;

            VecOfTokensOnAccount::<T>::mutate(owner, |tokens| {
                tokens
                    .as_mut()
                    .unwrap()
                    .retain(|token| token.id != token_id);
            });

            AccountForToken::<T>::remove(&token_id);

            Ok(())
        }

        pub fn burn_basic_nft(
            token_id: TokenId,
            owner: Option<T::AccountId>,
        ) -> dispatch::DispatchResult {
            let owner = match owner {
                Some(owner) => owner,
                None => Self::account_for_token(&token_id).ok_or(Error::<T>::NonExistentToken)?,
            };

            Self::dec_total_for_account(&owner)?;

            TokensWithTypes::<T>::mutate(&owner, |tuple_tokens| {
                tuple_tokens
                    .as_mut()
                    .unwrap()
                    .retain(|token| token.id != token_id);
            });

            AccountForToken::<T>::remove(&token_id);

            Ok(())
        }

        fn transfer_nft(
            dest_account: &T::AccountId,
            owner: &T::AccountId,
            token_id: TokenId,
        ) -> dispatch::DispatchResult {
            ensure!(
                *owner != T::AccountId::default(),
                Error::<T>::NonExistentToken
            );

            Self::dec_total_for_account(owner)?;
            Self::inc_total_for_account(dest_account)?;

            AccountForToken::<T>::insert(token_id, dest_account);

            // Remove token from current owner
            let token = VecOfTokensOnAccount::<T>::mutate(owner, |tokens| {
                let tokens_mut = tokens.as_mut().unwrap();
                let index = tokens_mut.iter().position(|token| token.id == token_id);
                tokens_mut.remove(index.unwrap())
            });

            // Transfer token to dest_account
            VecOfTokensOnAccount::<T>::mutate(dest_account, |tokens| {
                tokens.get_or_insert(Vec::default()).push(token);
            });

            Ok(())
        }

        pub fn transfer_basic_nft(
            token_id: TokenId,
            owner: Option<T::AccountId>,
            dest_account: &T::AccountId,
        ) -> dispatch::DispatchResult {
            let owner = match owner {
                Some(owner) => owner,
                None => Self::account_for_token(&token_id).ok_or(Error::<T>::NonExistentToken)?,
            };

            ensure!(
                owner != T::AccountId::default(),
                Error::<T>::NonExistentToken
            );

            Self::dec_total_for_account(&owner)?;
            Self::inc_total_for_account(dest_account)?;

            AccountForToken::<T>::insert(token_id, dest_account);

            // Remove token from current owner
            let token = TokensWithTypes::<T>::mutate(&owner, |tokens| {
                let tokens_mut = tokens.as_mut().unwrap();
                let index = tokens_mut.iter().position(|token| token.id == token_id);
                tokens_mut.remove(index.unwrap())
            });

            // Transfer token to dest_account
            TokensWithTypes::<T>::mutate(dest_account, |tokens| {
                tokens.get_or_insert(Vec::default()).push(token);
            });

            Ok(())
        }

        fn inc_total_for_account(account: &T::AccountId) -> Result<(), ArithmeticError> {
            TotalForAccount::<T>::try_mutate(account, |cnt| {
                cnt.checked_add(1)
                    .map_or(Err(ArithmeticError::Overflow), |new_cnt| {
                        *cnt = new_cnt;
                        Ok(())
                    })
            })
        }

        fn dec_total_for_account(account: &T::AccountId) -> Result<(), ArithmeticError> {
            TotalForAccount::<T>::try_mutate(account, |cnt| {
                cnt.checked_sub(1)
                    .map_or(Err(ArithmeticError::Overflow), |new_cnt| {
                        *cnt = new_cnt;
                        Ok(())
                    })
            })
        }
    }
}
