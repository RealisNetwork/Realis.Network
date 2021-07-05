#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::pallet_prelude::{Decode, Encode};
use primitive_types::U256;

pub type TokenId = U256;
pub type Basic = u8;

#[derive(Encode, Decode, Clone, Eq, PartialEq, PartialOrd, Ord, Debug, Copy)]
pub struct Token {
    pub id: TokenId,
    pub token_type: Type,
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
pub enum Type {
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
