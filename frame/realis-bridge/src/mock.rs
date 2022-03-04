#![cfg(test)]

use crate::*;
use frame_support::{parameter_types, PalletId};
use sp_core::H256;
use sp_runtime::traits::BlakeTwo256;
use sp_runtime::{testing::Header, traits::IdentityLookup};

use crate::{self as realis_bridge, Config};
pub use pallet_balances as balances;

pub type Block = sp_runtime::generic::Block<Header, UncheckedExtrinsic>;
pub type UncheckedExtrinsic = sp_runtime::generic::UncheckedExtrinsic<u32, u64, Call, ()>;

frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic
    {
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
        Nft: pallet_nft::{Pallet, Call, Storage, Event<T>},
        RealisBridge: realis_bridge::{Pallet, Call, Event<T>},
    }
);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub BlockWeights: frame_system::limits::BlockWeights =
        frame_system::limits::BlockWeights::simple_max(
            frame_support::weights::constants::WEIGHT_PER_SECOND * 2
        );
    pub const MaxLocks: u32 = 1024;
    pub static ExistentialDeposit: u128 = 1;
    pub static Period: u64 = 5;
    pub static Offset: u64 = 0;
}

impl frame_system::Config for Test {
    type BaseCallFilter = frame_support::traits::AllowAll;
    type BlockWeights = ();
    type BlockLength = ();
    type Origin = Origin;
    type Call = Call;
    type Index = u64;
    type BlockNumber = u64;
    type Hash = H256;
    type Hashing = BlakeTwo256;
    type AccountId = u64;
    type Lookup = IdentityLookup<Self::AccountId>;
    type Header = Header;
    type Event = Event;
    type BlockHashCount = BlockHashCount;
    type DbWeight = ();
    type Version = ();
    type PalletInfo = PalletInfo;
    type AccountData = pallet_balances::AccountData<u128>;
    type OnNewAccount = ();
    type OnKilledAccount = ();
    type SystemWeightInfo = ();
    type SS58Prefix = ();
    type OnSetCode = ();
}

impl pallet_balances::Config for Test {
    type MaxLocks = MaxLocks;
    type MaxReserves = ();
    type ReserveIdentifier = [u8; 8];
    type Balance = u128;
    type Event = Event;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = ();
}

parameter_types! {
    pub const ExistentialDepositOfRealisTokens: u64 = 1;
}

impl pallet_nft::Config for Test {
    type Event = Event;
    type Balance = u128;
    type WeightInfo = pallet_nft::weights::SubstrateWeight<Test>;
}

parameter_types! {
    pub const RealisBridgePalletId: PalletId = PalletId(*b"rl/relbr");
}

impl Config for Test {
    type Event = Event;
    type BridgeCurrency = pallet_balances::Pallet<Test>;
    type PalletId = RealisBridgePalletId;
}
