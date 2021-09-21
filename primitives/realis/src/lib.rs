#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::pallet_prelude::{Decode, Encode};
use serde_repr::{Serialize_repr, Deserialize_repr};
use sp_std::fmt::{Display, Formatter};
use primitive_types::U256;
use sp_std::vec::Vec;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_std::str::FromStr;

pub type TokenId = U256;
pub type Basic = u8;
pub type String = Vec<u8>;

#[derive(Encode, Decode, Clone, Eq, PartialEq, PartialOrd, Ord, Debug)]
pub struct Token {
    pub id: TokenId,
    pub token_type: TokenType,
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, PartialOrd, Ord, Debug)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum TokenType {
    Basic(Basic, Rarity, String),
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, PartialOrd, Ord, Debug, Copy)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum Status {
    OnSell,
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
#[cfg_attr(feature = "std", derive(Serialize_repr, Deserialize_repr))]
#[repr(u8)]
pub enum Rarity {
    Common = 1,
    Uncommon = 2,
    Rare = 3,
    Epic = 4,
    Legendary = 5,
    Relic = 6,
}

impl FromStr for Rarity {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Common" => Ok(Rarity::Common),
            "Uncommon" => Ok(Rarity::Uncommon),
            "Rare" => Ok(Rarity::Rare),
            "Mythical" => Ok(Rarity::Epic),
            "Legendary" => Ok(Rarity::Legendary),
            "Relic" => Ok(Rarity::Relic),
            _ => Err(()),
        }
    }
}

impl Display for Rarity {
    fn fmt(&self, f: &mut Formatter<'_>) -> sp_std::fmt::Result {
        write!(f, "{:?}", self)
    }
}
