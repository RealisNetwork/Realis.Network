use cumulus_primitives_core::ParaId;
use grandpa_primitives::AuthorityId as GrandpaId;
use node_runtime::{
    wasm_binary_unwrap, AuthorityDiscoveryConfig, BabeConfig,
    BalancesConfig, Block, CouncilConfig,
    DemocracyConfig, GrandpaConfig, ImOnlineConfig, IndicesConfig,
    NftConfig, RealisBridgeConfig, RealisGameApiConfig, SessionConfig,
    StakerStatus, StakingConfig, SudoConfig, SystemConfig,
    TechnicalCommitteeConfig, MAX_NOMINATIONS,
    pallet_staking, realis_game_api, Runtime
};
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_core::sr25519;
use sp_runtime::Perbill;
pub use node_primitives::{AccountId, Balance, Signature};
use node_runtime::constants::currency::DOLLARS;
pub use node_runtime::GenesisConfig;
use crate::chain_spec::{session_keys, get_account_id_from_seed};

/// Helper function to create GenesisConfig for testing
pub fn testnet_genesis(
    initial_authorities: Vec<(
        AccountId,
        AccountId,
        GrandpaId,
        BabeId,
        ImOnlineId,
        AuthorityDiscoveryId,
    )>,
    initial_nominators: Vec<AccountId>,
    root_key: AccountId,
    nft_master: Vec<AccountId>,
    api_master: Vec<AccountId>,
    white_list: Vec<AccountId>,
    bridge_master: Vec<AccountId>,
    endowed_accounts: Option<Vec<AccountId>>,
    id: ParaId,
) -> GenesisConfig {
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
        ]
    });
    // endow all authorities and nominators.
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

    let num_endowed_accounts = endowed_accounts.len();

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
        democracy: DemocracyConfig::default(),
        // elections: ElectionsConfig {
        //     members: endowed_accounts
        //         .iter()
        //         .take((num_endowed_accounts + 1) / 2)
        //         .cloned()
        //         .map(|member| (member, STASH))
        //         .collect(),
        // },
        council: CouncilConfig::default(),
        technical_committee: TechnicalCommitteeConfig {
            members: endowed_accounts
                .iter()
                .take((num_endowed_accounts + 1) / 2)
                .cloned()
                .collect(),
            phantom: Default::default(),
        },
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
        parachain_info: parachain_runtime::ParachainInfoConfig { parachain_id: id },
        parachain_system: Default::default(),
        technical_membership: Default::default(),
        treasury: Default::default(),
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
}
