use cumulus_primitives_core::ParaId;
use hex_literal::hex;
use parachain_template_runtime::{AccountId, AuraId, BeefyId, Signature, EXISTENTIAL_DEPOSIT};
use sc_chain_spec::{ChainSpecExtension, ChainSpecGroup};
use sc_service::ChainType;
use serde::{Deserialize, Serialize};
use sp_core::{sr25519, ByteArray, Pair, Public};
use sp_runtime::traits::{IdentifyAccount, Verify};

/// Specialized `ChainSpec` for the normal parachain runtime.
pub type ChainSpec =
	sc_service::GenericChainSpec<parachain_template_runtime::GenesisConfig, Extensions>;

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
}

impl RelayChain {
	pub fn name(&self) -> &'static str {
		match self {
			RelayChain::Kusama => "SORA Kusama",
			RelayChain::Rococo => "SORA Rococo",
		}
	}

	pub fn id(&self) -> &'static str {
		match self {
			RelayChain::Kusama => "sora_kusama",
			RelayChain::Rococo => "sora_rococo",
		}
	}

	pub fn root_key(&self) -> AccountId {
		let bytes = match self {
			RelayChain::Kusama => {
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
			RelayChain::Kusama => vec![
				hex!("ac0ad7c17a14833a42f8a282cd0715868c6b2680827e47b158474fdefd82e164"),
				hex!("f043af25b769db28c9f9ca876e8d55b4a5a7d634b1b30b2e5e796666f65cb24a"),
			],
			RelayChain::Rococo => vec![
				hex!("caeedb2ddad0aca6d587dd24422ab8f6281a5b2495eb5d30265294cb29238567"),
				hex!("3617852ccd789ce50f10d7843542964c71e8e08ef2977c1af3435eaabaca1521"),
			],
		};
		public_keys
			.into_iter()
			.map(|x| {
				(
					AccountId::from(x),
					(AuraId::from_slice(&x).unwrap(), BeefyId::from_slice(&x).unwrap()),
				)
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
			RelayChain::Rococo => "rococo",
		}
	}
}

/// Generate collator keys from seed.
///
/// This function's return type must always match the session keys of the chain in tuple format.
pub fn get_collator_keys_from_seed(seed: &str) -> (AuraId, BeefyId) {
	(get_public_from_seed::<AuraId>(seed), get_public_from_seed::<BeefyId>(seed))
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
) -> parachain_template_runtime::SessionKeys {
	parachain_template_runtime::SessionKeys { aura, beefy }
}

pub fn kusama_chain_spec() -> Result<ChainSpec, String> {
	ChainSpec::from_json_bytes(&include_bytes!("../res/kusama.json")[..])
}

pub fn rococo_chain_spec() -> Result<ChainSpec, String> {
	ChainSpec::from_json_bytes(&include_bytes!("../res/rococo.json")[..])
}

pub fn raw_config(relay_chain: RelayChain) -> ChainSpec {
	// Give your base currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "XOR".into());
	properties.insert("tokenDecimals".into(), 18u64.into());
	properties.insert("ss58Format".into(), parachain_template_runtime::SS58Prefix::get().into());
	let root_key = relay_chain.root_key();
	let session_keys = relay_chain.session_keys();
	let endowed_accounts = relay_chain.endowed_accounts();
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
				2011u32.into(),
			)
		},
		Vec::new(),
		None,
		Some(relay_chain.id()),
		None,
		Some(properties),
		Extensions { relay_chain: relay_chain.relay_chain().to_owned(), para_id: 2011 },
	)
}

pub fn development_config() -> ChainSpec {
	// Give your base currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "UNIT".into());
	properties.insert("tokenDecimals".into(), 12.into());
	properties.insert("ss58Format".into(), 42.into());

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
				1000.into(),
			)
		},
		Vec::new(),
		None,
		None,
		None,
		None,
		Extensions {
			relay_chain: "rococo-local".into(), // You MUST set this to the correct network!
			para_id: 1000,
		},
	)
}

pub fn local_testnet_config() -> ChainSpec {
	// Give your base currency a unit name and decimal places
	let mut properties = sc_chain_spec::Properties::new();
	properties.insert("tokenSymbol".into(), "UNIT".into());
	properties.insert("tokenDecimals".into(), 12.into());
	properties.insert("ss58Format".into(), 42.into());

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
				],
				1000.into(),
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
			para_id: 1000,
		},
	)
}

fn testnet_genesis(
	root_key: AccountId,
	invulnerables: Vec<(AccountId, (AuraId, BeefyId))>,
	endowed_accounts: Vec<AccountId>,
	id: ParaId,
) -> parachain_template_runtime::GenesisConfig {
	parachain_template_runtime::GenesisConfig {
		substrate_bridge_inbound_channel: Default::default(),
		substrate_bridge_outbound_channel: Default::default(),
		system: parachain_template_runtime::SystemConfig {
			code: parachain_template_runtime::WASM_BINARY
				.expect("WASM binary was not build, please build it!")
				.to_vec(),
		},
		balances: parachain_template_runtime::BalancesConfig {
			balances: endowed_accounts.iter().cloned().map(|k| (k, 1 << 60)).collect(),
		},
		parachain_info: parachain_template_runtime::ParachainInfoConfig { parachain_id: id },
		collator_selection: parachain_template_runtime::CollatorSelectionConfig {
			invulnerables: invulnerables.iter().cloned().map(|(acc, _)| acc).collect(),
			candidacy_bond: EXISTENTIAL_DEPOSIT * 16,
			..Default::default()
		},
		session: parachain_template_runtime::SessionConfig {
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
		polkadot_xcm: parachain_template_runtime::PolkadotXcmConfig {
			safe_xcm_version: Some(SAFE_XCM_VERSION),
		},
		sudo: parachain_template_runtime::SudoConfig { key: Some(root_key) },
	}
}
