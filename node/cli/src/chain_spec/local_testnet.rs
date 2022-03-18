use service::ChainType;
use sp_core::sr25519;
use service::ChainSpec;
pub use node_primitives::{AccountId, Balance, Signature};
pub use node_runtime::{Block, GenesisConfig};

/// Local testnet config (multivalidator Alice + Bob)
pub fn local_testnet_config() -> ChainSpec {
    ChainSpec::from_genesis(
        "Local Testnet",
        "local_testnet",
        ChainType::Local,
        local_testnet_genesis,
        vec![],
        None,
        None,
        None,
        Default::default(),
    )
}

fn local_testnet_genesis() -> GenesisConfig {
    testnet_genesis(
        vec![
            authority_keys_from_seed("Alice"),
            authority_keys_from_seed("Bob"),
        ],
        vec![],
        get_account_id_from_seed::<sr25519::Public>("Alice"),
        vec![get_account_id_from_seed::<sr25519::Public>("Alice")],
        vec![get_account_id_from_seed::<sr25519::Public>("Alice")],
        vec![get_account_id_from_seed::<sr25519::Public>("Alice")],
        vec![get_account_id_from_seed::<sr25519::Public>("Alice")],
        None,
    )
}
