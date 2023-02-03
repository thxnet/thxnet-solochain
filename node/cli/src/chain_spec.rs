// This file is part of Substrate.

// Copyright (C) 2018-2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later WITH Classpath-exception-2.0

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Substrate chain configurations.

use grandpa_primitives::AuthorityId as GrandpaId;
use kitchensink_runtime::{
	constants::currency::*, wasm_binary_unwrap, AuthorityDiscoveryConfig, BabeConfig,
	BalancesConfig, Block, CouncilConfig, DemocracyConfig, ElectionsConfig, GrandpaConfig,
	ImOnlineConfig, IndicesConfig, MaxNominations, NominationPoolsConfig, SessionConfig,
	SessionKeys, SocietyConfig, StakerStatus, StakingConfig, SudoConfig, SystemConfig,
	TechnicalCommitteeConfig,
};
use pallet_im_online::sr25519::AuthorityId as ImOnlineId;
use sc_chain_spec::ChainSpecExtension;
use sc_service::{ChainType, Properties};
use sc_telemetry::TelemetryEndpoints;
use serde::{Deserialize, Serialize};
use sp_authority_discovery::AuthorityId as AuthorityDiscoveryId;
use sp_consensus_babe::AuthorityId as BabeId;
use sp_core::{crypto::UncheckedInto, sr25519, Pair, Public};
use sp_runtime::{
	traits::{IdentifyAccount, Verify},
	Perbill,
};

pub use kitchensink_runtime::GenesisConfig;
pub use node_primitives::{AccountId, Balance, Signature};

type AccountPublic = <Signature as Verify>::Signer;

const STAGING_TELEMETRY_URL: &str = "wss://telemetry.polkadot.io/submit/";

/// Node `ChainSpec` extensions.
///
/// Additional parameters for some Substrate core modules,
/// customizable from the chain spec.
#[derive(Default, Clone, Serialize, Deserialize, ChainSpecExtension)]
#[serde(rename_all = "camelCase")]
pub struct Extensions {
	/// Block numbers with known hashes.
	pub fork_blocks: sc_client_api::ForkBlocks<Block>,
	/// Known bad block hashes.
	pub bad_blocks: sc_client_api::BadBlocks<Block>,
	/// The light sync state extension used by the sync-state rpc.
	pub light_sync_state: sc_sync_state_rpc::LightSyncStateExtension,
}

/// Specialized `ChainSpec`.
pub type ChainSpec = sc_service::GenericChainSpec<GenesisConfig, Extensions>;
/// Flaming Fir testnet generator
pub fn flaming_fir_config() -> Result<ChainSpec, String> {
	ChainSpec::from_json_bytes(&include_bytes!("../res/flaming-fir.json")[..])
}

fn session_keys(
	grandpa: GrandpaId,
	babe: BabeId,
	im_online: ImOnlineId,
	authority_discovery: AuthorityDiscoveryId,
) -> SessionKeys {
	SessionKeys { grandpa, babe, im_online, authority_discovery }
}

fn staging_testnet_config_genesis() -> GenesisConfig {
	#[rustfmt::skip]
	// stash, controller, session-key
	// generated with secret:
	// for i in 1 2 3 4 ; do for j in stash controller; do subkey inspect "$secret"/fir/$j/$i; done; done
	//
	// and
	//
	// for i in 1 2 3 4 ; do for j in session; do subkey --ed25519 inspect "$secret"//fir//$j//$i; done; done

	let initial_authorities: Vec<(
		AccountId,
		AccountId,
		GrandpaId,
		BabeId,
		ImOnlineId,
		AuthorityDiscoveryId,
	)> = vec![
		(
			// 5GBhyjJRZ5kzfq97cPcnxJtFnnpcJ6Q8Q4wkUgJ3A1jNEJ8q
			array_bytes::hex_n_into_unchecked("b6480fbd53b0563756a36122d40f051ce6c510cefd41197ce954119e25a6f44f"),
			// 5GE2GYsRdHsvYAFwoLcnBnk4nm1TsHngXfhChSGvvN3SkEoY
			array_bytes::hex_n_into_unchecked("b80c1f36b8d51e5e97ee642d923dfde919ab5969aeafb000641fc3faa560d748"),
			// 5GRQysEwRfLxmLWN6saMCQTVxkVh8mosCrwDE4t6FCtLz2U6
			array_bytes::hex2array_unchecked("c0bc472a34ffca44275e95237fc6dba328f8826291cd5375cd72c5f4c1444f68").unchecked_into(),
			// 5CrFuPn2Nro625isWiZKoGqFhYnbiMPstLS4jAs8ReH34Hp6
			array_bytes::hex2array_unchecked("22bdec45fa533b4d3495e9bcbdcc61f172207cdc263acf732edd9623ea4cc113").unchecked_into(),
			// 5FFYBJVJuRXcEB64PtvXLSpbTZevMyRTkCUzcjPy6EjWP7DY
			array_bytes::hex2array_unchecked("8cf7d17ba5ac3fee86f0866d3515f31f4af3755f67792f6381143eae9a792101").unchecked_into(),
			// 5Dek6oPsPErg47UEMQQcUt71S71ZzNLS2GZQbceHM21NQvoP
			array_bytes::hex2array_unchecked("46322148b8f5f5c122f3707e8892f73f0c8a7ab5c1d1e2bb7b280e7c822e4139").unchecked_into(),
		),
		(
			// 5DaWozPaVCwTiQUixznRWJWfg2rCq9mrHceCUyDfiUAAtXmZ
			array_bytes::hex_n_into_unchecked("42f8692be17acdfafd6119b65e8cc84eaa2e30e1ae755ed45c1cc87925cc615d"),
			// 5EAE95xyfp5qoPPHSb2NgjX8DqJt2AHWbdemgYLzFYuUucvL
			array_bytes::hex_n_into_unchecked("5caea2c42e419788bdc375f77a86cb274173ed5ee784e0cdcda438af0d9af436"),
			// 5GLoC45VT39Emw4QgiFQ7iJLdBWCqbo5QWsBgxBksQE9GKxf
			array_bytes::hex2array_unchecked("bd36d11f92b5bd79f3c3d9866257f89dda667c709168dd5106b5455116c03b67").unchecked_into(),
			// 5GrvoHFBME7jHJQWAkHpz6xvQ6AjHMiRfzskPFyReRuGSwH2
			array_bytes::hex2array_unchecked("d431c9a59eaa037fa2a8ea5c15110131858f32d90f56c34ff795e12b2309c12a").unchecked_into(),
			// 5CS7HKDyfCmC3woskZ4jpXtyLUBLyeW1Hn5cBJBK4SnQaJB7
			array_bytes::hex2array_unchecked("10530244451a03a4a5dcf347260539f3e87486e078fe0cc15b0a1305946cf16d").unchecked_into(),
			// 5Co43xKV97dAAT6FX28wArVr1CVjz3MPUtF6ZNJM4Cf6rabv
			array_bytes::hex2array_unchecked("204c49bb123a0e032c5df3146ba6696c12f658afdf0be25ae15846ea82d74f43").unchecked_into(),
		),
		(
			// 5EHoqEggMMgb3xhtLbuweyYrmWA9tjrARQNeAZiffYBpMGNu
			array_bytes::hex_n_into_unchecked("6276c597f9954cba5217b2468e33a26e65b86ec933e85f0f526bae708e728112"),
			// 5ECTgrwvrrfWrb4J15Cc1DaZFcHCmahnZmoCispeheea3e3K
			array_bytes::hex_n_into_unchecked("5e62bb960261e2c16ffe672acbfdd8778c6f7cf095e18f6416f5d106c53d932e"),
			// 5CedyemYq63SVib8z7GxHKMApMq1qLiVM7XEGi6ZA5B5D9Aw
			array_bytes::hex2array_unchecked("19e148aa935ed76b132ba38b42afe484a3dfd0f872a02a3834f676a0c074ed22").unchecked_into(),
			// 5HeNSXpweDw8Nof3A6jaqKFU9P2mzuwYBA9RR7CMN2o4UbnR
			array_bytes::hex2array_unchecked("f6da26834c6c1985f7789940c4ff272cd69403f1d3cdcd386f865229cc95d17e").unchecked_into(),
			// 5HQvBbguk3ZMmBuzcmH5Ddtkyv6ExrnizARaXv5ZqYjM7WZn
			array_bytes::hex2array_unchecked("ec9791d4afe0abed4f039378d11dca2c58c4546a241359d8433529656a3dc51d").unchecked_into(),
			// 5Enns2Q5UBUFffeQYdMrt6hJYDFN9h83H6YTjreAfVm1AYK5
			array_bytes::hex2array_unchecked("7891a0458da4debae4a7902fa265dd71639f48f43754737a9be32c9c86035624").unchecked_into(),
		),
		(
			// 5DbuAUyg6zJQZiDjuSgwxKvoqzTiYrT6cQarZ7P7V2Mdk8aK
			array_bytes::hex_n_into_unchecked("4406e77bc60e9a365bc6e4f9d7216aa9d911b906801dae87b1e0681dcd854e4e"),
			// 5Gc2Lh4cfcACqD6MrnzGbymZnh8PDr2G4NChrapix6Fuassa
			array_bytes::hex_n_into_unchecked("c8d3c2dcd08b79371452804fdfa244d642138f4a7904fb2ec16551d3917c4225"),
			// 5HStrP62VSV5FQtMUUnHYaUnF4m6ZEBrwGuCjGmdRqxA4Hfj
			array_bytes::hex2array_unchecked("ee19942c057b3f621845de8162fd54468f3cdd3b857f501b0d95b0bfb46f9ba6").unchecked_into(),
			// 5ELTgkEXvoxz54wtav4BxT63ZCfTu97tutL6YFdbEXMHG4jt
			array_bytes::hex2array_unchecked("647cb07447d1d289c4ecb0be5076370706c36fea55c35203f355933f40d81846").unchecked_into(),
			// 5EP94Cfdfu7K7mwh79CTz1JDS2dh3FEe9YRzTR6EEzFo4kjx
			array_bytes::hex2array_unchecked("6687b608a78ca96f9b7064ef4002eb9739b073549a3f90e1573e538df12fe326").unchecked_into(),
			// 5GriKDD7iCRGyDwnn5jtEcQBg55JX1Hxtv7wZMJWwPEN7Q7A
			array_bytes::hex2array_unchecked("d407c34fa128c21bbebb7d87a98454faddd48b50e3c0e49c98468ee22f756075").unchecked_into(),
		),
		(
			// 5Esv7FBVtv5YbFmVarfvRhC5ouazdXUG2BmpBZPZdkqFUGi5
			array_bytes::hex_n_into_unchecked("7c7a3e167dbd9722256eefc8055739e5f85bbc48f8df304cde4b59dbeeb6c505"),
			// 5HQndxnampS2SZGSWQJ86AWpNULuaSmzGtdj8U9hL3PiKaiq
			array_bytes::hex_n_into_unchecked("ec7e2b5d6cb8298957f6ca1a5f98be55503f66ad534eb7d792cf6f7e7377b724"),
			// 5DPYJypytm5o95wX4UQGFxgKtLbH1zdeLXFU8jhsmJCxrrXP
			array_bytes::hex2array_unchecked("3a99c2a0c3827ede05e1dc7524a638b0d37395663d773bdfc78643e190e2efdd").unchecked_into(),
			// 5HWAGu2hVdb6erJABPR6afuqKDcMFMteLnBLU6F21WSBoxQY
			array_bytes::hex2array_unchecked("f0973b80419284ae574b02fd2e6801dd7797e9ff39a772a634e54c32617c030c").unchecked_into(),
			// 5DLfPkda48dK6VSgS4AELHNVc6ZMQfPceqqdYRcT1uRmciTz
			array_bytes::hex2array_unchecked("3867dd7a2d3cb4885ad7351c1c22f944c2f831b52eebe8f5e8e1ee5d0dea8326").unchecked_into(),
			// 5H6hBYLbdZcv7RK79qQCYZg7ZbKkPyLcF9TzDhmFAmozWXf1
			array_bytes::hex2array_unchecked("deb1631a453dff4149aaff8851d59f52f15046603cb75c1e8c5e0a71da0ac272").unchecked_into(),
		),		
	];

	// generated with secret: subkey inspect "$secret"/fir
	let root_key: AccountId = array_bytes::hex_n_into_unchecked(
		// 5DRdwFbzV2TX4K6ZZkmJVviGX6BBtD2CWhUv4oAytZWdFKXV
		"3c333012f29059536abd37dd570eab4425c07b39630c0c9b0829ee3933f8cc00",
	);

	let endowed_accounts: Vec<AccountId> = vec![root_key.clone()];

	testnet_genesis(initial_authorities, vec![], root_key, Some(endowed_accounts))
}

/// Staging testnet config.
pub fn staging_testnet_config() -> ChainSpec {
	let mut properties = Properties::new();
	properties.insert("tokenSymbol".into(), "XHTDEV".into());
	properties.insert("tokenDecimals".into(), 10.into());
	properties.insert("ss58Format".into(), 42.into());

	let boot_nodes = vec![];
	ChainSpec::from_genesis(
		"The Hybrid Network Testnet",
		"thx_testnet",
		ChainType::Live,
		staging_testnet_config_genesis,
		boot_nodes,
		Some(
			TelemetryEndpoints::new(vec![(STAGING_TELEMETRY_URL.to_string(), 0)])
				.expect("Staging telemetry url is valid; qed"),
		),
		None,
		None,
		Some(properties),
		Default::default(),
	)
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
pub fn authority_keys_from_seed(
	seed: &str,
) -> (AccountId, AccountId, GrandpaId, BabeId, ImOnlineId, AuthorityDiscoveryId) {
	(
		get_account_id_from_seed::<sr25519::Public>(&format!("{}//stash", seed)),
		get_account_id_from_seed::<sr25519::Public>(seed),
		get_from_seed::<GrandpaId>(seed),
		get_from_seed::<BabeId>(seed),
		get_from_seed::<ImOnlineId>(seed),
		get_from_seed::<AuthorityDiscoveryId>(seed),
	)
}

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
	endowed_accounts: Option<Vec<AccountId>>,
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
		]
	});
	// endow all authorities and nominators.
	initial_authorities
		.iter()
		.map(|x| &x.0)
		.chain(initial_nominators.iter())
		.for_each(|x| {
			if !endowed_accounts.contains(x) {
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
			let limit = (MaxNominations::get() as usize).min(initial_authorities.len());
			let count = rng.gen::<usize>() % limit;
			let nominations = initial_authorities
				.as_slice()
				.choose_multiple(&mut rng, count)
				.into_iter()
				.map(|choice| choice.0.clone())
				.collect::<Vec<_>>();
			(x.clone(), x.clone(), STASH, StakerStatus::Nominator(nominations))
		}))
		.collect::<Vec<_>>();

	let num_endowed_accounts = endowed_accounts.len();

	const ENDOWMENT: Balance = 1_000_000 * DOLLARS;
	const STASH: Balance = ENDOWMENT / 2;

	GenesisConfig {
		system: SystemConfig { code: wasm_binary_unwrap().to_vec() },
		balances: BalancesConfig {
			balances: endowed_accounts.iter().cloned().map(|x| {
				let balance = if x == root_key {
					495_000_000 * DOLLARS
				} else {
					ENDOWMENT
				};
				(x, balance)
			}).collect(),
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
			minimum_validator_count: 1 as u32,
			invulnerables: initial_authorities.iter().map(|x| x.0.clone()).collect(),
			slash_reward_fraction: Perbill::from_percent(10),
			stakers,
			..Default::default()
		},
		democracy: DemocracyConfig::default(),
		elections: ElectionsConfig {
			members: endowed_accounts
				.iter()
				.take((num_endowed_accounts + 1) / 2)
				.cloned()
				.map(|member| (member, STASH))
				.collect(),
		},
		council: CouncilConfig::default(),
		technical_committee: TechnicalCommitteeConfig {
			members: endowed_accounts
				.iter()
				.take((num_endowed_accounts + 1) / 2)
				.cloned()
				.collect(),
			phantom: Default::default(),
		},
		sudo: SudoConfig { key: Some(root_key) },
		babe: BabeConfig {
			authorities: vec![],
			epoch_config: Some(kitchensink_runtime::BABE_GENESIS_EPOCH_CONFIG),
		},
		im_online: ImOnlineConfig { keys: vec![] },
		authority_discovery: AuthorityDiscoveryConfig { keys: vec![] },
		grandpa: GrandpaConfig { authorities: vec![] },
		technical_membership: Default::default(),
		treasury: Default::default(),
		society: SocietyConfig {
			members: endowed_accounts
				.iter()
				.take((num_endowed_accounts + 1) / 2)
				.cloned()
				.collect(),
			pot: 0,
			max_members: 999,
		},
		vesting: Default::default(),
		assets: pallet_assets::GenesisConfig {
			// This asset is used by the NIS pallet as counterpart currency.
			assets: vec![(9, get_account_id_from_seed::<sr25519::Public>("Alice"), true, 1)],
			..Default::default()
		},
		transaction_storage: Default::default(),
		transaction_payment: Default::default(),
		alliance: Default::default(),
		alliance_motion: Default::default(),
		nomination_pools: NominationPoolsConfig {
			min_create_bond: 10 * DOLLARS,
			min_join_bond: 1 * DOLLARS,
			..Default::default()
		},
	}
}

fn development_config_genesis() -> GenesisConfig {
	testnet_genesis(
		vec![authority_keys_from_seed("Alice")],
		vec![],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		None,
	)
}

/// Development config (single validator Alice)
pub fn development_config() -> ChainSpec {
	ChainSpec::from_genesis(
		"Development",
		"dev",
		ChainType::Development,
		development_config_genesis,
		vec![],
		None,
		None,
		None,
		None,
		Default::default(),
	)
}

fn local_testnet_genesis() -> GenesisConfig {
	testnet_genesis(
		vec![authority_keys_from_seed("Alice"), authority_keys_from_seed("Bob")],
		vec![],
		get_account_id_from_seed::<sr25519::Public>("Alice"),
		None,
	)
}

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
		None,
		Default::default(),
	)
}

#[cfg(test)]
pub(crate) mod tests {
	use super::*;
	use crate::service::{new_full_base, NewFullBase};
	use sc_service_test;
	use sp_runtime::BuildStorage;

	fn local_testnet_genesis_instant_single() -> GenesisConfig {
		testnet_genesis(
			vec![authority_keys_from_seed("Alice")],
			vec![],
			get_account_id_from_seed::<sr25519::Public>("Alice"),
			None,
		)
	}

	/// Local testnet config (single validator - Alice)
	pub fn integration_test_config_with_single_authority() -> ChainSpec {
		ChainSpec::from_genesis(
			"Integration Test",
			"test",
			ChainType::Development,
			local_testnet_genesis_instant_single,
			vec![],
			None,
			None,
			None,
			None,
			Default::default(),
		)
	}

	/// Local testnet config (multivalidator Alice + Bob)
	pub fn integration_test_config_with_two_authorities() -> ChainSpec {
		ChainSpec::from_genesis(
			"Integration Test",
			"test",
			ChainType::Development,
			local_testnet_genesis,
			vec![],
			None,
			None,
			None,
			None,
			Default::default(),
		)
	}

	#[test]
	#[ignore]
	fn test_connectivity() {
		sp_tracing::try_init_simple();

		sc_service_test::connectivity(integration_test_config_with_two_authorities(), |config| {
			let NewFullBase { task_manager, client, network, transaction_pool, .. } =
				new_full_base(config, false, |_, _| ())?;
			Ok(sc_service_test::TestNetComponents::new(
				task_manager,
				client,
				network,
				transaction_pool,
			))
		});
	}

	#[test]
	fn test_create_development_chain_spec() {
		development_config().build_storage().unwrap();
	}

	#[test]
	fn test_create_local_testnet_chain_spec() {
		local_testnet_config().build_storage().unwrap();
	}

	#[test]
	fn test_staging_test_net_chain_spec() {
		staging_testnet_config().build_storage().unwrap();
	}
}
