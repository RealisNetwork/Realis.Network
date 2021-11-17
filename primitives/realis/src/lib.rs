#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::pallet_prelude::{Decode, Encode};
use primitive_types::U256;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_std::fmt::{Display, Formatter};
use sp_std::vec::Vec;

pub type TokenId = U256;
pub type String = Vec<u8>;

#[derive(Encode, Decode, Clone, Eq, PartialEq, PartialOrd, Ord, Debug)]
pub struct Token {
    pub id: TokenId,
    pub token_type: TokenType,
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, PartialOrd, Ord, Debug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum TokenType {
    Basic(Rarity, String, u32, String),
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, PartialOrd, Ord, Debug, Copy)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum Status {
    OnSell,
    OnDelegateSell,
    InDelegation,
    Free,
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, PartialOrd, Ord, Debug, Copy)]
pub enum Stackable {
    Silver,
    Gold,
    Diamond,
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, PartialOrd, Ord, Debug, Copy)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Epic,
    Legendary,
    Relic,
}

impl Display for Rarity {
    fn fmt(&self, f: &mut Formatter<'_>) -> sp_std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
