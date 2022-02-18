use hex_literal::hex;
use sc_service::ChainType;
use sp_core::sr25519;
use std::str::FromStr;
use cumulus_primitives_core::ParaId;
pub use node_primitives::{AccountId, Balance, Signature};
pub use node_runtime::{Block, GenesisConfig};
use sc_telemetry::serde_json::Map;
use crate::chain_spec::ChainSpec;
use crate::chain_spec::{get_account_id_from_seed, authority_keys_from_seed};
use crate::chain_spec::testnet::testnet_genesis;

/// Development config (single validator Alice)
pub fn development_config() -> ChainSpec {
    let mut properties = Map::new();
    properties.insert("tokenDecimals".into(), 12.into());
    properties.insert("tokenSymbol".into(), "LIS".into());
    properties.insert("ss58Format".into(), 42.into());

    ChainSpec::from_genesis(
        "Development",
        "dev",
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
        test_acc_1.clone(),
        test_acc_2.clone(),
        test_acc_3.clone(),
        test_acc_4.clone(),
    ];

    let para_id = ParaId::new(2000);

    testnet_genesis(
        vec![authority_keys_from_seed("Alice")],
        vec![],
        get_account_id_from_seed::<sr25519::Public>("Alice"),
        nft_master,
        api_master,
        white_list,
        bridge_master,
        None,
        para_id,
    )
}
