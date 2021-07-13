#![cfg_attr(not(feature = "std"), no_std)]

use evm::ExitReason;
use evm::{executor::PrecompileOutput, Context, ExitError, ExitSucceed};
use frame_support::pallet_prelude::{Decode, Encode};
use impl_trait_for_tuples::impl_for_tuples;
use primitive_types::U256;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_core::H160;
use sp_std::vec::Vec;

pub use evm::backend::{Basic as Account, Log};

pub type TokenId = U256;
pub type Basic = u8;

#[derive(Encode, Decode, Clone, Eq, PartialEq, PartialOrd, Ord, Debug, Copy)]
pub struct Token {
    pub id: TokenId,
    pub token_type: TokenType,
    // market_type: 	MarketType
}

// #[derive(Encode, Decode, Clone, Eq, PartialEq, PartialOrd, Ord, Debug, Copy)]
// pub enum MarketType {
// 	Tradeable 	(TradeStatus),
// 	Untradeable
// }

// #[derive(Encode, Decode, Clone, Eq, PartialEq, PartialOrd, Ord, Debug, Copy)]
// pub enum TradeStatus {
// 	ForSale,
// 	OnHands,
// }

#[derive(Encode, Decode, Clone, Eq, PartialEq, PartialOrd, Ord, Debug, Copy)]
pub enum TokenType {
    // Skin 		,
    Mergeable(Mergeable),
    Stackable(Stackable),
    Basic(Basic),
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, PartialOrd, Ord, Debug, Copy)]
pub struct Mergeable {
    pub rarity: Rarity,
    pub socket: Socket,
    pub params: Params,
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, PartialOrd, Ord, Debug, Copy)]
pub enum Stackable {
    Silver,
    Gold,
    Diamond,
}

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

#[derive(Encode, Decode, Clone, Eq, PartialEq, PartialOrd, Ord, Debug, Copy)]
pub struct Params {
    pub strength: u8,
    pub agility: u8,
    pub intelligence: u8,
}

#[derive(Clone, Eq, PartialEq, Encode, Decode, Default)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
/// External input from the transaction.
pub struct Vicinity {
    /// Current transaction gas price.
    pub gas_price: U256,
    /// Origin of the transaction.
    pub origin: H160,
}

#[derive(Clone, Eq, PartialEq, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub struct ExecutionInfo<T> {
    pub exit_reason: ExitReason,
    pub value: T,
    pub used_gas: U256,
    pub logs: Vec<Log>,
}

pub type CallInfo = ExecutionInfo<Vec<u8>>;
pub type CreateInfo = ExecutionInfo<H160>;

#[derive(Clone, Eq, PartialEq, Encode, Decode)]
#[cfg_attr(feature = "std", derive(Debug, Serialize, Deserialize))]
pub enum CallOrCreateInfo {
    Call(CallInfo),
    Create(CreateInfo),
}

pub trait PrecompileSet {
    /// Try to execute the code address as precompile. If the code address is not
    /// a precompile or the precompile is not yet available, return `None`.
    /// Otherwise, calculate the amount of gas needed with given `input` and
    /// `target_gas`. Return `Some(Ok(status, output, gas_used))` if the execution
    /// is successful. Otherwise return `Some(Err(_))`.
    fn execute(
        address: H160,
        input: &[u8],
        target_gas: Option<u64>,
        context: &Context,
    ) -> Option<core::result::Result<PrecompileOutput, ExitError>>;
}

/// One single precompile used by EVM engine.
pub trait Precompile {
    /// Try to execute the precompile. Calculate the amount of gas needed with given `input` and
    /// `target_gas`. Return `Ok(status, output, gas_used)` if the execution is
    /// successful. Otherwise return `Err(_)`.
    fn execute(
        input: &[u8],
        target_gas: Option<u64>,
        context: &Context,
    ) -> core::result::Result<PrecompileOutput, ExitError>;
}

#[impl_for_tuples(16)]
#[tuple_types_no_default_trait_bound]
impl PrecompileSet for Tuple {
    for_tuples!( where #( Tuple: Precompile )* );

    fn execute(
        address: H160,
        input: &[u8],
        target_gas: Option<u64>,
        context: &Context,
    ) -> Option<core::result::Result<PrecompileOutput, ExitError>> {
        let mut index = 0;

        for_tuples!( #(
			index += 1;
			if address == H160::from_low_u64_be(index) {
				return Some(Tuple::execute(input, target_gas, context))
			}
		)* );

        None
    }
}

pub trait LinearCostPrecompile {
    const BASE: u64;
    const WORD: u64;

    fn execute(input: &[u8], cost: u64) -> core::result::Result<(ExitSucceed, Vec<u8>), ExitError>;
}

impl<T: LinearCostPrecompile> Precompile for T {
    fn execute(
        input: &[u8],
        target_gas: Option<u64>,
        _: &Context,
    ) -> core::result::Result<PrecompileOutput, ExitError> {
        let cost = ensure_linear_cost(target_gas, input.len() as u64, T::BASE, T::WORD)?;

        let (exit_status, output) = T::execute(input, cost)?;
        Ok(PrecompileOutput {
            exit_status,
            cost,
            output,
            logs: Default::default(),
        })
    }
}

/// Linear gas cost
fn ensure_linear_cost(
    target_gas: Option<u64>,
    len: u64,
    base: u64,
    word: u64,
) -> Result<u64, ExitError> {
    let cost = base
        .checked_add(
            word.checked_mul(len.saturating_add(31) / 32)
                .ok_or(ExitError::OutOfGas)?,
        )
        .ok_or(ExitError::OutOfGas)?;

    if let Some(target_gas) = target_gas {
        if cost > target_gas {
            return Err(ExitError::OutOfGas);
        }
    }

    Ok(cost)
}

#[derive(PartialEq, Eq, Clone, Default, Encode, Decode)]
pub struct OpaqueExtrinsic(Vec<u8>);

impl OpaqueExtrinsic {
    /// Convert an encoded extrinsic to an `OpaqueExtrinsic`.
    pub fn from_bytes(mut bytes: &[u8]) -> Result<Self, codec::Error> {
        OpaqueExtrinsic::decode(&mut bytes)
    }
}