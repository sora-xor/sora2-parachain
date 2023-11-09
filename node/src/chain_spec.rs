use bridge_types::SubNetworkId;
use cumulus_primitives_core::ParaId;
use hex_literal::hex;
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use sc_service::ChainType;
use serde::{Deserialize, Serialize};
use sora2_parachain_runtime::{
    AccountId, AuraId, BeefyId, BeefyLightClientConfig, CouncilConfig, DemocracyConfig, Signature,
    TechnicalCommitteeConfig, EXISTENTIAL_DEPOSIT,
};
use sp_core::{sr25519, ByteArray, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec =
    sc_service::GenericChainSpec<sora2_parachain_runtime::GenesisConfig, Extensions>;

/// The default XCM version to set in genesis config.
const SAFE_XCM_VERSION: u32 = xcm::prelude::XCM_VERSION;

/// Helper function to generate a crypto pair from seed
pub fn get_public_from_seed<TPublic: Public>(seed: &str) -> <TPublic::Pair as Pair>::Public {
    TPublic::Pair::from_string(&format!("//{}", seed), None)
        .expect("static values are valid; qed")
        .public()
}

/// The extensions for the [`ChainSpec`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, ChainSpecGroup, ChainSpecExtension)]
#[serde(deny_unknown_fields)]
pub struct Extensions {
    /// The relay chain of the Parachain.
    pub relay_chain: String,
    /// The id of the Parachain.
    pub para_id: u32,
}

impl Extensions {
    /// Try to get the extension from the given `ChainSpec`.
    pub fn try_get(chain_spec: &dyn sc_service::ChainSpec) -> Option<&Self> {
        sc_chain_spec::get_extension(chain_spec.extensions())
    }
}

type AccountPublic = <Signature as Verify>::Signer;

pub enum RelayChain {
    Kusama,
    Rococo,
    Polkadot,
}

pub fn dummy_beefy_id_from_account_id(a: AccountId) -> BeefyId {
    let mut id_raw = [0u8; 33];

    // NOTE: AccountId is 32 bytes, whereas BeefyId is 33 bytes.
    id_raw[1..].copy_from_slice(a.as_ref());
    id_raw[0..4].copy_from_slice(b"beef");

    sp_core::ecdsa::Public(id_raw).into()
}

impl RelayChain {
    pub fn name(&self) -> &'static str {
        match self {
            RelayChain::Kusama => "SORA Kusama",
            RelayChain::Rococo => "SORA Rococo",
            RelayChain::Polkadot => "SORA Polkadot",
        }
    }

    pub fn id(&self) -> &'static str {
        match self {
            RelayChain::Kusama => "sora_kusama",
            RelayChain::Rococo => "sora_rococo",
            RelayChain::Polkadot => "sora_polkadot",
        }
    }

    pub fn root_key(&self) -> AccountId {
        let bytes = match self {
            RelayChain::Kusama => {
                hex!("de5ef29355f16efa342542cd7567bebd371b3e80dd33aee99cc50cb484688058")
            },
            RelayChain::Polkadot => {
                hex!("de5ef29355f16efa342542cd7567bebd371b3e80dd33aee99cc50cb484688058")
            },
            RelayChain::Rococo => {
                hex!("54fd1e1728cd833d21da6f3e36c50884062e35edfc24aec7a70c18a60451255a")
            },
        };
        AccountId::from(bytes)
    }

    pub fn session_keys(&self) -> Vec<(AccountId, (AuraId, BeefyId))> {
        let public_keys = match self {
            RelayChain::Polkadot => vec![
                (
                    hex!("74ca689ef5d95cf72b47adba0cf9080ddab33c71d3e838ed2aa19cd8e2f6014c"),
                    Some(hex!(
                        "035e87f3efbe033675c9451fbfbfd7d8777cbd6b3f48570a617366444f012c6a1a"
                    )),
                ),
                (
                    hex!("56ca3b3b5104baa59e213ab20c6d531702828081833adf0f79d60afa81380462"),
                    Some(hex!(
                        "039a57dc9b4ea068c2da95462b7230399058a1de70bfada9480d2e4eece4cef2ad"
                    )),
                ),
            ],
            RelayChain::Kusama => vec![
                (hex!("ac0ad7c17a14833a42f8a282cd0715868c6b2680827e47b158474fdefd82e164"), None),
                (hex!("f043af25b769db28c9f9ca876e8d55b4a5a7d634b1b30b2e5e796666f65cb24a"), None),
            ],
            RelayChain::Rococo => vec![
                (hex!("caeedb2ddad0aca6d587dd24422ab8f6281a5b2495eb5d30265294cb29238567"), None),
                (hex!("3617852ccd789ce50f10d7843542964c71e8e08ef2977c1af3435eaabaca1521"), None),
            ],
        };
        public_keys
            .into_iter()
            .map(|(sr25519, ecdsa)| {
                if let Some(ecdsa) = ecdsa {
                    (
                        AccountId::from(sr25519),
                        (
                            AuraId::from_slice(&sr25519).unwrap(),
                            BeefyId::from_slice(&ecdsa).unwrap(),
                        ),
                    )
                } else {
                    log::warn!("No Beefy session key for this relaychain. Using dummy key.");
                    (
                        AccountId::from(sr25519),
                        (
                            AuraId::from_slice(&sr25519).unwrap(),
                            dummy_beefy_id_from_account_id(AccountId::from(sr25519)),
                        ),
                    )
                }
            })
            .collect()
    }

    pub fn endowed_accounts(&self) -> Vec<AccountId> {
        std::iter::once(self.root_key())
            .chain(self.session_keys().into_iter().map(|x| x.0))
            .collect()
    }

    pub fn relay_chain(&self) -> &'static str {
        match self {
            RelayChain::Kusama => "kusama",
            RelayChain::Polkadot => "polkadot",
            RelayChain::Rococo => "rococo",
        }
    }

    pub fn bridge_network_id(&self) -> SubNetworkId {
        match self {
            RelayChain::Kusama => SubNetworkId::Kusama,
            RelayChain::Polkadot => SubNetworkId::Polkadot,
            RelayChain::Rococo => SubNetworkId::Rococo,
        }
    }
}

/// Generate collator keys from seed.
///
/// This function's return type must always match the session keys of the chain in tuple format.
pub fn get_collator_keys_from_seed(seed: &str) -> (AuraId, BeefyId) {
    (get_public_from_seed::<AuraId>(seed), get_public_from_seed::<BeefyId>(seed))
}

pub fn authority_keys_from_public_keys(
    collator_address: [u8; 32],
    sr25519_key: [u8; 32],
    ecdsa_key: [u8; 33],
) -> (AccountId, (AuraId, BeefyId)) {
    (
        collator_address.into(),
        (AuraId::from_slice(&sr25519_key).unwrap(), BeefyId::from_slice(&ecdsa_key).unwrap()),
    )
}

/// Helper function to generate an account ID from seed
pub fn get_account_id_from_seed<TPublic: Public>(seed: &str) -> AccountId
where
    AccountPublic: From<<TPublic::Pair as Pair>::Public>,
{
    AccountPublic::from(get_public_from_seed::<TPublic>(seed)).into_account()
}

/// Generate the session keys from individual elements.
///
/// The input must be a tuple of individual keys (a single arg for now since we have just one key).
pub fn template_session_keys(
    (aura, beefy): (AuraId, BeefyId),
) -> sora2_parachain_runtime::SessionKeys {
    sora2_parachain_runtime::SessionKeys { aura, beefy }
}

pub fn kusama_chain_spec() -> Result<ChainSpec, String> {
    ChainSpec::from_json_bytes(&include_bytes!("../res/kusama.json")[..])
}

pub fn polkadot_chain_spec() -> Result<ChainSpec, String> {
    ChainSpec::from_json_bytes(&include_bytes!("../res/polkadot.json")[..])
}

pub fn rococo_chain_spec() -> Result<ChainSpec, String> {
    ChainSpec::from_json_bytes(&include_bytes!("../res/rococo.json")[..])
}

pub fn coded_config(relay_chain: RelayChain, para_id: u32) -> ChainSpec {
    // Give your base currency a unit name and decimal places
    let mut properties = sc_chain_spec::Properties::new();
    properties.insert("tokenSymbol".into(), "XOR".into());
    properties.insert("tokenDecimals".into(), 18u64.into());
    properties.insert("ss58Format".into(), sora2_parachain_runtime::SS58Prefix::get().into());
    let root_key = relay_chain.root_key();
    let session_keys = relay_chain.session_keys();
    let endowed_accounts = relay_chain.endowed_accounts();
    let bridge_network_id = relay_chain.bridge_network_id();
    ChainSpec::from_genesis(
        // Name
        relay_chain.name(),
        // ID
        relay_chain.id(),
        ChainType::Live,
        move || {
            testnet_genesis(
                root_key.clone(),
                session_keys.clone(),
                endowed_accounts.clone(),
                para_id.into(),
                bridge_network_id,
                vec![],
                vec![],
            )
        },
        Vec::new(),
        None,
        Some(relay_chain.id()),
        None,
        Some(properties),
        Extensions { relay_chain: relay_chain.relay_chain().to_owned(), para_id },
    )
}

pub fn development_config() -> ChainSpec {
    // Give your base currency a unit name and decimal places
    let mut properties = sc_chain_spec::Properties::new();
    properties.insert("tokenSymbol".into(), "XOR".into());
    properties.insert("tokenDecimals".into(), 18.into());
    properties.insert("ss58Format".into(), sora2_parachain_runtime::SS58Prefix::get().into());

    ChainSpec::from_genesis(
        // Name
        "Development",
        // ID
        "dev",
        ChainType::Development,
        move || {
            testnet_genesis(
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                // initial collators.
                vec![
                    (
                        get_account_id_from_seed::<sr25519::Public>("Alice"),
                        get_collator_keys_from_seed("Alice"),
                    ),
                    (
                        get_account_id_from_seed::<sr25519::Public>("Bob"),
                        get_collator_keys_from_seed("Bob"),
                    ),
                ],
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
                ],
                2011.into(),
                SubNetworkId::Rococo,
                vec![
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                    get_account_id_from_seed::<sr25519::Public>("Bob"),
                    get_account_id_from_seed::<sr25519::Public>("Charlie"),
                    get_account_id_from_seed::<sr25519::Public>("Dave"),
                ],
                vec![],
            )
        },
        Vec::new(),
        None,
        None,
        None,
        None,
        Extensions {
            relay_chain: "rococo-local".into(), // You MUST set this to the correct network!
            para_id: 2011,
        },
    )
}

pub fn local_testnet_config() -> ChainSpec {
    // Give your base currency a unit name and decimal places
    let mut properties = sc_chain_spec::Properties::new();
    properties.insert("tokenSymbol".into(), "XOR".into());
    properties.insert("tokenDecimals".into(), 18.into());
    properties.insert("ss58Format".into(), sora2_parachain_runtime::SS58Prefix::get().into());

    ChainSpec::from_genesis(
        // Name
        "Local Testnet",
        // ID
        "local_testnet",
        ChainType::Local,
        move || {
            testnet_genesis(
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                // initial collators.
                vec![
                    (
                        get_account_id_from_seed::<sr25519::Public>("Alice"),
                        get_collator_keys_from_seed("Alice"),
                    ),
                    (
                        get_account_id_from_seed::<sr25519::Public>("Bob"),
                        get_collator_keys_from_seed("Bob"),
                    ),
                    (
                        get_account_id_from_seed::<sr25519::Public>("Charlie"),
                        get_collator_keys_from_seed("Charlie"),
                    ),
                ],
                vec![
                    AccountId::from(hex!(
                        "e02b00cb5bbf5c0338075237cdbfb7d11dbaf19aafce71744610b6a87b5e0f22"
                    )),
                    AccountId::from(hex!(
                        "caeedb2ddad0aca6d587dd24422ab8f6281a5b2495eb5d30265294cb29238567"
                    )),
                    AccountId::from(hex!(
                        "3617852ccd789ce50f10d7843542964c71e8e08ef2977c1af3435eaabaca1521"
                    )),
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                    get_account_id_from_seed::<sr25519::Public>("Bob"),
                    get_account_id_from_seed::<sr25519::Public>("Charlie"),
                ],
                2011.into(),
                SubNetworkId::Rococo,
                vec![
                    AccountId::from(hex!(
                        "e02b00cb5bbf5c0338075237cdbfb7d11dbaf19aafce71744610b6a87b5e0f22"
                    )),
                    AccountId::from(hex!(
                        "caeedb2ddad0aca6d587dd24422ab8f6281a5b2495eb5d30265294cb29238567"
                    )),
                    AccountId::from(hex!(
                        "3617852ccd789ce50f10d7843542964c71e8e08ef2977c1af3435eaabaca1521"
                    )),
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                    get_account_id_from_seed::<sr25519::Public>("Bob"),
                    get_account_id_from_seed::<sr25519::Public>("Charlie"),
                ],
                vec![],
            )
        },
        // Bootnodes
        Vec::new(),
        // Telemetry
        None,
        // Protocol ID
        Some("template-local"),
        // Fork ID
        None,
        // Properties
        Some(properties),
        // Extensions
        Extensions {
            relay_chain: "rococo-local".into(), // You MUST set this to the correct network!
            para_id: 2011,
        },
    )
}

/// Config for docker-compose based local testnet
pub fn docker_local_testnet_config() -> ChainSpec {
    // Give your base currency a unit name and decimal places
    let mut properties = sc_chain_spec::Properties::new();
    properties.insert("tokenSymbol".into(), "XOR".into());
    properties.insert("tokenDecimals".into(), 18.into());
    properties.insert("ss58Format".into(), sora2_parachain_runtime::SS58Prefix::get().into());

    ChainSpec::from_genesis(
        // Name
        "Local Testnet",
        // ID
        "local_testnet",
        ChainType::Local,
        move || {
            testnet_genesis(
                get_account_id_from_seed::<sr25519::Public>("Alice"),
                // initial collators.
                vec![(
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                    get_collator_keys_from_seed("Alice"),
                )],
                vec![
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                    get_account_id_from_seed::<sr25519::Public>("Bob"),
                    get_account_id_from_seed::<sr25519::Public>("Charlie"),
                    get_account_id_from_seed::<sr25519::Public>("Dave"),
                    get_account_id_from_seed::<sr25519::Public>("Eve"),
                    get_account_id_from_seed::<sr25519::Public>("Relayer"),
                ],
                2011.into(),
                SubNetworkId::Rococo,
                vec![
                    get_account_id_from_seed::<sr25519::Public>("Alice"),
                    get_account_id_from_seed::<sr25519::Public>("Bob"),
                    get_account_id_from_seed::<sr25519::Public>("Charlie"),
                    get_account_id_from_seed::<sr25519::Public>("Dave"),
                ],
                vec![],
            )
        },
        // Bootnodes
        Vec::new(),
        // Telemetry
        None,
        // Protocol ID
        Some("template-local"),
        // Fork ID
        None,
        // Properties
        Some(properties),
        // Extensions
        Extensions {
            relay_chain: "rococo-local".into(), // You MUST set this to the correct network!
            para_id: 2011,
        },
    )
}

/// Config for bridge private testnet
pub fn bridge_dev_config() -> ChainSpec {
    // Give your base currency a unit name and decimal places
    let mut properties = sc_chain_spec::Properties::new();
    properties.insert("tokenSymbol".into(), "XOR".into());
    properties.insert("tokenDecimals".into(), 18.into());
    properties.insert("ss58Format".into(), sora2_parachain_runtime::SS58Prefix::get().into());

    ChainSpec::from_genesis(
        // Name
        "Local Testnet",
        // ID
        "local_testnet",
        ChainType::Local,
        move || {
            testnet_genesis(
                hex!("f6d0e31012ebeef4b9cc4cddd0593a8579d226dc17ce725139225e81683f0143").into(),
                // initial collators.
                vec![
                    authority_keys_from_public_keys(
                        // scheme: sr25519, seed: <seed>//parachain-collator-1
                        hex!("9232d7e4f6b7e1a881346c92f63d65e0a0ce6def5170e9766ed9d1001ed27e5d"),
                        // scheme: sr25519, seed: <seed>//parachain-collator-1
                        hex!("9232d7e4f6b7e1a881346c92f63d65e0a0ce6def5170e9766ed9d1001ed27e5d"),
                        // scheme: ecdsa, seed: <seed>//parachain-collator-1
                        hex!("035cb006bff18dcad55c58409b1cdc31be223b1340596c53f3704859fa0057469d"),
                    ),
                    authority_keys_from_public_keys(
                        // scheme: sr25519, seed: <seed>//parachain-collator-2
                        hex!("54036a0a47f28c64885fd8f0300c8ff436c3007e6b5ef70d4c3f1d3ee8856f5c"),
                        // scheme: sr25519, seed: <seed>//parachain-collator-2
                        hex!("54036a0a47f28c64885fd8f0300c8ff436c3007e6b5ef70d4c3f1d3ee8856f5c"),
                        // scheme: ecdsa, seed: <seed>//parachain-collator-2
                        hex!("03c9a2529b1b857aa620a6b908de3aded3503d4549dd36ecfa79f800ae46ca0c9a"),
                    ),
                ],
                vec![
                    hex!("f6d0e31012ebeef4b9cc4cddd0593a8579d226dc17ce725139225e81683f0143").into(),
                    hex!("9232d7e4f6b7e1a881346c92f63d65e0a0ce6def5170e9766ed9d1001ed27e5d").into(),
                    hex!("54036a0a47f28c64885fd8f0300c8ff436c3007e6b5ef70d4c3f1d3ee8856f5c").into(),
                    hex!("7a2eaa9ba604e1c6575c0cada3e50155f98cd625566ea7239577c9565236662a").into(),
                    hex!("ac95d7df7e9f61b82654a0a9c93a74cca9182062a89aff6f0545f09ab9c1a152").into(),
                    hex!("4c6018b95a633613714b65201f9f41fb8901be88dc5fb0053fc3c10c02ddad33").into(),
                    hex!("306c3e7cb2075ca8cd7d3a5ec71e6f51e3fa6e8e550dce003a1c39167e0f317b").into(),
                    hex!("328be9c672c4fff8ae9065ebdf116a47e1121933616a1d1749ff9bb3356fd542").into(),
                ],
                2011.into(),
                SubNetworkId::Rococo,
                vec![
                    hex!("f6d0e31012ebeef4b9cc4cddd0593a8579d226dc17ce725139225e81683f0143").into(),
                    hex!("9232d7e4f6b7e1a881346c92f63d65e0a0ce6def5170e9766ed9d1001ed27e5d").into(),
                    hex!("54036a0a47f28c64885fd8f0300c8ff436c3007e6b5ef70d4c3f1d3ee8856f5c").into(),
                    hex!("7a2eaa9ba604e1c6575c0cada3e50155f98cd625566ea7239577c9565236662a").into(),
                ],
                vec![],
            )
        },
        // Bootnodes
        Vec::new(),
        // Telemetry
        None,
        // Protocol ID
        Some("template-local"),
        // Fork ID
        None,
        // Properties
        Some(properties),
        // Extensions
        Extensions {
            relay_chain: "rococo-local".into(), // You MUST set this to the correct network!
            para_id: 2011,
        },
    )
}

fn testnet_genesis(
    root_key: AccountId,
    invulnerables: Vec<(AccountId, (AuraId, BeefyId))>,
    endowed_accounts: Vec<AccountId>,
    id: ParaId,
    bridge_network_id: SubNetworkId,
    technical_committee_accounts: Vec<AccountId>,
    council_accounts: Vec<AccountId>,
) -> sora2_parachain_runtime::GenesisConfig {
    sora2_parachain_runtime::GenesisConfig {
        beefy_light_client: BeefyLightClientConfig { network_id: bridge_network_id },
        substrate_bridge_outbound_channel: Default::default(),
        system: sora2_parachain_runtime::SystemConfig {
            code: sora2_parachain_runtime::WASM_BINARY
                .expect("WASM binary was not build, please build it!")
                .to_vec(),
        },
        balances: sora2_parachain_runtime::BalancesConfig {
            balances: endowed_accounts.iter().cloned().map(|k| (k, 1 << 60)).collect(),
        },
        parachain_info: sora2_parachain_runtime::ParachainInfoConfig { parachain_id: id },
        collator_selection: sora2_parachain_runtime::CollatorSelectionConfig {
            invulnerables: invulnerables.iter().cloned().map(|(acc, _)| acc).collect(),
            candidacy_bond: EXISTENTIAL_DEPOSIT * 16,
            ..Default::default()
        },
        session: sora2_parachain_runtime::SessionConfig {
            keys: invulnerables
                .into_iter()
                .map(|(acc, keys)| {
                    (
                        acc.clone(),                 // account id
                        acc,                         // validator id
                        template_session_keys(keys), // session keys
                    )
                })
                .collect(),
        },
        // no need to pass anything to aura, in fact it will panic if we do. Session will take care
        // of this.
        aura: Default::default(),
        aura_ext: Default::default(),
        beefy: Default::default(),
        parachain_system: Default::default(),
        polkadot_xcm: sora2_parachain_runtime::PolkadotXcmConfig {
            safe_xcm_version: Some(SAFE_XCM_VERSION),
        },
        sudo: sora2_parachain_runtime::SudoConfig { key: Some(root_key) },
        technical_committee: TechnicalCommitteeConfig {
            members: technical_committee_accounts,
            phantom: Default::default(),
        },
        council: CouncilConfig { members: council_accounts, phantom: Default::default() },
        democracy: DemocracyConfig::default(),
    }
}
