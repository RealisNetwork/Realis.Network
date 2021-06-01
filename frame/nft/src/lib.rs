#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// https://substrate.dev/docs/en/knowledgebase/runtime/frame

use frame_support::{decl_module, decl_storage, decl_event, decl_error, ensure, dispatch, traits::{
    ExistenceRequirement, ExistenceRequirement::AllowDeath, StoredMap, WithdrawReasons, OnNewAccount, Get,
}, Parameter};
use sp_runtime::{traits::{AtLeast32BitUnsigned, Bounded, CheckedAdd, CheckedSub, Member, Saturating, StaticLookup,
                          StoredMapError, Zero,}, RuntimeDebug};
use frame_system::{ensure_signed, split_inner, RefCount, ensure_root};
use pallet_balances;
use primitive_types::U256;
// use std::collections::HashSet;
use codec::{Decode, Encode, EncodeLike};
use sp_std::prelude::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

pub type TokenId = U256;

#[derive(Encode, Decode, Clone, Eq, PartialEq, PartialOrd, Ord, Debug, Copy)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Mythical,
    Legendary,
}

// impl Default for Rarity {
//     fn default() -> Self { Rarity::Common }
// }

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
        if r == WithdrawReasons::from(WithdrawReasons::TRANSACTION_PAYMENT) {
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
    /// The total balance in this account including any that is reserved and ignoring any frozen.
    fn total(&self) -> Balance {
        self.free.saturating_add(self.reserved)
    }
}

/// Configure the pallet by specifying the parameters and types on which it depends.
pub trait Config: frame_system::Config {
    /// Because this pallet emits events, it depends on the runtime's definition of an event.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Config>::Event>;
    // type TokenId;
    type Balance: Member + Parameter + AtLeast32BitUnsigned + Default + Copy;

    type OnNewAccount: OnNewAccount<(Self::RealisTokenId, Self::AccountId)>;

    type ExistentialDeposit: Get<Self::Balance>;

    type RealisTokenId: Parameter + AtLeast32BitUnsigned + Default + Copy;

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
        MaxTokenId get(fn max_realis_token_id): T::RealisTokenId = 17u32.into();
        MinTokenId get(fn min_realis_token_id): T::RealisTokenId = 1u32.into();
		TokensForAccount get(fn tokens_of_owner_by_index): map hasher(opaque_blake2_256) T::AccountId => Vec<Token>;
        AccountForToken get(fn account_for_token): map hasher(opaque_blake2_256) TokenId => T::AccountId;
        TotalForAccount get(fn total_for_account): map hasher(blake2_128_concat) T::AccountId => u32;
        AllTokensInAccount get(fn all_tokens_in_account): map hasher(opaque_blake2_256) TokenId => Option<Token>;
        NftMasters get(fn nft_masters) config(): Vec<T::AccountId>;
        pub SystemAccount get(fn system_account):
            map hasher(blake2_128_concat) (T::RealisTokenId, T::AccountId) => AccountInfo<T::Index, AccountData<<T as Config>::Balance>>;
	}
}

// Pallets use events to inform users when important changes are made.
// https://substrate.dev/docs/en/knowledgebase/runtime/events
decl_event!(
	pub enum Event<T> where
    AccountId = <T as frame_system::Config>::AccountId,
    TokenBalance = <T as Config>::Balance,
    RealisTokenId = <T as Config>::RealisTokenId, {
		/// Event documentation should end with an array that provides descriptive names for event
		/// parameters. [something, who]
		SomethingStored(TokenId, AccountId),
		TokenMinted(AccountId, TokenId),
		TokenBurned(),
        BasicTokenBurned(TokenId),
		TokenTransferred(TokenId, AccountId),
        TokenBreeded(TokenId),
                /// An account was created with some free balance. \[account, free_balance\]
                Endowed(AccountId, RealisTokenId, TokenBalance),
                /// Some assets were transferred. \[token_id, from, to, amount\]
                Transfer(AccountId, AccountId, RealisTokenId, TokenBalance),
                /// A balance was set by root. \[who, free, reserved\]
                RealisTokenBalanceSet(
                    AccountId,
                    TokenId,
                    TokenBalance,
                    TokenBalance,
                ),
                /// Some amount was deposited (e.g. for transaction fees). \[who, deposit\]
                Deposit(AccountId, TokenId, TokenBalance),
                /// Some balance was reserved (moved from free to reserved). \[who, value\]
                Reserved(AccountId, TokenId, TokenBalance),
                /// Some balance was unreserved (moved from reserved to free). \[who, value\]
                Unreserved(AccountId, TokenId, TokenBalance),
                /// A new \[account\] was created.
                NewAccount(AccountId, RealisTokenId),
                TokenCreated(TokenId),
                /// Some assets were issued. [token_id, owner, total_supply]
                Issued(TokenId, AccountId, TokenBalance),
        // TokensTransferred(TokenId, AccountId, TokenId, AccountId),
	}
);

// Errors inform users that something went wrong.
decl_error! {
	pub enum Error for Module<T: Config> {
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
		#[weight = 10_000]
		pub fn mint(origin, target_account: T::AccountId,
		token_id: TokenId,
        rarity: Rarity,
        socket: Socket,
        params: Params,
        ) -> dispatch::DispatchResult {
            let who = ensure_signed(origin)?;
		    ensure!(
                Self::nft_masters().contains(&who),
                Error::<T>::NotNftMaster
            );

		    let token_info: Vec<Token> = sp_std::vec![Token {
               token_id,
               rarity,
               socket,
               params
            }];

            let token = Token {
               token_id,
               rarity,
               socket,
               params
            };
            Self::mint_nft(&target_account, token_info, token_id, token)?;
		    Self::deposit_event(RawEvent::TokenMinted(target_account.clone(), token_id));
            Ok(())

		}

        #[weight = 10_000]
		pub fn mint_basic(origin, target_account: T::AccountId, token_id: TokenId) -> dispatch::DispatchResult {
            let who = ensure_signed(origin)?;
		    ensure!(
                Self::nft_masters().contains(&who),
                Error::<T>::NotNftMaster
            );

            Self::mint_basic_nft(&target_account, token_id)?;
		    Self::deposit_event(RawEvent::TokenMinted(target_account.clone(), token_id));
            Ok(())

		}

		///Burn token(only owner)
		#[weight = 10_000]
		pub fn burn(origin, token_id: TokenId) -> dispatch::DispatchResult {
            let who = ensure_signed(origin)?;
            // ensure!(
            //     who != T::AccountId::default(),
            //     Error::<T>::NonExistentToken
            // );

            ensure!(
                who == Self::account_for_token(&token_id),
                Error::<T>::NotTokenOwner
            );

			let id_of_token = Self::burn_nft(token_id)?;
		    Self::deposit_event(RawEvent::TokenBurned());
            Ok(())
        }

        #[weight = 10_000]
		pub fn burn_basic(origin, token_id: TokenId) -> dispatch::DispatchResult {
            let who = ensure_signed(origin)?;

            // ensure!(
            //     who != T::AccountId::default(),
            //     Error::<T>::NonExistentToken
            // );

            ensure!(
                who == Self::account_for_token(&token_id),
                Error::<T>::NotTokenOwner
            );

			let id_of_token = Self::burn_basic_nft(token_id)?;
		    Self::deposit_event(RawEvent::TokenBurned());
            Ok(())
        }

        ///Transfer token(only owner)
        #[weight = 10_000]
        pub fn transfer(origin, dest_account: T::AccountId, token_id: TokenId) -> dispatch::DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(who == Self::account_for_token(&token_id), Error::<T>::NotTokenOwner);

            Self::transfer_nft(&dest_account, token_id)?;
            Self::deposit_event(RawEvent::TokenTransferred(token_id.clone(), dest_account.clone()));
            Ok(())
        }

        #[weight = 10_000]
        pub fn transfer_basic(origin, dest_account: T::AccountId, token_id: TokenId) -> dispatch::DispatchResult {
            let who = ensure_signed(origin)?;
            ensure!(who == Self::account_for_token(&token_id), Error::<T>::NotTokenOwner);

            Self::transfer_basic_nft(&dest_account, token_id)?;
            Self::deposit_event(RawEvent::TokenTransferred(token_id.clone(), dest_account.clone()));
            Ok(())
        }

    }
}

impl<T: Config> Module<T> {
    pub fn mint_nft(target_account: &T::AccountId, token_info: Vec<Token>, token_id: TokenId, token: Token) -> dispatch::result::Result<TokenId, dispatch::DispatchError> {
        // fn mint(target_account: &T::AccountId, token_id: Self::TokenId) -> dispatch::result::Result<Self::TokenId, _> {
        ensure!(
                !AccountForToken::<T>::contains_key(token_id),
                 Error::<T>::TokenExist
                 );
        TokensForAccount::<T>::mutate(target_account, |token_info| token_info.push(token));
        // hash_set_of_tokens.insert(token_id)
        TotalForAccount::<T>::mutate(&target_account, |total| *total += 1);
        AccountForToken::<T>::insert(token_id, &target_account);
        // Self::deposit_event(RawEvent::TokenMinted(target_account, token_id));
        Ok(token_id)
    }

    pub fn mint_basic_nft(target_account: &T::AccountId, token_id: TokenId) -> dispatch::result::Result<TokenId, dispatch::DispatchError> {
        // fn mint(target_account: &T::AccountId, token_id: Self::TokenId) -> dispatch::result::Result<Self::TokenId, _> {
        ensure!(
                !AccountForToken::<T>::contains_key(token_id),
                 Error::<T>::TokenExist
                 );

        // hash_set_of_tokens.insert(token_id);
        TotalForAccount::<T>::mutate(&target_account, |total| *total += 1);
        AccountForToken::<T>::insert(token_id, &target_account);
        // Self::deposit_event(RawEvent::TokenMinted(target_account, token_id));
        Ok(token_id)
    }

    pub fn burn_nft(token_id: TokenId) -> dispatch::DispatchResult {
        let owner = Self::owner_of(token_id);


        TotalForAccount::<T>::mutate(&owner, |total| *total -= 1);

        // TokensForAccount::<T>::mutate(&owner, |token_id| token_id.burn(&token_id));
        TokensForAccount::<T>::take(&owner);
        AccountForToken::<T>::remove(&token_id);

        Ok(())
    }

    pub fn burn_basic_nft(token_id: TokenId) -> dispatch::result::Result<Vec<Token>, dispatch::DispatchError> {
        let owner = Self::owner_of(token_id);


        TotalForAccount::<T>::mutate(&owner, |total| *total -= 1);

        let deleted_token = TokensForAccount::<T>::take(&owner);
        // TokensForAccount::<T>::mutate(&owner, &token_id, |tokens| {
        //     let pos = tokens
        //         .binary_search(&token_id)
        //         .expect("We already checked that we have the correct owner; qed");
        //     tokens.remove(pos);
        // });
        AccountForToken::<T>::remove(&token_id);

        Ok(deleted_token)
    }


    fn transfer_nft(dest_account: &T::AccountId, token_id: TokenId) -> dispatch::DispatchResult
    {
        let owner = Self::owner_of(token_id);
        ensure!(
                owner != T::AccountId::default(),
                Error::<T>::NonExistentToken
            );

        TotalForAccount::<T>::mutate(&owner, |total| *total -= 1);
        TotalForAccount::<T>::mutate(dest_account, |total| *total += 1);
        AccountForToken::<T>::remove(token_id);

        let transferred_token = TokensForAccount::<T>::take(&owner);

        TokensForAccount::<T>::insert(dest_account, transferred_token);
        AccountForToken::<T>::insert(token_id, &dest_account);

        Ok(())
    }

    pub fn transfer_basic_nft(dest_account: &T::AccountId, token_id: TokenId) -> dispatch::DispatchResult
    {
        let owner = Self::owner_of(token_id);
        ensure!(
                owner != T::AccountId::default(),
                Error::<T>::NonExistentToken
            );

        TotalForAccount::<T>::mutate(&owner, |total| *total -= 1);
        TotalForAccount::<T>::mutate(dest_account, |total| *total += 1);
        AccountForToken::<T>::remove(token_id);

        let transferred_token = AccountForToken::<T>::take(token_id);

        AccountForToken::<T>::insert(token_id, &dest_account);

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

    pub fn do_transfer(
        transactor: &T::AccountId,
        dest: &T::AccountId,
        token_id: T::RealisTokenId,
        value: <T as Config>::Balance,
        existence_requirement: ExistenceRequirement,
    ) -> dispatch::DispatchResult {
        if value.is_zero() || transactor == dest {
            return Ok(());
        }

        Self::try_mutate_account(dest, token_id, |to_account, _| -> dispatch::DispatchResult {
            Self::try_mutate_account(transactor, token_id, |from_account, _| -> dispatch::DispatchResult {
                from_account.free = from_account
                    .free
                    .checked_sub(&value)
                    .ok_or(Error::<T>::InsufficientBalance)?;

                // NOTE: total stake being stored in the same type means that this could never overflow
                // but better to be safe than sorry.
                to_account.free = to_account
                    .free
                    .checked_add(&value)
                    .ok_or(Error::<T>::Overflow)?;

                let ed = T::ExistentialDeposit::get();
                ensure!(to_account.total() >= ed, Error::<T>::ExistentialDeposit);

                Self::ensure_can_withdraw(
                    transactor,
                    token_id,
                    value,
                    WithdrawReasons::TRANSFER,
                    from_account.free,
                )?;

                let allow_death = existence_requirement == ExistenceRequirement::AllowDeath;
                let allow_death = allow_death && frame_system::Module::<T>::allow_death(transactor);
                ensure!(
                    allow_death || from_account.free >= ed,
                    Error::<T>::KeepAlive
                );

                Ok(())
            })
        })?;

        // Emit transfer event.
        Self::deposit_event(RawEvent::Transfer(
            transactor.clone(),
            dest.clone(),
            token_id,
            value,
        ));

        Ok(())
    }

    fn try_mutate_account<R, E: From<StoredMapError>>(
        who: &T::AccountId,
        token_id: T::RealisTokenId,
        f: impl FnOnce(&mut AccountData<<T as Config>::Balance>, bool) -> Result<R, E>,
    ) -> Result<R, E> {
        Self::try_mutate_exists(&(token_id, who.clone()), |maybe_account| {
            let is_new = maybe_account.is_none();
            let mut account = maybe_account.take().unwrap_or_default();
            f(&mut account, is_new).map(move |result| {
                let maybe_endowed = if is_new { Some(account.free) } else { None };
                *maybe_account = Self::post_mutation(who, account);
                (maybe_endowed, result)
            })
        })
            .map(|(maybe_endowed, result)| {
                if let Some(endowed) = maybe_endowed {
                    Self::deposit_event(RawEvent::Endowed(who.clone(), token_id, endowed));
                }
                result
            })
    }

    fn ensure_can_withdraw(
        who: &T::AccountId,
        token_id: T::RealisTokenId,
        amount: T::Balance,
        reasons: WithdrawReasons,
        new_balance: T::Balance,
    ) -> dispatch::DispatchResult {
        if amount.is_zero() {
            return Ok(());
        }
        let min_balance = Self::account(token_id, who).frozen(reasons.into());
        ensure!(
            new_balance >= min_balance,
            Error::<T>::LiquidityRestrictions
        );
        Ok(())
    }

    fn try_mutate_exists<R, E: From<StoredMapError>>(
        k: &(T::RealisTokenId, T::AccountId),
        f: impl FnOnce(&mut Option<AccountData<T::Balance>>) -> Result<R, E>,
    ) -> Result<R, E> {
        SystemAccount::<T>::try_mutate_exists(k, |maybe_value| {
            let existed = maybe_value.is_some();
            let (maybe_prefix, mut maybe_data) = split_inner(maybe_value.take(), |account| {
                ((account.nonce, account.refcount), account.data)
            });
            f(&mut maybe_data).map(|result| {
                *maybe_value = maybe_data.map(|data| {
                    let (nonce, refcount) = maybe_prefix.unwrap_or_default();
                    AccountInfo {
                        nonce,
                        refcount,
                        data,
                    }
                });
                (existed, maybe_value.is_some(), result)
            })
        })
            .map(|(existed, exists, v)| {
                if !existed && exists {
                    Self::on_created_account(k.clone());
                } else if existed && !exists {
                    // TODO:
                    //Self::on_killed_account(k.clone());
                }
                v
            })
    }

    fn post_mutation(
        _who: &T::AccountId,
        new: AccountData<<T as Config>::Balance>,
    ) -> Option<AccountData<<T as Config>::Balance>> {
        let total = new.total();
        if total < T::ExistentialDeposit::get() {
            // TODO:
            /*
            if !total.is_zero() {
                T::DustRemoval::on_unbalanced(NegativeImbalance::new(total));
                Self::deposit_event(RawEvent::DustLost(who.clone(), total));
            }
            */
            None
        } else {
            Some(new)
        }
    }

    fn account(token_id: T::RealisTokenId, who: &T::AccountId) -> AccountData<T::Balance> {
        Self::get(&(token_id, who.clone()))
    }

    pub fn on_created_account(who: (T::RealisTokenId, T::AccountId)) {
        <T as Config>::OnNewAccount::on_new_account(&who);
        Self::deposit_event(RawEvent::NewAccount(who.1, who.0));
    }

    fn owner_of(token_id: TokenId) -> T::AccountId {
        Self::account_for_token(token_id)
    }

    pub fn get(k: &(T::RealisTokenId, T::AccountId)) -> AccountData<T::Balance> {
        SystemAccount::<T>::get(k).data
    }

}

#[cfg(test)]
mod test {

    #[test]
    fn mint_nft_if_possible() {
        assert_eq!(super::mint_nft(&1, &1, 222, Common, Head, {strength: 2, agility: 3, intelligence: 4}));
    }
}