#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::pallet_prelude::{Decode, Encode};
use primitive_types::U256;
use sp_std::vec::Vec;
#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

pub use evm::backend::{Basic as Account, Log};

pub type TokenId = U256;
pub type Basic = u8;

#[derive(Encode, Decode, Clone, Eq, PartialEq, PartialOrd, Ord, Debug, Copy)]
pub struct Token {
    pub id: TokenId,
    pub token_type: TokenType,
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, PartialOrd, Ord, Debug, Copy)]
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
