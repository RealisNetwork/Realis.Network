
use grandpa_primitives::AuthorityId as GrandpaId;
use hex_literal::hex;
use node_runtime::{
    wasm_binary_unwrap, AuthorityDiscoveryConfig, BabeConfig,
    BalancesConfig, /*CouncilConfig,*/
    /*DemocracyConfig,*/ /*ElectionsConfig,*/ GrandpaConfig, ImOnlineConfig, IndicesConfig,
    NftConfig, RealisBridgeConfig, RealisGameApiConfig, SessionConfig,
    /*SocietyConfig,*/ StakerStatus, StakingConfig, SudoConfig, SystemConfig,
    /*TechnicalCommitteeConfig,*/ MAX_NOMINATIONS,
};
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_runtime::{
    Perbill,
};

pub use node_primitives::{AccountId, Balance, Signature};
use node_runtime::constants::currency::DOLLARS;
use node_runtime::pallet_staking;
use node_runtime::realis_game_api;
use node_runtime::Runtime;
pub use node_runtime::{Block, GenesisConfig};
use crate::chain_spec::{ChainSpec, session_keys};

///Realis chain-spec from realis.json
pub fn realis_config() -> Result<ChainSpec, String> {
    ChainSpec::from_json_bytes(&include_bytes!("../../../../../docker/realis.json")[..])
}

///Realis chain-spec
pub fn realis_genesis(
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
) -> GenesisConfig {
    let mut endowed_accounts: Vec<AccountId> = endowed_accounts.unwrap_or_else(|| {
        vec![
            hex!["781f4331933557680355932ef7f39b88e938fcb4338cc0e03edb3c523e47fd09"].into(),
            hex!["fe8823fb870f61eed24638a228adbe5885de6e945bb1375ca6a7415a4824756e"].into(),
            hex!["bc95bdafa3582b0ecbf5caf1e30b00412fa7c2dfbccd518f3b842c63890cc979"].into(),
            hex!["08bdc3547dc26a647391b509960b00adafa550496e9a95339a2faa02343be20f"].into(),
            hex!["d4c2ffb1322efb7fe78463ad6f24301751454685edd96640197cab2c44e1b16c"].into(),
            hex!["10f908b91793b30fc4870e255a0e102745e2a8f268814cd28389ba7f4220764d"].into(),
            hex!["d671cde125c8b7f42afbf40fb9d0d93d4d80c888cd34824c99ab292b589dbe75"].into(),
            hex!["d4c2ffb1322efb7fe78463ad6f24301751454685edd96640197cab2c44e1b16c"].into(),
            pallet_staking::Pallet::<Runtime>::account_id(),
            realis_game_api::Pallet::<Runtime>::account_id(),
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
    let _num_endowed_accounts = endowed_accounts.len();

    const ENDOWMENT: Balance = 900_000 * DOLLARS / 12 * 100;
    const GAME_WALLET: Balance = 10_000_000 * DOLLARS / 10;
    const STAKING_POOL: Balance = 30_000_000 * DOLLARS / 10;
    const STASH: Balance = ENDOWMENT / 1000;

    let pallet_id_staking = pallet_staking::Pallet::<Runtime>::account_id().clone();
    let game_wallet = realis_game_api::Pallet::<Runtime>::account_id().clone();

    GenesisConfig {
        system: SystemConfig {
            code: wasm_binary_unwrap().to_vec(),
            // changes_trie_config: Default::default(),
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
}
