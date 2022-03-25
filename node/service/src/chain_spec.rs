// Copyright 2017-2020 Parity Technologies (UK) Ltd.
// This file is part of Polkadot.

// Polkadot is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Polkadot is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Polkadot.  If not, see <http://www.gnu.org/licenses/>.

//! Polkadot chain configurations.

use std::str::FromStr;
use beefy_primitives::crypto::AuthorityId as BeefyId;
use grandpa::AuthorityId as GrandpaId;
use hex_literal::hex;
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
#[cfg(feature = "polkadot-native")]
use polkadot::constants::currency::UNITS as DOT;
use polkadot_primitives::v1::{AccountId, AccountPublic, AssignmentId, ValidatorId};
#[cfg(feature = "polkadot-native")]
use node_runtime as polkadot;
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_consensus_babe::AuthorityId as BabeId;

use sc_chain_spec::{ChainSpecExtension, ChainType};
use serde::{Deserialize, Serialize};
use sp_core::{sr25519, crypto::UncheckedInto, Pair, Public};
use sp_runtime::{traits::IdentifyAccount, Perbill};
use telemetry::TelemetryEndpoints;
use node_runtime::{Runtime, realis_game_api};
use crate::RealisChainSpec;

#[cfg(feature = "polkadot-native")]
const POLKADOT_STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";
const DEFAULT_PROTOCOL_ID: &str = "dot";

/// Node `ChainSpec` extensions.
///
/// Additional parameters for some Substrate core modules,
/// customizable from the chain spec.
#[derive(Default, Clone, Serialize, Deserialize, ChainSpecExtension)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
    /// Block numbers with known hashes.
    pub fork_blocks: sc_client_api::ForkBlocks<polkadot_primitives::v1::Block>,
    /// Known bad block hashes.
    pub bad_blocks: sc_client_api::BadBlocks<polkadot_primitives::v1::Block>,
    /// The light sync state.
    ///
    /// This value will be set by the `sync-state rpc` implementation.
    pub light_sync_state: sc_sync_state_rpc::LightSyncStateExtension,
}

/// The `ChainSpec` parameterized for the polkadot runtime.
#[cfg(feature = "polkadot-native")]
pub type PolkadotChainSpec = service::GenericChainSpec<polkadot::GenesisConfig, Extensions>;

// Dummy chain spec, in case when we don't have the native runtime.
pub type DummyChainSpec = service::GenericChainSpec<(), Extensions>;

/// Specialized `ChainSpec`.
pub type ChainSpec = service::GenericChainSpec<node_runtime::GenesisConfig, Extensions>;
/// Flaming Fir testnet generator
pub fn flaming_fir_config() -> Result<ChainSpec, String> {
    ChainSpec::from_json_bytes(&include_bytes!("../../cli/res/flaming-fir.json")[..])
}

pub fn polkadot_config() -> Result<PolkadotChainSpec, String> {
    RealisChainSpec::from_json_bytes(&include_bytes!("../../../docker/realis.json")[..])
}

/// The default parachains host configuration.
#[cfg(any(
feature = "polkadot-native"
))]
fn default_parachains_host_configuration(
) -> polkadot_runtime_parachains::configuration::HostConfiguration<
    polkadot_primitives::v1::BlockNumber,
> {
    use polkadot_primitives::v1::{MAX_CODE_SIZE, MAX_POV_SIZE};

    polkadot_runtime_parachains::configuration::HostConfiguration {
        validation_upgrade_frequency: 1u32,
        validation_upgrade_delay: 1,
        code_retention_period: 1200,
        max_code_size: MAX_CODE_SIZE,
        max_pov_size: MAX_POV_SIZE,
        max_head_data_size: 32 * 1024,
        group_rotation_frequency: 20,
        chain_availability_period: 4,
        thread_availability_period: 4,
        max_upward_queue_count: 8,
        max_upward_queue_size: 1024 * 1024,
        max_downward_message_size: 1024 * 1024,
        ump_service_total_weight: 100_000_000_000,
        max_upward_message_size: 1024 * 1024,
        max_upward_message_num_per_candidate: 5,
        hrmp_sender_deposit: 0,
        hrmp_recipient_deposit: 0,
        hrmp_channel_max_capacity: 8,
        hrmp_channel_max_total_size: 8 * 1024,
        hrmp_max_parachain_inbound_channels: 4,
        hrmp_max_parathread_inbound_channels: 4,
        hrmp_channel_max_message_size: 1024 * 1024,
        hrmp_max_parachain_outbound_channels: 4,
        hrmp_max_parathread_outbound_channels: 4,
        hrmp_max_message_num_per_candidate: 5,
        dispute_period: 6,
        no_show_slots: 2,
        n_delay_tranches: 25,
        needed_approvals: 2,
        relay_vrf_modulo_samples: 2,
        zeroth_delay_tranche_width: 0,
        ..Default::default()
    }
}

#[cfg(feature = "polkadot-native")]
fn polkadot_session_keys(
    babe: BabeId,
    grandpa: GrandpaId,
    im_online: ImOnlineId,
    para_validator: ValidatorId,
    para_assignment: AssignmentId,
    authority_discovery: AuthorityDiscoveryId,
) -> polkadot::SessionKeys {
    polkadot::SessionKeys {
        babe,
        grandpa,
        im_online,
        para_validator,
        para_assignment,
        authority_discovery,
    }
}

#[cfg(feature = "polkadot-native")]
fn polkadot_staging_testnet_config_genesis(wasm_binary: &[u8]) -> polkadot::GenesisConfig {
    // subkey inspect "$SECRET"
    let endowed_accounts: Vec<AccountId> = vec![
            hex!["781f4331933557680355932ef7f39b88e938fcb4338cc0e03edb3c523e47fd09"].into(),
            hex!["fe8823fb870f61eed24638a228adbe5885de6e945bb1375ca6a7415a4824756e"].into(),
            hex!["bc95bdafa3582b0ecbf5caf1e30b00412fa7c2dfbccd518f3b842c63890cc979"].into(),
            hex!["08bdc3547dc26a647391b509960b00adafa550496e9a95339a2faa02343be20f"].into(),
            hex!["d4c2ffb1322efb7fe78463ad6f24301751454685edd96640197cab2c44e1b16c"].into(),
            hex!["10f908b91793b30fc4870e255a0e102745e2a8f268814cd28389ba7f4220764d"].into(),
            hex!["d671cde125c8b7f42afbf40fb9d0d93d4d80c888cd34824c99ab292b589dbe75"].into(),
            hex!["d4c2ffb1322efb7fe78463ad6f24301751454685edd96640197cab2c44e1b16c"].into(),
            // node_runtime::pallet_staking::Pallet::<Runtime>::account_id(),
            // realis_game_api::Pallet::<Runtime>::account_id(),
        ];

    let initial_authorities: Vec<(
        AccountId,
        AccountId,
        BabeId,
        GrandpaId,
        ImOnlineId,
        ValidatorId,
        AssignmentId,
        AuthorityDiscoveryId,
    )> = vec![
        (
            //5ERCqy118nnXDai8g4t3MjdX7ZC5PrQzQpe9vwex5cELWqbt
            hex!["681af4f93073484e1acd6b27395d0d258f1a6b158c808846c8fd05ee2435056e"].into(),
            //5GTS114cfQNBgpQULhMaNCPXGds6NokegCnikxDe1vqANhtn
            hex!["c2463372598ebabd21ee5bc33e1d7e77f391d2df29ce2fbe6bed0d13be629a45"].into(),
            //5FhGbceKeH7fuGogcBwd28ZCkAwDGYBADCTeHiYrvx2ztyRd
            hex!["a097bfc6a33499ed843b711f52f523f8a7174f798a9f98620e52f4170dbe2948"]
                .unchecked_into(),
            //5Es7nDkJt2by5qVCCD7PZJdp76KJw1LdRCiNst5S5f4eecnz
            hex!["7bde49dda82c2c9f082b807ef3ceebff96437d67b3e630c584db7a220ecafacf"]
                .unchecked_into(),
            //5D4e8zRjaYzFamqChGPPtu26PcKbKgUrhb7WqcNbKa2RDFUR
            hex!["2c2fb730a7d9138e6d62fcf516f9ecc2d712af3f2f03ca330c9564b8c0c1bb33"]
                .unchecked_into(),
            //5DD3JY5ENkjcgVFbVSgUbZv7WmrnyJ8bxxu56ee6hZFiRdnh
            hex!["3297a8622988cc23dd9c131e3fb8746d49e007f6e58a81d43420cd539e250e4c"]
                .unchecked_into(),
            //5Gpodowhud8FG9xENXR5YwTFbUAWyoEtw7sYFytFsG4z7SU6
            hex!["d2932edf775088bd088dc5a112ad867c24cc95858f77f8a1ab014de8d4f96a3f"]
                .unchecked_into(),
            //5GUMj8tnjL3PJZgXoiWtgLCaMVNHBNeSeTqDsvcxmaVAjKn9
            hex!["c2fb0f74591a00555a292bc4882d3158bafc4c632124cb60681f164ef81bcf72"]
                .unchecked_into(),
        ),
        (
            //5HgDCznTkHKUjzPkQoTZGWbvbyqB7sqHDBPDKdF1FyVYM7Er
            hex!["f8418f189f84814fd40cc1b2e90873e72ea789487f3b98ed42811ba76d10fc37"].into(),
            //5GQTryeFwuvgmZ2tH5ZeAKZHRM9ch5WGVGo6ND9P8f9uMsNY
            hex!["c002bb4af4a1bd2f33d104aef8a41878fe1ac94ba007029c4dfdefa8b698d043"].into(),
            //5C7YkWSVH1zrpsE5KwW1ua1qatyphzYxiZrL24mjkxz7mUbn
            hex!["022b14fbcf65a93b81f453105b9892c3fc4aa74c22c53b4abab019e1d58fbd41"]
                .unchecked_into(),
            //5GwFC6Tmg4fhj4PxSqHycgJxi3PDfnC9RGDsNHoRwAvXvpnZ
            hex!["d77cafd3b32c8b52b0e2780a586a6e527c94f1bdec117c4e4acb0a491461ffa3"]
                .unchecked_into(),
            //5DSVrGURuDuh8Luzo8FYq7o2NWiUSLSN6QAVNrj9BtswWH6R
            hex!["3cdb36a5a14715999faffd06c5b9e5dcdc24d4b46bc3e4df1aaad266112a7b49"]
                .unchecked_into(),
            //5DLEG2AupawCXGwhJtrzBRc3zAhuP8V662dDrUTzAsCiB9Ec
            hex!["38134245c9919ecb20bf2eedbe943b69ba92ceb9eb5477b92b0afd3cb6ce2858"]
                .unchecked_into(),
            //5D83o9fDgnHxaKPkSx59hk8zYzqcgzN2mrf7cp8fiVEi7V4E
            hex!["2ec917690dc1d676002e3504c530b2595490aa5a4603d9cc579b9485b8d0d854"]
                .unchecked_into(),
            //5DwBJquZgncRWXFxj2ydbF8LBUPPUbiq86sXWXgm8Z38m8L2
            hex!["52bae9b8dedb8058dda93ec6f57d7e5a517c4c9f002a4636fada70fed0acf376"]
                .unchecked_into(),
        ),
        (
            //5DMHpkRpQV7NWJFfn2zQxCLiAKv7R12PWFRPHKKk5X3JkYfP
            hex!["38e280b35d08db46019a210a944e4b7177665232ab679df12d6a8bbb317a2276"].into(),
            //5FbJpSHmFDe5FN3DVGe1R345ZePL9nhcC9V2Cczxo7q8q6rN
            hex!["9c0bc0e2469924d718ae683737f818a47c46b0612376ecca06a2ac059fe1f870"].into(),
            //5E5Pm3Udzxy26KGkLE5pc8JPfQrvkYHiaXWtuEfmQsBSgep9
            hex!["58fecadc2df8182a27e999e7e1fd7c99f8ec18f2a81f9a0db38b3653613f3f4d"]
                .unchecked_into(),
            //5FxcystSLHtaWoy2HEgRNerj9PrUs452B6AvHVnQZm5ZQmqE
            hex!["ac4d0c5e8f8486de05135c10a707f58aa29126d5eb28fdaaba00f9a505f5249d"]
                .unchecked_into(),
            //5E7KqVXaVGuAqiqMigpuH8oXHLVh4tmijmpJABLYANpjMkem
            hex!["5a781385a0235fe8594dd101ec55ef9ba01883f8563a0cdd37b89e0303f6a578"]
                .unchecked_into(),
            //5H9AybjkpyZ79yN5nHuBqs6RKuZPgM7aAVVvTQsDFovgXb2A
            hex!["e09570f62a062450d4406b4eb43e7f775ff954e37606646cd590d1818189501f"]
                .unchecked_into(),
            //5Ccgs7VwJKBawMbwMENDmj2eFAxhFdGksVHdk8aTAf4w7xox
            hex!["1864832dae34df30846d5cc65973f58a2d01b337d094b1284ec3466ecc90251d"]
                .unchecked_into(),
            //5EsSaZZ7niJs7hmAtp4QeK19AcAuTp7WXB7N7gRipVooerq4
            hex!["7c1d92535e6d94e21cffea6633a855a7e3c9684cd2f209e5ddbdeaf5111e395b"]
                .unchecked_into(),
        ),
        (
            //5Ea11qhmGRntQ7pyEkEydbwxvfrYwGMKW6rPERU4UiSBB6rd
            hex!["6ed057d2c833c45629de2f14b9f6ce6df1edbf9421b7a638e1fb4828c2bd2651"].into(),
            //5CZomCZwPB78BZMZsCiy7WSpkpHhdrN8QTSyjcK3FFEZHBor
            hex!["1631ff446b3534d031adfc37b7f7aed26d2a6b3938d10496aab3345c54707429"].into(),
            //5CSM6vppouFHzAVPkVFWN76DPRUG7B9qwJe892ccfSfJ8M5f
            hex!["108188c43a7521e1abe737b343341c2179a3a89626c7b017c09a5b10df6f1c42"]
                .unchecked_into(),
            //5GwkG4std9KcjYi3ThSC7QWfhqokmYVvWEqTU9h7iswjhLnr
            hex!["d7de8a43f7ee49fa3b3aaf32fb12617ec9ff7b246a46ab14e9c9d259261117fa"]
                .unchecked_into(),
            //5CoUk3wrCGJAWbiJEcsVjYhnd2JAHvR59jBRbSw77YrBtRL1
            hex!["209f680bc501f9b59358efe3636c51fd61238a8659bac146db909aea2595284b"]
                .unchecked_into(),
            //5EcSu96wprFM7G2HfJTjYu8kMParnYGznSUNTsoEKXywEsgG
            hex!["70adf80395b3f59e4cab5d9da66d5a286a0b6e138652a06f72542e46912df922"]
                .unchecked_into(),
            //5Ge3sjpD43Cuy7rNoJQmE9WctgCn6Faw89Pe7xPs3i55eHwJ
            hex!["ca5f6b970b373b303f64801a0c2cadc4fc05272c6047a2560a27d0c65589ca1d"]
                .unchecked_into(),
            //5EFcjHLvB2z5vd5g63n4gABmhzP5iPsKvTwd8sjfvTehNNrk
            hex!["60cae7fa5a079d9fc8061d715fbcc35ef57c3b00005694c2badce22dcc5a9f1b"]
                .unchecked_into(),
        ),
    ];

    //sudo account
    let root_key = hex!["10f908b91793b30fc4870e255a0e102745e2a8f268814cd28389ba7f4220764d"].into();
    //NFT Master
    let sudo_1: AccountId =
        hex!["10f908b91793b30fc4870e255a0e102745e2a8f268814cd28389ba7f4220764d"].into();
    let sudo_2: AccountId =
        hex!["1aa0d5c594a4581ec17069ec9631cd6225d5fb403fe4d85c8ec8aa51833fdf7f"].into();
    let sudo_3: AccountId =
        hex!["cc32b24b66c8636b31394dce95949a27022c901d2597c5584554aa5d81db7416"].into();
    let sudo_4: AccountId =
        hex!["24c42c17c4f95987c9916fc7e9bcd0c9385b6724f72658d943b643b6c3d83b73"].into();
    let sudo_5: AccountId =
        hex!["a662140fcc5ff36f191a4f8ce6fd314a33c0149a5864060d50fd06c44535b777"].into();

    let test_acc_1: AccountId =
        sp_core::sr25519::Public::from_str("5CFvFsZy7ViPUdEuuK19QuUqqCApVr2wbRWkHjcvQGsgzQmv")
            .unwrap()
            .into();
    let test_acc_2: AccountId =
        sp_core::sr25519::Public::from_str("5DHasdJm8bVxqxuAu5p8QfDMFfABtt3Rgf8feWSDP8KmYVAL")
            .unwrap()
            .into();
    let test_acc_3: AccountId =
        sp_core::sr25519::Public::from_str("5EAH4UrLxaNM6Kz1pH9bNSkKxAe21DkdDfyPahoj6KFN79Ax")
            .unwrap()
            .into();
    let test_acc_4: AccountId =
        sp_core::sr25519::Public::from_str("5HnBcUqsgjBKD5cpAi4wDrTBhftFjt1ZFG8pXLmN5u1zozRk")
            .unwrap()
            .into();

    let nft_master = vec![
        sudo_1.clone(),
        sudo_2.clone(),
        sudo_3.clone(),
        sudo_4.clone(),
        sudo_5.clone(),
    ];

    let api_master = vec![
        sudo_1.clone(),
        sudo_2.clone(),
        sudo_3.clone(),
        sudo_4.clone(),
        sudo_5.clone(),
    ];

    let bridge_master = vec![
        sudo_1.clone(),
        sudo_2.clone(),
        sudo_3.clone(),
        sudo_4.clone(),
        sudo_5.clone(),
    ];

    let white_list = vec![
        test_acc_1.clone(),
        test_acc_2.clone(),
        test_acc_3.clone(),
        test_acc_4.clone(),
    ];

    const ENDOWMENT: u128 = 1_000_000 * DOT;
    const STASH: u128 = 100 * DOT;

    polkadot::GenesisConfig {
        system: polkadot::SystemConfig { code: wasm_binary.to_vec() },
        balances: polkadot::BalancesConfig {
            balances: endowed_accounts
                .iter()
                .map(|k: &AccountId| (k.clone(), ENDOWMENT))
                .chain(initial_authorities.iter().map(|x| (x.0.clone(), STASH)))
                .collect(),
        },
        indices: polkadot::IndicesConfig { indices: vec![] },
        session: polkadot::SessionConfig {
            keys: initial_authorities
                .iter()
                .map(|x| {
                    (
                        x.0.clone(),
                        x.0.clone(),
                        polkadot_session_keys(
                            x.2.clone(),
                            x.3.clone(),
                            x.4.clone(),
                            x.5.clone(),
                            x.6.clone(),
                            x.7.clone(),
                        ),
                    )
                })
                .collect::<Vec<_>>(),
        },
        staking: polkadot::StakingConfig {
            validator_count: 50,
            minimum_validator_count: 4,
            stakers: initial_authorities
                .iter()
                .map(|x| (x.0.clone(), x.1.clone(), STASH, polkadot::StakerStatus::Validator))
                .collect(),
            invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
            force_era: node_runtime::pallet_staking::Forcing::ForceNone,
            slash_reward_fraction: Perbill::from_percent(10),
            ..Default::default()
        },
        phragmen_election: Default::default(),
        democracy: Default::default(),
        council: polkadot::CouncilConfig { members: vec![], phantom: Default::default() },
        technical_committee: polkadot::TechnicalCommitteeConfig {
            members: vec![],
            phantom: Default::default(),
        },
        technical_membership: Default::default(),
        babe: polkadot::BabeConfig {
            authorities: Default::default(),
            epoch_config: Some(polkadot::BABE_GENESIS_EPOCH_CONFIG),
        },
        grandpa: Default::default(),
        im_online: Default::default(),
        authority_discovery: polkadot::AuthorityDiscoveryConfig { keys: vec![] },
        claims: polkadot::ClaimsConfig { claims: vec![], vesting: vec![] },
        vesting: polkadot::VestingConfig { vesting: vec![] },
        treasury: Default::default(),
        configuration: polkadot::ConfigurationConfig {
            config: default_parachains_host_configuration(),
        },
        paras: Default::default(),
        xcm_pallet: polkadot::XcmPalletConfig { safe_xcm_version: Some(2) },
        sudo: polkadot::SudoConfig { key: root_key },
        nft: polkadot::NftConfig {
            nft_masters: nft_master,
        },
        realis_game_api: node_runtime::RealisGameApiConfig {
            api_masters: api_master,
            whitelist: white_list,
        },
        realis_bridge: node_runtime::RealisBridgeConfig {
            bridge_masters: bridge_master,
        },
    }
}

/// Polkadot staging testnet config.
#[cfg(feature = "polkadot-native")]
pub fn polkadot_staging_testnet_config() -> Result<PolkadotChainSpec, String> {
    let wasm_binary = polkadot::WASM_BINARY.ok_or("Polkadot development wasm not available")?;
    let boot_nodes = vec![];

    Ok(PolkadotChainSpec::from_genesis(
        "Staging Testnet",
        "staging_testnet",
        ChainType::Live,
        move || polkadot_staging_testnet_config_genesis(wasm_binary),
        boot_nodes,
        Some(
            TelemetryEndpoints::new(vec![(POLKADOT_STAGING_TELEMETRY_URL.to_string(), 0)])
                .expect("Polkadot Staging telemetry url is valid; qed"),
        ),
        Some(DEFAULT_PROTOCOL_ID),
        None,
        Default::default(),
    ))
}

/// Helper function to generate a crypto pair from seed
pub fn get_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
    where
        AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_from_seed::<TPublic>(seed)).into_account()
}

/// Helper function to generate stash, controller and session key from seed
pub fn get_authority_keys_from_seed(
    seed: &str,
) -> (
    AccountId,
    AccountId,
    BabeId,
    GrandpaId,
    ImOnlineId,
    ValidatorId,
    AssignmentId,
    AuthorityDiscoveryId,
    BeefyId,
) {
    let keys = get_authority_keys_from_seed_no_beefy(seed);
    (keys.0, keys.1, keys.2, keys.3, keys.4, keys.5, keys.6, keys.7, get_from_seed::<BeefyId>(seed))
}

/// Helper function to generate stash, controller and session key from seed
pub fn get_authority_keys_from_seed_no_beefy(
    seed: &str,
) -> (
    AccountId,
    AccountId,
    BabeId,
    GrandpaId,
    ImOnlineId,
    ValidatorId,
    AssignmentId,
    AuthorityDiscoveryId,
) {
    (
        get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", seed)),
        get_account_id_from_seed::<sr25519::Public>(seed),
        get_from_seed::<BabeId>(seed),
        get_from_seed::<GrandpaId>(seed),
        get_from_seed::<ImOnlineId>(seed),
        get_from_seed::<ValidatorId>(seed),
        get_from_seed::<AssignmentId>(seed),
        get_from_seed::<AuthorityDiscoveryId>(seed),
    )
}

fn testnet_accounts() -> Vec<AccountId> {
    vec![
        get_account_id_from_seed::<sr25519::Public>("Alice"),
        get_account_id_from_seed::<sr25519::Public>("Bob"),
        get_account_id_from_seed::<sr25519::Public>("Charlie"),
        get_account_id_from_seed::<sr25519::Public>("Dave"),
        get_account_id_from_seed::<sr25519::Public>("Eve"),
        get_account_id_from_seed::<sr25519::Public>("Ferdie"),
        get_account_id_from_seed::<sr25519::Public>("Alice//stash"),
        get_account_id_from_seed::<sr25519::Public>("Bob//stash"),
        get_account_id_from_seed::<sr25519::Public>("Charlie//stash"),
        get_account_id_from_seed::<sr25519::Public>("Dave//stash"),
        get_account_id_from_seed::<sr25519::Public>("Eve//stash"),
        get_account_id_from_seed::<sr25519::Public>("Ferdie//stash"),
        realis_game_api::Pallet::<Runtime>::account_id(),
        pallet_staking::Pallet::<Runtime>::account_id(),
    ]
}

/// Helper function to create polkadot `GenesisConfig` for testing
#[cfg(feature = "polkadot-native")]
pub fn polkadot_testnet_genesis(
    wasm_binary: &[u8],
    _initial_authorities: Vec<(
        AccountId,
        AccountId,
        BabeId,
        GrandpaId,
        ImOnlineId,
        ValidatorId,
        AssignmentId,
        AuthorityDiscoveryId,
    )>,
    _root_key: AccountId,
    endowed_accounts: Option<Vec<AccountId>>,
) -> polkadot::GenesisConfig {
    let endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(testnet_accounts);

    let initial_authorities: Vec<(
        AccountId,
        AccountId,
        BabeId,
        GrandpaId,
        ImOnlineId,
        ValidatorId,
        AssignmentId,
        AuthorityDiscoveryId,
    )> = vec![
        (
            //5ERCqy118nnXDai8g4t3MjdX7ZC5PrQzQpe9vwex5cELWqbt
            hex!["681af4f93073484e1acd6b27395d0d258f1a6b158c808846c8fd05ee2435056e"].into(),
            //5GTS114cfQNBgpQULhMaNCPXGds6NokegCnikxDe1vqANhtn
            hex!["c2463372598ebabd21ee5bc33e1d7e77f391d2df29ce2fbe6bed0d13be629a45"].into(),
            //5FhGbceKeH7fuGogcBwd28ZCkAwDGYBADCTeHiYrvx2ztyRd
            hex!["a097bfc6a33499ed843b711f52f523f8a7174f798a9f98620e52f4170dbe2948"]
                .unchecked_into(),
            //5Es7nDkJt2by5qVCCD7PZJdp76KJw1LdRCiNst5S5f4eecnz
            hex!["7bde49dda82c2c9f082b807ef3ceebff96437d67b3e630c584db7a220ecafacf"]
                .unchecked_into(),
            //5D4e8zRjaYzFamqChGPPtu26PcKbKgUrhb7WqcNbKa2RDFUR
            hex!["2c2fb730a7d9138e6d62fcf516f9ecc2d712af3f2f03ca330c9564b8c0c1bb33"]
                .unchecked_into(),
            //5DD3JY5ENkjcgVFbVSgUbZv7WmrnyJ8bxxu56ee6hZFiRdnh
            hex!["3297a8622988cc23dd9c131e3fb8746d49e007f6e58a81d43420cd539e250e4c"]
                .unchecked_into(),
            //5Gpodowhud8FG9xENXR5YwTFbUAWyoEtw7sYFytFsG4z7SU6
            hex!["d2932edf775088bd088dc5a112ad867c24cc95858f77f8a1ab014de8d4f96a3f"]
                .unchecked_into(),
            //5GUMj8tnjL3PJZgXoiWtgLCaMVNHBNeSeTqDsvcxmaVAjKn9
            hex!["c2fb0f74591a00555a292bc4882d3158bafc4c632124cb60681f164ef81bcf72"]
                .unchecked_into(),
        ),
        (
            //5HgDCznTkHKUjzPkQoTZGWbvbyqB7sqHDBPDKdF1FyVYM7Er
            hex!["f8418f189f84814fd40cc1b2e90873e72ea789487f3b98ed42811ba76d10fc37"].into(),
            //5GQTryeFwuvgmZ2tH5ZeAKZHRM9ch5WGVGo6ND9P8f9uMsNY
            hex!["c002bb4af4a1bd2f33d104aef8a41878fe1ac94ba007029c4dfdefa8b698d043"].into(),
            //5C7YkWSVH1zrpsE5KwW1ua1qatyphzYxiZrL24mjkxz7mUbn
            hex!["022b14fbcf65a93b81f453105b9892c3fc4aa74c22c53b4abab019e1d58fbd41"]
                .unchecked_into(),
            //5GwFC6Tmg4fhj4PxSqHycgJxi3PDfnC9RGDsNHoRwAvXvpnZ
            hex!["d77cafd3b32c8b52b0e2780a586a6e527c94f1bdec117c4e4acb0a491461ffa3"]
                .unchecked_into(),
            //5DSVrGURuDuh8Luzo8FYq7o2NWiUSLSN6QAVNrj9BtswWH6R
            hex!["3cdb36a5a14715999faffd06c5b9e5dcdc24d4b46bc3e4df1aaad266112a7b49"]
                .unchecked_into(),
            //5DLEG2AupawCXGwhJtrzBRc3zAhuP8V662dDrUTzAsCiB9Ec
            hex!["38134245c9919ecb20bf2eedbe943b69ba92ceb9eb5477b92b0afd3cb6ce2858"]
                .unchecked_into(),
            //5D83o9fDgnHxaKPkSx59hk8zYzqcgzN2mrf7cp8fiVEi7V4E
            hex!["2ec917690dc1d676002e3504c530b2595490aa5a4603d9cc579b9485b8d0d854"]
                .unchecked_into(),
            //5DwBJquZgncRWXFxj2ydbF8LBUPPUbiq86sXWXgm8Z38m8L2
            hex!["52bae9b8dedb8058dda93ec6f57d7e5a517c4c9f002a4636fada70fed0acf376"]
                .unchecked_into(),
        ),
        (
            //5DMHpkRpQV7NWJFfn2zQxCLiAKv7R12PWFRPHKKk5X3JkYfP
            hex!["38e280b35d08db46019a210a944e4b7177665232ab679df12d6a8bbb317a2276"].into(),
            //5FbJpSHmFDe5FN3DVGe1R345ZePL9nhcC9V2Cczxo7q8q6rN
            hex!["9c0bc0e2469924d718ae683737f818a47c46b0612376ecca06a2ac059fe1f870"].into(),
            //5E5Pm3Udzxy26KGkLE5pc8JPfQrvkYHiaXWtuEfmQsBSgep9
            hex!["58fecadc2df8182a27e999e7e1fd7c99f8ec18f2a81f9a0db38b3653613f3f4d"]
                .unchecked_into(),
            //5FxcystSLHtaWoy2HEgRNerj9PrUs452B6AvHVnQZm5ZQmqE
            hex!["ac4d0c5e8f8486de05135c10a707f58aa29126d5eb28fdaaba00f9a505f5249d"]
                .unchecked_into(),
            //5E7KqVXaVGuAqiqMigpuH8oXHLVh4tmijmpJABLYANpjMkem
            hex!["5a781385a0235fe8594dd101ec55ef9ba01883f8563a0cdd37b89e0303f6a578"]
                .unchecked_into(),
            //5H9AybjkpyZ79yN5nHuBqs6RKuZPgM7aAVVvTQsDFovgXb2A
            hex!["e09570f62a062450d4406b4eb43e7f775ff954e37606646cd590d1818189501f"]
                .unchecked_into(),
            //5Ccgs7VwJKBawMbwMENDmj2eFAxhFdGksVHdk8aTAf4w7xox
            hex!["1864832dae34df30846d5cc65973f58a2d01b337d094b1284ec3466ecc90251d"]
                .unchecked_into(),
            //5EsSaZZ7niJs7hmAtp4QeK19AcAuTp7WXB7N7gRipVooerq4
            hex!["7c1d92535e6d94e21cffea6633a855a7e3c9684cd2f209e5ddbdeaf5111e395b"]
                .unchecked_into(),
        ),
        (
            //5Ea11qhmGRntQ7pyEkEydbwxvfrYwGMKW6rPERU4UiSBB6rd
            hex!["6ed057d2c833c45629de2f14b9f6ce6df1edbf9421b7a638e1fb4828c2bd2651"].into(),
            //5CZomCZwPB78BZMZsCiy7WSpkpHhdrN8QTSyjcK3FFEZHBor
            hex!["1631ff446b3534d031adfc37b7f7aed26d2a6b3938d10496aab3345c54707429"].into(),
            //5CSM6vppouFHzAVPkVFWN76DPRUG7B9qwJe892ccfSfJ8M5f
            hex!["108188c43a7521e1abe737b343341c2179a3a89626c7b017c09a5b10df6f1c42"]
                .unchecked_into(),
            //5GwkG4std9KcjYi3ThSC7QWfhqokmYVvWEqTU9h7iswjhLnr
            hex!["d7de8a43f7ee49fa3b3aaf32fb12617ec9ff7b246a46ab14e9c9d259261117fa"]
                .unchecked_into(),
            //5CoUk3wrCGJAWbiJEcsVjYhnd2JAHvR59jBRbSw77YrBtRL1
            hex!["209f680bc501f9b59358efe3636c51fd61238a8659bac146db909aea2595284b"]
                .unchecked_into(),
            //5EcSu96wprFM7G2HfJTjYu8kMParnYGznSUNTsoEKXywEsgG
            hex!["70adf80395b3f59e4cab5d9da66d5a286a0b6e138652a06f72542e46912df922"]
                .unchecked_into(),
            //5Ge3sjpD43Cuy7rNoJQmE9WctgCn6Faw89Pe7xPs3i55eHwJ
            hex!["ca5f6b970b373b303f64801a0c2cadc4fc05272c6047a2560a27d0c65589ca1d"]
                .unchecked_into(),
            //5EFcjHLvB2z5vd5g63n4gABmhzP5iPsKvTwd8sjfvTehNNrk
            hex!["60cae7fa5a079d9fc8061d715fbcc35ef57c3b00005694c2badce22dcc5a9f1b"]
                .unchecked_into(),
        ),
    ];

    //sudo account
    let root_key = hex!["10f908b91793b30fc4870e255a0e102745e2a8f268814cd28389ba7f4220764d"].into();
    //NFT Master
    let sudo_1: AccountId =
        hex!["10f908b91793b30fc4870e255a0e102745e2a8f268814cd28389ba7f4220764d"].into();
    let sudo_2: AccountId =
        hex!["1aa0d5c594a4581ec17069ec9631cd6225d5fb403fe4d85c8ec8aa51833fdf7f"].into();
    let sudo_3: AccountId =
        hex!["cc32b24b66c8636b31394dce95949a27022c901d2597c5584554aa5d81db7416"].into();
    let sudo_4: AccountId =
        hex!["24c42c17c4f95987c9916fc7e9bcd0c9385b6724f72658d943b643b6c3d83b73"].into();
    let sudo_5: AccountId =
        hex!["a662140fcc5ff36f191a4f8ce6fd314a33c0149a5864060d50fd06c44535b777"].into();

    let test_acc_1: AccountId =
        sp_core::sr25519::Public::from_str("5CFvFsZy7ViPUdEuuK19QuUqqCApVr2wbRWkHjcvQGsgzQmv")
            .unwrap()
            .into();
    let test_acc_2: AccountId =
        sp_core::sr25519::Public::from_str("5DHasdJm8bVxqxuAu5p8QfDMFfABtt3Rgf8feWSDP8KmYVAL")
            .unwrap()
            .into();
    let test_acc_3: AccountId =
        sp_core::sr25519::Public::from_str("5EAH4UrLxaNM6Kz1pH9bNSkKxAe21DkdDfyPahoj6KFN79Ax")
            .unwrap()
            .into();
    let test_acc_4: AccountId =
        sp_core::sr25519::Public::from_str("5HnBcUqsgjBKD5cpAi4wDrTBhftFjt1ZFG8pXLmN5u1zozRk")
            .unwrap()
            .into();

    let nft_master = vec![
        sudo_1.clone(),
        sudo_2.clone(),
        sudo_3.clone(),
        sudo_4.clone(),
        sudo_5.clone(),
    ];

    let api_master = vec![
        sudo_1.clone(),
        sudo_2.clone(),
        sudo_3.clone(),
        sudo_4.clone(),
        sudo_5.clone(),
    ];

    let bridge_master = vec![
        sudo_1.clone(),
        sudo_2.clone(),
        sudo_3.clone(),
        sudo_4.clone(),
        sudo_5.clone(),
    ];

    let white_list = vec![
        test_acc_1.clone(),
        test_acc_2.clone(),
        test_acc_3.clone(),
        test_acc_4.clone(),
    ];

    // const ENDOWMENT: u128 = 1_000_000 * DOT;
    // const STASH: u128 = 100 * DOT;
    const ENDOWMENT: node_primitives::Balance = 30_000 * node_runtime::constants::currency::DOLLARS / 10;
    const GAME_WALLET: node_primitives::Balance = 10_000_000 * node_runtime::constants::currency::DOLLARS / 10;
    const STAKING_POOL: node_primitives::Balance = 30_000_000 * node_runtime::constants::currency::DOLLARS / 10;
    const STASH: node_primitives::Balance = ENDOWMENT / 1000;

    let pallet_id_staking = pallet_staking::Pallet::<Runtime>::account_id();
    let game_wallet = realis_game_api::Pallet::<Runtime>::account_id();

    polkadot::GenesisConfig {
        system: polkadot::SystemConfig { code: wasm_binary.to_vec() },
        indices: polkadot::IndicesConfig { indices: vec![] },
        balances: polkadot::BalancesConfig {
            balances: endowed_accounts
                .iter()
                .cloned()
                .map(|x| {
                    if x == pallet_id_staking {
                        (x, STAKING_POOL)
                    } else if x == game_wallet {
                        (x, GAME_WALLET)
                    } else {
                        (x, ENDOWMENT)
                    }
                })
                .collect(),
        },
        session: polkadot::SessionConfig {
            keys: initial_authorities
                .iter()
                .map(|x| {
                    (
                        x.0.clone(),
                        x.0.clone(),
                        polkadot_session_keys(
                            x.2.clone(),
                            x.3.clone(),
                            x.4.clone(),
                            x.5.clone(),
                            x.6.clone(),
                            x.7.clone(),
                        ),
                    )
                })
                .collect::<Vec<_>>(),
        },
        staking: polkadot::StakingConfig {
            minimum_validator_count: 1,
            validator_count: initial_authorities.len() as u32,
            stakers: initial_authorities
                .iter()
                .map(|x| (x.0.clone(), x.1.clone(), STASH, polkadot::StakerStatus::Validator))
                .collect(),
            invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
            force_era: node_runtime::pallet_staking::Forcing::NotForcing,
            slash_reward_fraction: Perbill::from_percent(10),
            ..Default::default()
        },
        phragmen_election: Default::default(),
        democracy: polkadot::DemocracyConfig::default(),
        council: polkadot::CouncilConfig { members: vec![], phantom: Default::default() },
        technical_committee: polkadot::TechnicalCommitteeConfig {
            members: vec![],
            phantom: Default::default(),
        },
        technical_membership: Default::default(),
        babe: polkadot::BabeConfig {
            authorities: Default::default(),
            epoch_config: Some(polkadot::BABE_GENESIS_EPOCH_CONFIG),
        },
        grandpa: Default::default(),
        im_online: Default::default(),
        authority_discovery: polkadot::AuthorityDiscoveryConfig { keys: vec![] },
        claims: polkadot::ClaimsConfig { claims: vec![], vesting: vec![] },
        vesting: polkadot::VestingConfig { vesting: vec![] },
        treasury: Default::default(),
        configuration: polkadot::ConfigurationConfig {
            config: default_parachains_host_configuration(),
        },
        paras: Default::default(),
        xcm_pallet: polkadot::XcmPalletConfig { safe_xcm_version: Some(2) },
        sudo: polkadot::SudoConfig { key: root_key },
        nft: node_runtime::NftConfig {
            nft_masters: nft_master,
        },
        realis_game_api: node_runtime::RealisGameApiConfig {
            api_masters: api_master,
            whitelist: white_list,
        },
        realis_bridge: node_runtime::RealisBridgeConfig {
            bridge_masters: bridge_master,
        },
    }
}

// #[cfg(feature = "polkadot-native")]
fn polkadot_development_config_genesis(wasm_binary: &[u8]) -> polkadot::GenesisConfig {
    polkadot_testnet_genesis(
        wasm_binary,
        vec![get_authority_keys_from_seed_no_beefy("Alice")],
        get_account_id_from_seed::<sr25519::Public>("Alice"),
        None,
    )
}

/// Polkadot development config (single validator Alice)
#[cfg(feature = "polkadot-native")]
pub fn polkadot_development_config() -> Result<PolkadotChainSpec, String> {
    let wasm_binary = polkadot::WASM_BINARY.ok_or("Polkadot development wasm not available")?;

    Ok(PolkadotChainSpec::from_genesis(
        "Development",
        "dev",
        ChainType::Development,
        move || polkadot_development_config_genesis(wasm_binary),
        vec![],
        None,
        Some(DEFAULT_PROTOCOL_ID),
        None,
        Default::default(),
    ))
}

#[cfg(feature = "polkadot-native")]
fn polkadot_local_testnet_genesis(wasm_binary: &[u8]) -> polkadot::GenesisConfig {
    polkadot_testnet_genesis(
        wasm_binary,
        vec![
            get_authority_keys_from_seed_no_beefy("Alice"),
            get_authority_keys_from_seed_no_beefy("Bob"),
        ],
        get_account_id_from_seed::<sr25519::Public>("Alice"),
        None,
    )
}

/// Polkadot local testnet config (multivalidator Alice + Bob)
#[cfg(feature = "polkadot-native")]
pub fn polkadot_local_testnet_config() -> Result<PolkadotChainSpec, String> {
    let wasm_binary = polkadot::WASM_BINARY.ok_or("Polkadot development wasm not available")?;

    Ok(PolkadotChainSpec::from_genesis(
        "Local Testnet",
        "local_testnet",
        ChainType::Local,
        move || polkadot_local_testnet_genesis(wasm_binary),
        vec![],
        None,
        Some(DEFAULT_PROTOCOL_ID),
        None,
        Default::default(),
    ))
}