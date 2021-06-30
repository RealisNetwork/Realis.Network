#![cfg_attr(not(feature = "std"), no_std)]
#![feature(option_result_contains)]

use frame_support::{
	dispatch,
	traits::{OnNewAccount, WithdrawReasons},
};
use frame_system::RefCount;
pub use pallet::*;
pub use primitive_types::U256;
use sp_runtime::traits::{AtLeast32BitUnsigned, Saturating};
use sp_std::prelude::*;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

/// Add benchmarking module
mod benchmarking;
pub mod weights;

pub use weights::WeightInfo;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use sp_runtime::ArithmeticError;

	use super::*;

	#[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
	pub struct Pallet<T>(PhantomData<T>);

	pub type TokenId = U256;

	#[derive(Encode, Decode, Clone, Eq, PartialEq, PartialOrd, Ord, Debug, Copy)]
	pub enum Rarity {
		Common,
		Uncommon,
		Rare,
		Mythical,
		Legendary,
	}

	#[derive(Encode, Decode, Clone, Eq, PartialEq, PartialOrd, Ord, Debug, Copy)]
	pub enum Socket {
		Head,
		Body,
		LegLeft,
		LegRight,
		ArmLeft,
		ArmRight,
		Weapon,
	}

	#[derive(Encode, Decode, Clone, Eq, PartialEq, PartialOrd, Ord, Debug, Copy, Default)]
	pub struct Types {
		pub tape: u8,
	}

	#[derive(Encode, Decode, Clone, Eq, PartialEq, PartialOrd, Ord, Debug, Copy)]
	pub struct Params {
		pub strength: u8,
		pub agility: u8,
		pub intelligence: u8,
	}

	#[derive(Encode, Decode, Clone, Eq, PartialEq, PartialOrd, Ord, Debug, Copy)]
	pub struct Token {
		pub token_id: TokenId,
		pub rarity: Rarity,
		pub socket: Socket,
		pub params: Params,
	}

	/// Simplified reasons for withdrawing balance.
	#[derive(Encode, Decode, Clone, Copy, PartialEq, Eq, RuntimeDebug)]
	pub enum Reasons {
		/// Paying system transaction fees.
		Fee = 0,
		/// Any reason other than paying system transaction fees.
		Misc = 1,
		/// Any reason at all.
		All = 2,
	}

	impl From<WithdrawReasons> for Reasons {
		fn from(r: WithdrawReasons) -> Reasons {
			if r == WithdrawReasons::TRANSACTION_PAYMENT {
				Reasons::Fee
			} else if r.contains(WithdrawReasons::TRANSACTION_PAYMENT) {
				Reasons::All
			} else {
				Reasons::Misc
			}
		}
	}

	#[derive(Clone, Eq, PartialEq, Default, RuntimeDebug, Encode, Decode)]
	pub struct AccountInfo<Index, AccountData> {
		/// The number of transactions this account has sent.
		pub nonce: Index,
		/// The number of other modules that currently depend on this account's existence. The account
		/// cannot be reaped until this is zero.
		pub refcount: RefCount,
		/// The additional data that belongs to this account. Used to store the balance(s) in a lot of
		/// chains.
		pub data: AccountData,
	}

	#[derive(Encode, Decode, Clone, PartialEq, Eq, Default, RuntimeDebug)]
	pub struct AccountData<Balance> {
		/// Non-reserved part of the balance. There may still be restrictions on this, but it is the
		/// total pool what may in principle be transferred, reserved and used for tipping.
		///
		/// This is the only balance that matters in terms of most operations on tokens. It
		/// alone is used to determine the balance when in the contract execution environment.
		pub free: Balance,
		/// Balance which is reserved and may not be used at all.
		///
		/// This can still get slashed, but gets slashed last of all.
		///
		/// This balance is a 'reserve' balance that other subsystems use in order to set aside tokens
		/// that are still 'owned' by the account holder, but which are suspendable.
		pub reserved: Balance,
		/// The amount that `free` may not drop below when withdrawing for *anything except transaction
		/// fee payment*.
		pub misc_frozen: Balance,
		/// The amount that `free` may not drop below when withdrawing specifically for transaction
		/// fee payment.
		pub fee_frozen: Balance,
	}

	impl<Balance: Saturating + Copy + Ord> AccountData<Balance> {
		/// How much this account's balance can be reduced for the given `reasons`.
		#[allow(dead_code)]
		fn usable(&self, reasons: Reasons) -> Balance {
			self.free.saturating_sub(self.frozen(reasons))
		}
		/// The amount that this account's free balance may not be reduced beyond for the given
		/// `reasons`.
		fn frozen(&self, reasons: Reasons) -> Balance {
			match reasons {
				Reasons::All => self.misc_frozen.max(self.fee_frozen),
				Reasons::Misc => self.misc_frozen,
				Reasons::Fee => self.fee_frozen,
			}
		}
		// /// The total balance in this account including any that is reserved and ignoring any frozen.
		// fn total(&self) -> Balance {
		//     self.free.saturating_add(self.reserved)
		// }
	}

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// Because this pallet emits events, it depends on the runtime's definition of an event.
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
		// type TokenId;
		type Balance: Member + Parameter + AtLeast32BitUnsigned + Default + Copy;

		type OnNewAccount: OnNewAccount<(Self::RealisTokenId, Self::AccountId)>;

		type ExistentialDeposit: Get<Self::Balance>;

		type RealisTokenId: Parameter + AtLeast32BitUnsigned + Default + Copy;

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
		/// An account was created with some free balance. \[account, free_balance\]
		Endowed(T::AccountId, T::RealisTokenId, T::Balance),
		/// Some assets were transferred. \[token_id, from, to, amount\]
		Transfer(T::AccountId, T::AccountId, T::RealisTokenId, T::Balance),
		/// A balance was set by root. \[who, free, reserved\]
		RealisTokenBalanceSet(T::AccountId, TokenId, T::Balance, T::Balance),
		/// Some amount was deposited (e.g. for transaction fees). \[who, deposit\]
		Deposit(T::AccountId, TokenId, T::Balance),
		/// Some balance was reserved (moved from free to reserved). \[who, value\]
		Reserved(T::AccountId, TokenId, T::Balance),
		/// Some balance was unreserved (moved from reserved to free). \[who, value\]
		Unreserved(T::AccountId, TokenId, T::Balance),
		/// A new \[account\] was created.
		NewAccount(T::AccountId, T::RealisTokenId),
		TokenCreated(TokenId),
		/// Some assets were issued. [token_id, owner, total_supply]
		Issued(TokenId, T::AccountId, T::Balance),
		// TokensTransferred(TokenId, AccountId, TokenId, AccountId),
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
		/// Account liquidity restrictions prevent withdrawal
		LiquidityRestrictions,
		/// Got an overflow after adding
		Overflow,
		/// Balance too low to send value
		InsufficientBalance,
		/// Value too low to create account due to existential deposit
		ExistentialDeposit,
		/// Transfer/payment would kill account
		KeepAlive,
		/// A vesting schedule already exists for this account
		ExistingVestingSchedule,
		/// Beneficiary account must pre-exist
		DeadAccount,
		InvalidTransfer,
		/// Have no permission to transfer someone's balance
		NotAllowed,
	}

	#[pallet::storage]
	#[pallet::getter(fn max_token_id)]
	pub(super) type MaxTokenId<T: Config> = StorageValue<_, T::RealisTokenId, ValueQuery>;

	#[pallet::storage]
	pub(crate) type MinTokenId<T: Config> = StorageValue<_, T::RealisTokenId, ValueQuery>;

	/// Map where
	///	key - AccountId
	/// value - vector vector of TokenId and Token that belong specific account for each account
	#[pallet::storage]
	#[pallet::getter(fn tokens_of_owner_by_index)]
	pub(crate) type VecOfTokensOnAccount<T: Config> =
	StorageMap<_, Blake2_128Concat, T::AccountId, Vec<(TokenId, Token)>>;

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

	/// Map where (same as VecOfTokensOnAccount by not for Token, insted for Types)
	#[pallet::storage]
	#[pallet::getter(fn tokens_with_types)]
	pub(crate) type TokensWithTypes<T: Config> =
	StorageMap<_, Blake2_128Concat, T::AccountId, Vec<(TokenId, Types)>>;

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
		/// Create token and push it to specific account
		/// Token parametrs are determined by functions arguments: rarity, socket, params
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
				token_id,
				rarity,
				socket,
				params,
			};

			// Push token on account
			Self::mint_nft(&target_account, token_id, token)?;
			// Call mint event
			Self::deposit_event(Event::TokenMinted(target_account, token_id));

			Ok(())
		}

		/// Create token(type token) and push it to specific account
		/// Token parametrs are determined by functions arguments: type
		#[pallet::weight(T::WeightInfo::mint_basic())]
		pub fn mint_basic(
			origin: OriginFor<T>,
			target_account: T::AccountId,
			token_id: TokenId,
			type_token: Types,
		) -> DispatchResult {
			// Check is signed correct
			let who = ensure_signed(origin)?;
			// Check if account that signed operation have permission for this operation
			ensure!(Self::nft_masters().contains(&who), Error::<T>::NotNftMaster);
			// Push token on account
			Self::mint_basic_nft(&target_account, token_id, type_token)?;
			// Call mint event
			Self::deposit_event(Event::TokenMinted(target_account, token_id));
			Ok(())
		}

		/// Burn token(only owner)
		#[pallet::weight(T::WeightInfo::burn())]
		pub fn burn(origin: OriginFor<T>, token_id: TokenId) -> DispatchResult {
			let origin = ensure_signed(origin)?;
			let owner = Self::account_for_token(&token_id).ok_or(Error::<T>::NonExistentToken)?;
			ensure!(origin == owner, Error::<T>::NotTokenOwner);

			Self::burn_nft(token_id, &owner)?;
			Self::deposit_event(Event::TokenBurned());
			Ok(())
		}

		#[pallet::weight(T::WeightInfo::burn_basic())]
		pub fn burn_basic(origin: OriginFor<T>, token_id: TokenId) -> DispatchResult {
			let origin = ensure_signed(origin)?;
			let owner = Self::account_for_token(&token_id).ok_or(Error::<T>::NonExistentToken)?;
			ensure!(origin == owner, Error::<T>::NotTokenOwner);

			Self::burn_basic_nft(token_id, Some(owner))?;
			Self::deposit_event(Event::TokenBurned());
			Ok(())
		}

		/// Transfer token(only owner)
		#[pallet::weight(T::WeightInfo::transfer())]
		pub fn transfer(
			origin: OriginFor<T>,
			dest_account: T::AccountId,
			token_id: TokenId,
		) -> DispatchResult {
			let origin = ensure_signed(origin)?;
			let owner = Self::account_for_token(&token_id).ok_or(Error::<T>::NonExistentToken)?;
			ensure!(origin == owner, Error::<T>::NotTokenOwner);

			Self::transfer_nft(&dest_account, &owner, token_id)?;
			Self::deposit_event(Event::TokenTransferred(token_id, dest_account));
			Ok(())
		}

		#[pallet::weight(T::WeightInfo::transfer_basic())]
		pub fn transfer_basic(
			origin: OriginFor<T>,
			dest_account: T::AccountId,
			token_id: TokenId,
		) -> DispatchResult {
			let origin = ensure_signed(origin)?;
			let owner = Self::account_for_token(&token_id).ok_or(Error::<T>::NonExistentToken)?;
			ensure!(origin == owner, Error::<T>::NotTokenOwner);

			Self::transfer_basic_nft(token_id, Some(owner), &dest_account)?;
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
				tokens.get_or_insert(Vec::default()).push((token_id, token));
			});

			AccountForToken::<T>::insert(token_id, &target_account);

			Ok(token_id)
		}

		pub fn mint_basic_nft(
			target_account: &T::AccountId,
			token_id: TokenId,
			type_tokens: Types,
		) -> dispatch::result::Result<TokenId, dispatch::DispatchError> {
			ensure!(
                !AccountForToken::<T>::contains_key(token_id),
                Error::<T>::TokenExist
            );

			Self::inc_total_for_account(target_account)?;

			TokensWithTypes::<T>::mutate(&target_account, |tokens| {
				tokens
					.get_or_insert(Vec::default())
					.push((token_id, type_tokens));
			});

			AccountForToken::<T>::insert(token_id, &target_account);

			Ok(token_id)
		}

		pub fn burn_nft(token_id: TokenId, owner: &T::AccountId) -> dispatch::DispatchResult {
			Self::dec_total_for_account(owner)?;

			VecOfTokensOnAccount::<T>::mutate(owner, |tokens| {
				tokens.as_mut().unwrap().retain(|val| val.0 != token_id);
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
					.retain(|val| val.0 != token_id);
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

			// TODO check is owner in TokensForAccount
			// TODO check is owner own this token

			// Remove (token_id, token) pair from current owner
			let (token_id, token) = VecOfTokensOnAccount::<T>::mutate(owner, |tokens| {
				let tokens_mut = tokens.as_mut().unwrap();
				let index = tokens_mut.iter().position(|(id, _)| *id == token_id);
				tokens_mut.remove(index.unwrap())
			});

			// Transfer (token_id, token) to dest_account
			VecOfTokensOnAccount::<T>::mutate(dest_account, |tokens| {
				tokens.get_or_insert(Vec::default()).push((token_id, token));
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

			// Remove (token_id, token) pair from current owner
			let (token_id, token) = TokensWithTypes::<T>::mutate(&owner, |tokens| {
				let tokens_mut = tokens.as_mut().unwrap();
				let index = tokens_mut.iter().position(|(id, _)| *id == token_id);
				tokens_mut.remove(index.unwrap())
			});

			// Transfer (token_id, token) to dest_account
			TokensWithTypes::<T>::mutate(dest_account, |tokens| {
				tokens.get_or_insert(Vec::default()).push((token_id, token));
			});

			Ok(())
		}

		pub fn realis_token_ids() -> (T::RealisTokenId, T::RealisTokenId) {
			(<MinTokenId<T>>::get(), <MaxTokenId<T>>::get())
		}

		pub fn validate_realis_token_id(token_id: T::RealisTokenId) -> dispatch::DispatchResult {
			ensure!(
                token_id >= <MinTokenId<T>>::get() && token_id <= <MaxTokenId<T>>::get(),
                Error::<T>::InvalidTokenId
            );

			Ok(())
		}

		// TODO: cleanup unused stuff?
		// pub fn do_transfer(
		//     transactor: &T::AccountId,
		//     dest: &T::AccountId,
		//     token_id: T::RealisTokenId,
		//     value: <T as Config>::Balance,
		//     existence_requirement: ExistenceRequirement,
		// ) -> dispatch::DispatchResult {
		//     if value.is_zero() || transactor == dest {
		//         return Ok(());
		//     }
		//
		//     Self::try_mutate_account(
		//         dest,
		//         token_id,
		//         |to_account, _| -> dispatch::DispatchResult {
		//             Self::try_mutate_account(
		//                 transactor,
		//                 token_id,
		//                 |from_account, _| -> dispatch::DispatchResult {
		//                     from_account.free = from_account
		//                         .free
		//                         .checked_sub(&value)
		//                         .ok_or(Error::<T>::InsufficientBalance)?;
		//
		//                     // NOTE: total stake being stored in the same type means that this could never overflow
		//                     // but better to be safe than sorry.
		//                     to_account.free = to_account
		//                         .free
		//                         .checked_add(&value)
		//                         .ok_or(Error::<T>::Overflow)?;
		//
		//                     let ed = T::ExistentialDeposit::get();
		//                     ensure!(to_account.total() >= ed, Error::<T>::ExistentialDeposit);
		//
		//                     Self::ensure_can_withdraw(
		//                         transactor,
		//                         token_id,
		//                         value,
		//                         WithdrawReasons::TRANSFER,
		//                         from_account.free,
		//                     )?;
		//
		//                     let allow_death = existence_requirement == ExistenceRequirement::AllowDeath;
		//                     let allow_death = allow_death
		//                         && !frame_system::Pallet::<T>::is_provider_required(transactor);
		//                     ensure!(
		//                     allow_death || from_account.free >= ed,
		//                     Error::<T>::KeepAlive
		//                 );
		//
		//                     Ok(())
		//                 },
		//             )
		//         },
		//     )?;
		//
		//     // Emit transfer event.
		//     Self::deposit_event(Event::Transfer(
		//         transactor.clone(),
		//         dest.clone(),
		//         token_id,
		//         value,
		//     ));
		//
		//     Ok(())
		// }

		// fn try_mutate_account<R, E: From<StoredMapError>>(
		//     who: &T::AccountId,
		//     token_id: T::RealisTokenId,
		//     f: impl FnOnce(&mut AccountData<<T as Config>::Balance>, bool) -> Result<R, E>,
		// ) -> Result<R, E> {
		//     Self::try_mutate_exists(&(token_id, who.clone()), |maybe_account| {
		//         let is_new = maybe_account.is_none();
		//         let mut account = maybe_account.take().unwrap_or_default();
		//         f(&mut account, is_new).map(move |result| {
		//             let maybe_endowed = if is_new { Some(account.free) } else { None };
		//             *maybe_account = Self::post_mutation(who, account);
		//             (maybe_endowed, result)
		//         })
		//     })
		//         .map(|(maybe_endowed, result)| {
		//             if let Some(endowed) = maybe_endowed {
		//                 Self::deposit_event(Event::Endowed(who.clone(), token_id, endowed));
		//             }
		//             result
		//         })
		// }

		// fn ensure_can_withdraw(
		//     who: &T::AccountId,
		//     token_id: T::RealisTokenId,
		//     amount: T::Balance,
		//     reasons: WithdrawReasons,
		//     new_balance: T::Balance,
		// ) -> dispatch::DispatchResult {
		//     if amount.is_zero() {
		//         return Ok(());
		//     }
		//     let min_balance = Self::account(token_id, who).frozen(reasons.into());
		//     ensure!(
		//     new_balance >= min_balance,
		//     Error::<T>::LiquidityRestrictions
		// );
		//     Ok(())
		// }

		// fn try_mutate_exists<R, E: From<StoredMapError>>(
		//     k: &(T::RealisTokenId, T::AccountId),
		//     f: impl FnOnce(&mut Option<AccountData<T::Balance>>) -> Result<R, E>,
		// ) -> Result<R, E> {
		//     SystemAccount::<T>::try_mutate_exists(k, |maybe_value| {
		//         let existed = maybe_value.is_some();
		//         let (maybe_prefix, mut maybe_data) = split_inner(maybe_value.take(), |account| {
		//             ((account.nonce, account.refcount), account.data)
		//         });
		//         f(&mut maybe_data).map(|result| {
		//             *maybe_value = maybe_data.map(|data| {
		//                 let (nonce, refcount) = maybe_prefix.unwrap_or_default();
		//                 AccountInfo {
		//                     nonce,
		//                     refcount,
		//                     data,
		//                 }
		//             });
		//             (existed, maybe_value.is_some(), result)
		//         })
		//     })
		//         .map(|(existed, exists, v)| {
		//             if !existed && exists {
		//                 Self::on_created_account(k.clone());
		//             } else if existed && !exists {
		//                 // TODO:
		//                 //Self::on_killed_account(k.clone());
		//             }
		//             v
		//         })
		// }

		// fn post_mutation(
		//     _who: &T::AccountId,
		//     new: AccountData<<T as Config>::Balance>,
		// ) -> Option<AccountData<<T as Config>::Balance>> {
		//     let total = new.total();
		//     if total < T::ExistentialDeposit::get() {
		//         // TODO:
		//         /*
		//     if !total.is_zero() {
		//         T::DustRemoval::on_unbalanced(NegativeImbalance::new(total));
		//         Self::deposit_event(Event::DustLost(who.clone(), total));
		//     }
		//     */
		//         None
		//     } else {
		//         Some(new)
		//     }
		// }

		// fn account(token_id: T::RealisTokenId, who: &T::AccountId) -> AccountData<T::Balance> {
		//     Self::get(&(token_id, who.clone()))
		// }

		pub fn on_created_account(who: (T::RealisTokenId, T::AccountId)) {
			<T as Config>::OnNewAccount::on_new_account(&who);
			Self::deposit_event(Event::NewAccount(who.1, who.0));
		}

		// pub fn get(k: &(T::RealisTokenId, T::AccountId)) -> AccountData<T::Balance> {
		//     SystemAccount::<T>::get().data
		// }

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
