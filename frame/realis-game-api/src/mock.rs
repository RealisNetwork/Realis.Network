// This file is part of Substrate.

// Copyright (C) 2018-2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Test utilities

use crate as realis_game_api;
use crate::*;
use frame_election_provider_support::onchain;
use frame_support::pallet_prelude::GenesisBuild;
use frame_support::{parameter_types, traits::OneSessionHandler, PalletId};
use pallet_staking::EraIndex;
use sp_core::H256;
use sp_io;
use sp_runtime::traits::BlakeTwo256;
use sp_runtime::{
    curve::PiecewiseLinear,
    testing::{Header, TestXt, UintAuthorityId},
    traits::IdentityLookup,
    Perbill,
};
use sp_staking::SessionIndex;
use std::{cell::RefCell, collections::HashSet};

thread_local! {
    static SESSION: RefCell<(Vec<u64>, HashSet<u64>)> = RefCell::new(Default::default());
}

/// Another session handler struct to test on_disabled.
pub struct OtherSessionHandler;
impl OneSessionHandler<u64> for OtherSessionHandler {
    type Key = UintAuthorityId;

    fn on_genesis_session<'a, I: 'a>(_: I)
    where
        I: Iterator<Item = (&'a u64, Self::Key)>,
        u64: 'a,
    {
    }

    fn on_new_session<'a, I: 'a>(_: bool, validators: I, _: I)
    where
        I: Iterator<Item = (&'a u64, Self::Key)>,
        u64: 'a,
    {
        SESSION.with(|x| {
            *x.borrow_mut() = (validators.map(|x| x.0.clone()).collect(), HashSet::new())
        });
    }

    #[warn(dead_code)]
    fn on_disabled(validator_index: usize) {
        SESSION.with(|d| {
            let mut d = d.borrow_mut();
            let value = d.0[validator_index];
            d.1.insert(value);
        })
    }
}

impl sp_runtime::BoundToRuntimeAppPublic for OtherSessionHandler {
    type Public = UintAuthorityId;
}

pub fn is_disabled(controller: u64) -> bool {
    let stash = Staking::ledger(&controller).unwrap().stash;
    SESSION.with(|d| d.borrow().1.contains(&stash))
}

type UncheckedExtrinsic = frame_system::mocking::MockUncheckedExtrinsic<Test>;
type Block = frame_system::mocking::MockBlock<Test>;

frame_support::construct_runtime!(
    pub enum Test where
        Block = Block,
        NodeBlock = Block,
        UncheckedExtrinsic = UncheckedExtrinsic,
    {
        System: frame_system::{Pallet, Call, Config, Storage, Event<T>},
        Authorship: pallet_authorship::{Pallet, Call, Storage, Inherent},
        Timestamp: pallet_timestamp::{Pallet, Call, Storage, Inherent},
        Balances: pallet_balances::{Pallet, Call, Storage, Config<T>, Event<T>},
        Staking: pallet_staking::{Pallet, Call, Config<T>, Storage, Event<T>},
        Session: pallet_session::{Pallet, Call, Storage, Event, Config<T>},
        NFT: pallet_nft::{Pallet, Call, Storage, Event<T>},
        RealisGameApi: realis_game_api::{Pallet, Call, Event<T>},
    }
);

parameter_types! {
    pub const BlockHashCount: u64 = 250;
    pub BlockWeights: frame_system::limits::BlockWeights =
        frame_system::limits::BlockWeights::simple_max(
            frame_support::weights::constants::WEIGHT_PER_SECOND * 2
        );
    pub const MaxLocks: u32 = 1024;
    pub static SessionsPerEra: SessionIndex = 3;
    pub static ExistentialDeposit: u128 = 1;
    pub static SlashDeferDuration: EraIndex = 0;
    pub static Period: u64 = 5;
    pub static Offset: u64 = 0;
}

impl frame_system::Config for Test {
    type BaseCallFilter = ();
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
    type MaxLocks = ();
    type Balance = u128;
    type Event = Event;
    type DustRemoval = ();
    type ExistentialDeposit = ExistentialDeposit;
    type AccountStore = System;
    type WeightInfo = ();
}
parameter_types! {
    pub const UncleGenerations: u64 = 0;
    pub const DisabledValidatorsThreshold: Perbill = Perbill::from_percent(25);
}
sp_runtime::impl_opaque_keys! {
    pub struct SessionKeys {
        pub other: OtherSessionHandler,
    }
}
impl pallet_session::Config for Test {
    type SessionManager = pallet_session::historical::NoteHistoricalRoot<Test, Staking>;
    type Keys = SessionKeys;
    type ShouldEndSession = pallet_session::PeriodicSessions<Period, Offset>;
    type SessionHandler = (OtherSessionHandler,);
    type Event = Event;
    type ValidatorId = u64;
    type ValidatorIdOf = pallet_staking::StashOf<Test>;
    type DisabledValidatorsThreshold = DisabledValidatorsThreshold;
    type NextSessionRotation = pallet_session::PeriodicSessions<Period, Offset>;
    type WeightInfo = ();
}

impl pallet_session::historical::Config for Test {
    type FullIdentification = pallet_staking::Exposure<u64, u128>;
    type FullIdentificationOf = pallet_staking::ExposureOf<Test>;
}
impl pallet_authorship::Config for Test {
    type FindAuthor = ();
    type UncleGenerations = UncleGenerations;
    type FilterUncle = ();
    type EventHandler = ();
}
parameter_types! {
    pub const MinimumPeriod: u64 = 5;
}
impl pallet_timestamp::Config for Test {
    type Moment = u64;
    type OnTimestampSet = ();
    type MinimumPeriod = MinimumPeriod;
    type WeightInfo = ();
}
pallet_staking_reward_curve::build! {
    const I_NPOS: PiecewiseLinear<'static> = curve!(
        min_inflation: 0_025_000,
        max_inflation: 0_100_000,
        ideal_stake: 0_500_000,
        falloff: 0_050_000,
        max_piece_count: 40,
        test_precision: 0_005_000,
    );
}
parameter_types! {
    pub const BondingDuration: EraIndex = 3;
    pub const RewardCurve: &'static PiecewiseLinear<'static> = &I_NPOS;
    pub const MaxNominatorRewardedPerValidator: u32 = 64;
    pub const StakingPalletId: PalletId = PalletId(*b"da/staki");
}

thread_local! {
    pub static REWARD_REMAINDER_UNBALANCED: RefCell<u128> = RefCell::new(0);
}

impl onchain::Config for Test {
    type AccountId = u64;
    type BlockNumber = u64;
    type BlockWeights = BlockWeights;
    type Accuracy = Perbill;
    type DataProvider = Staking;
}

impl pallet_staking::Config for Test {
    type RewardRemainder = ();
    const MAX_NOMINATIONS: u32 = 16;
    type Currency = Balances;
    type UnixTime = Timestamp;
    type PalletId = StakingPalletId;
    type CurrencyToVote = frame_support::traits::SaturatingCurrencyToVote;
    type Event = Event;
    type Slash = ();
    type Reward = ();
    type SessionsPerEra = SessionsPerEra;
    type SlashDeferDuration = SlashDeferDuration;
    type SlashCancelOrigin = frame_system::EnsureRoot<Self::AccountId>;
    type BondingDuration = BondingDuration;
    type SessionInterface = Self;
    type EraPayout = pallet_staking::ConvertCurve<RewardCurve>;
    type NextNewSession = Session;
    type MaxNominatorRewardedPerValidator = MaxNominatorRewardedPerValidator;
    type ElectionProvider = onchain::OnChainSequentialPhragmen<Self>;
    type WeightInfo = ();
}

impl<LocalCall> frame_system::offchain::SendTransactionTypes<LocalCall> for Test
where
    Call: From<LocalCall>,
{
    type OverarchingCall = Call;
    type Extrinsic = TestXt<Call, ()>;
}

parameter_types! {
    pub const ExistentialDepositOfRealisTokens: u128 = 1;
}

impl pallet_nft::Config for Test {
    type Event = Event;
    type Balance = u128;
    type ExistentialDeposit = ExistentialDepositOfRealisTokens;
    type OnNewAccount = ();
    type RealisTokenId = u32;
}

parameter_types! {
    pub const GameApiPalletId: PalletId = PalletId(*b"rl/gamap");
}

impl Config for Test {
    type Event = Event;
    type PalletId = GameApiPalletId;
    type Currency = Balances;
    type StakingPoolId = StakingPalletId;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext(nft_master: Vec<u64>) -> sp_io::TestExternalities {
    let mut t = frame_system::GenesisConfig::default()
        .build_storage::<Test>()
        .unwrap();
    pallet_nft::GenesisConfig::<Test> {
        nft_masters: nft_master,
    }
    .assimilate_storage(&mut t)
    .unwrap();
    t.into()
}
