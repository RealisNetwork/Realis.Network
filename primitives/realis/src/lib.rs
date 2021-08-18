#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::pallet_prelude::{Decode, Encode};
use primitive_types::U256;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};
use sp_std::vec::Vec;

use sp_std::str::FromStr;

pub type TokenId = U256;
pub type Basic = u8;

#[derive(Encode, Decode, Clone, Eq, PartialEq, PartialOrd, Ord, Debug, Copy)]
pub struct Token {
    pub id: TokenId,
    pub token_type: TokenType,
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, PartialOrd, Ord, Debug, Copy)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum TokenType {
    Basic(Basic, Rarity),
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
    Mythical,
    Legendary,
}

#[derive(PartialEq, Eq, Clone, Default, Encode, Decode)]
pub struct OpaqueExtrinsic(Vec<u8>);

impl OpaqueExtrinsic {
    /// Convert an encoded extrinsic to an `OpaqueExtrinsic`.
    pub fn from_bytes(mut bytes: &[u8]) -> Result<Self, codec::Error> {
        OpaqueExtrinsic::decode(&mut bytes)
    }
}

impl FromStr for Rarity {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Common" => Ok(Rarity::Common),
            "Uncommon" => Ok(Rarity::Uncommon),
            "Rare" => Ok(Rarity::Rare),
            "Mythical" => Ok(Rarity::Mythical),
            "Legendary" => Ok(Rarity::Legendary),
            _ => Err(()),
        }
    }
}
