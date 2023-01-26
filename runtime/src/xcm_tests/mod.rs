pub mod para_x;
pub mod relay;
pub mod tests;


use super::*;
use crate as sora_para;

use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};
use sp_io::TestExternalities;
use sp_runtime::AccountId32;
use xcm_executor::traits::WeightTrader;
use xcm_executor::Assets;

use xcm_simulator::{decl_test_network, decl_test_parachain, decl_test_relay_chain};



pub const ALICE: AccountId32 = AccountId32::new([0u8; 32]);
pub const BOB: AccountId32 = AccountId32::new([1u8; 32]);

decl_test_relay_chain! {
	pub struct Relay {
		Runtime = relay::Runtime,
		XcmConfig = relay::XcmConfig,
		new_ext = relay_ext(),
	}
}

pub fn relay_ext() -> sp_io::TestExternalities {
	use relay::{Runtime, System};

	let mut t = frame_system::GenesisConfig::default()
		.build_storage::<Runtime>()
		.unwrap();

	pallet_balances::GenesisConfig::<Runtime> {
		balances: vec![(ALICE, 1_000)],
	}
	.assimilate_storage(&mut t)
	.unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1));
	ext
}

decl_test_network! {
	pub struct TestNet {
		relay_chain = Relay,
		parachains = vec![],
	}
}