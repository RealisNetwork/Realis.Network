use hex_literal::hex;
use sc_service::ChainType;
use sp_core::sr25519;
use std::str::FromStr;

use sc_telemetry::serde_json::Map;
use sp_runtime::Perbill;

use crate::chain_spec::ChainSpec;
use crate::chain_spec::{authority_keys_from_seed, get_account_id_from_seed, session_keys};
pub use node_primitives::{AccountId, Balance, Signature};
use node_runtime::constants::currency::DOLLARS;
use node_runtime::{
    pallet_staking, realis_game_api, wasm_binary_unwrap, AuthorityDiscoveryConfig,
    BabeConfig, BalancesConfig, GrandpaConfig, ImOnlineConfig, IndicesConfig, NftConfig,
    RealisBridgeConfig, RealisGameApiConfig, Runtime, SessionConfig, StakerStatus, StakingConfig,
    SudoConfig, SystemConfig, MAX_NOMINATIONS,
};
pub use node_runtime::{Block, GenesisConfig};

/// Development config (single validator Alice)
pub fn development_config() -> ChainSpec {
    let mut properties = Map::new();
    properties.insert("tokenDecimals".into(), 12.into());
    properties.insert("tokenSymbol".into(), "LIS".into());
    properties.insert("ss58Format".into(), 42.into());

    ChainSpec::from_genesis(
        "Development",
        "realis",
        ChainType::Development,
        development_config_genesis,
        vec![],
        None,
        None,
        Some(properties),
        Default::default(),
    )
}

fn development_config_genesis() -> GenesisConfig {
    let sudo_1: AccountId =
        hex!["10f908b91793b30fc4870e255a0e102745e2a8f268814cd28389ba7f4220764d"].into();
    let sudo_2: AccountId =
        sp_core::sr25519::Public::from_str("5D54XGhtRwffGsmrsaMyUdy3cZhtECnCGpxJgHto8e9csKEc")
            .unwrap()
            .into();
    let sudo_3: AccountId =
        sp_core::sr25519::Public::from_str("5C7odVdth9qyssQi81XHjkF8hWeLhMpnN27U24QgMB2YNJ6T")
            .unwrap()
            .into();
    let sudo_4: AccountId =
        sp_core::sr25519::Public::from_str("5EU1u5MaJLfB1hneKf7oPuZUa1PoSDBqpH6wU2E2yaB3h7Vi")
            .unwrap()
            .into();
    let sudo_5: AccountId =
        sp_core::sr25519::Public::from_str("5EKqhiruvSw3etmTccRcVT3dahwhMNutAyQEkhT3NoYeVkBf")
            .unwrap()
            .into();

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
        sudo_1.clone(),
        sudo_2.clone(),
        sudo_3.clone(),
        sudo_4.clone(),
        sudo_5.clone(),
        test_acc_1.clone(),
        test_acc_2.clone(),
        test_acc_3.clone(),
        test_acc_4.clone(),
    ];

    let endowed_accounts: Option<Vec<AccountId>> = None;
    let initial_nominators = vec![];
    let root_key = get_account_id_from_seed::<sr25519::Public>("Alice");

    let mut endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(|| {
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
            sudo_1.clone(),
            sudo_2.clone(),
            sudo_3.clone(),
            sudo_4.clone(),
            sudo_5.clone(),
        ]
    });
    // endow all authorities and nominators.
    let initial_authorities = vec![authority_keys_from_seed("Alice")];
    initial_authorities
        .iter()
        .map(|x| &x.0)
        .chain(initial_nominators.iter())
        .for_each(|x| {
            if !endowed_accounts.contains(&x) {
                endowed_accounts.push(x.clone())
            }
        });

    // stakers: all validators and nominators.
    let mut rng = rand::thread_rng();
    let stakers = initial_authorities
        .iter()
        .map(|x| (x.0.clone(), x.1.clone(), STASH, StakerStatus::Validator))
        .chain(initial_nominators.iter().map(|x| {
            use rand::{seq::SliceRandom, Rng};
            let limit = (MAX_NOMINATIONS as usize).min(initial_authorities.len());
            let count = rng.gen::<usize>() % limit;
            let nominations = initial_authorities
                .as_slice()
                .choose_multiple(&mut rng, count)
                .into_iter()
                .map(|choice| choice.0.clone())
                .collect::<Vec<_>>();
            (
                x.clone(),
                x.clone(),
                STASH,
                StakerStatus::Nominator(nominations),
            )
        }))
        .collect::<Vec<_>>();

    let _num_endowed_accounts = endowed_accounts.len();

    const ENDOWMENT: Balance = 30_000 * DOLLARS / 10;
    const GAME_WALLET: Balance = 10_000_000 * DOLLARS / 10;
    const STAKING_POOL: Balance = 30_000_000 * DOLLARS / 10;
    const STASH: Balance = ENDOWMENT / 1000;

    let pallet_id_staking = pallet_staking::Pallet::<Runtime>::account_id();
    let game_wallet = realis_game_api::Pallet::<Runtime>::account_id();

    GenesisConfig {
        system: SystemConfig {
            code: wasm_binary_unwrap().to_vec(),
            changes_trie_config: Default::default(),
        },
        balances: BalancesConfig {
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
        indices: IndicesConfig { indices: vec![] },
        session: SessionConfig {
            keys: initial_authorities
                .iter()
                .map(|x| {
                    (
                        x.0.clone(),
                        x.0.clone(),
                        session_keys(x.2.clone(), x.3.clone(), x.4.clone(), x.5.clone()),
                    )
                })
                .collect::<Vec<_>>(),
        },
        staking: StakingConfig {
            validator_count: initial_authorities.len() as u32,
            minimum_validator_count: initial_authorities.len() as u32,
            invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
            slash_reward_fraction: Perbill::from_percent(10),
            stakers,
            ..Default::default()
        },
        // democracy: DemocracyConfig::default(),
        // elections: ElectionsConfig {
        //     members: endowed_accounts
        //         .iter()
        //         .take((num_endowed_accounts + 1) / 2)
        //         .cloned()
        //         .map(|member| (member, STASH))
        //         .collect(),
        // },
        // council: CouncilConfig::default(),
        // technical_committee: TechnicalCommitteeConfig {
        //     members: endowed_accounts
        //         .iter()
        //         .take((num_endowed_accounts + 1) / 2)
        //         .cloned()
        //         .collect(),
        //     phantom: Default::default(),
        // },
        sudo: SudoConfig { key: root_key },
        babe: BabeConfig {
            authorities: vec![],
            epoch_config: Some(node_runtime::BABE_GENESIS_EPOCH_CONFIG),
        },
        im_online: ImOnlineConfig { keys: vec![] },
        authority_discovery: AuthorityDiscoveryConfig { keys: vec![] },
        grandpa: GrandpaConfig {
            authorities: vec![],
        },
        // technical_membership: Default::default(),
        // treasury: Default::default(),
        // society: SocietyConfig {
        //     members: endowed_accounts
        //         .iter()
        //         .take((num_endowed_accounts + 1) / 2)
        //         .cloned()
        //         .collect(),
        //     pot: 0,
        //     max_members: 999,
        // },
        vesting: Default::default(),
        gilt: Default::default(),
        nft: NftConfig {
            nft_masters: nft_master,
        },
        realis_game_api: RealisGameApiConfig {
            api_masters: api_master,
            whitelist: white_list,
        },
        realis_bridge: RealisBridgeConfig {
            bridge_masters: bridge_master,
        },
    }

    // testnet_genesis(
    //     vec![authority_keys_from_seed("Alice")],
    //     vec![],
    //     get_account_id_from_seed::<sr25519::Public>("Alice"),
    //     nft_master,
    //     api_master,
    //     white_list,
    //     bridge_master,
    //     None,
    // )
}
