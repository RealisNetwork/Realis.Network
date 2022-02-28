use hex_literal::hex;
use sc_service::ChainType;
use sp_core::crypto::UncheckedInto;
use std::str::FromStr;

pub use node_primitives::{AccountId, Balance, Signature};
use node_runtime::pallet_staking;
use node_runtime::realis_game_api;
use node_runtime::Runtime;
pub use node_runtime::{Block, GenesisConfig};
use sc_telemetry::serde_json::Map;
use crate::chain_spec::ChainSpec;
use crate::chain_spec::realis::realis_genesis;

///Realis test chain-spec
pub fn realis_testnet_config() -> ChainSpec {
    let mut properties = Map::new();
    properties.insert("tokenDecimals".into(), 12.into());
    properties.insert("tokenSymbol".into(), "LIS".into());
    properties.insert("ss58Format".into(), 42.into());

    ChainSpec::from_genesis(
        "Realis Network",
        "realis",
        ChainType::Live,
        realis_testnet_genesis,
        vec![],
        None,
        None,
        Some(properties),
        Default::default(),
    )
}

///Realis ctestnet genesis
pub fn realis_testnet_genesis() -> GenesisConfig {
    let initial_authorities = vec![
        (
            hex!["1aa0d5c594a4581ec17069ec9631cd6225d5fb403fe4d85c8ec8aa51833fdf7f"].into(),
            hex!["d671cde125c8b7f42afbf40fb9d0d93d4d80c888cd34824c99ab292b589dbe75"].into(),
            hex!["b7606f13fb700cdabffd98bf466557a9faeb68bc773ef6e2bf681b9913079d37"]
                .unchecked_into(),
            hex!["d671cde125c8b7f42afbf40fb9d0d93d4d80c888cd34824c99ab292b589dbe75"]
                .unchecked_into(),
            hex!["d671cde125c8b7f42afbf40fb9d0d93d4d80c888cd34824c99ab292b589dbe75"]
                .unchecked_into(),
            hex!["d671cde125c8b7f42afbf40fb9d0d93d4d80c888cd34824c99ab292b589dbe75"]
                .unchecked_into(),
        ),
        (
            hex!["cc32b24b66c8636b31394dce95949a27022c901d2597c5584554aa5d81db7416"].into(),
            hex!["10f908b91793b30fc4870e255a0e102745e2a8f268814cd28389ba7f4220764d"].into(),
            hex!["4a9e6cc2606a74d65ee2ba026e986024de8b60a22890023552b6cf6c977c8420"]
                .unchecked_into(),
            hex!["10f908b91793b30fc4870e255a0e102745e2a8f268814cd28389ba7f4220764d"]
                .unchecked_into(),
            hex!["10f908b91793b30fc4870e255a0e102745e2a8f268814cd28389ba7f4220764d"]
                .unchecked_into(),
            hex!["10f908b91793b30fc4870e255a0e102745e2a8f268814cd28389ba7f4220764d"]
                .unchecked_into(),
        ),
        (
            hex!["24c42c17c4f95987c9916fc7e9bcd0c9385b6724f72658d943b643b6c3d83b73"].into(),
            hex!["dc869f188c87d823da3d8e6b069a2688d0772d2dc3f09d8dfa96b8551a601513"].into(),
            hex!["32e610d5ed216b2681ba9ad4907f05220ef9b81edf7049dd73c732a670c14379"]
                .unchecked_into(),
            hex!["dc869f188c87d823da3d8e6b069a2688d0772d2dc3f09d8dfa96b8551a601513"]
                .unchecked_into(),
            hex!["dc869f188c87d823da3d8e6b069a2688d0772d2dc3f09d8dfa96b8551a601513"]
                .unchecked_into(),
            hex!["dc869f188c87d823da3d8e6b069a2688d0772d2dc3f09d8dfa96b8551a601513"]
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

    let endowed_accounts = vec![
        sudo_1.clone(),
        sudo_2.clone(),
        sudo_3.clone(),
        sudo_4.clone(),
        sudo_5.clone(),
        realis_game_api::Pallet::<Runtime>::account_id(),
        pallet_staking::Pallet::<Runtime>::account_id(),
    ];

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

    realis_genesis(
        initial_authorities,
        vec![],
        root_key,
        nft_master,
        api_master,
        white_list,
        bridge_master,
        Some(endowed_accounts),
    )
}
