pub mod para_x;
pub mod relay;
pub mod tests;

// use super::*;
// use crate as sora_para;

use frame_support::{
	pallet_prelude::*,
	traits::{Contains, Get},
};
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};
use sp_io::TestExternalities;
use sp_runtime::traits::{Convert, Zero};
use sp_runtime::AccountId32;
use xcm::{latest::Weight, prelude::*};
use xcm_executor::traits::WeightTrader;
use xcm_executor::traits::{InvertLocation, WeightBounds};
use xcm_executor::Assets;
use cumulus_primitives_core::{ChannelStatus, GetChannelInfo, ParaId};
use xcm_simulator::{decl_test_network, decl_test_parachain, decl_test_relay_chain};

pub const ALICE: AccountId32 = AccountId32::new([10u8; 32]);
pub const BOB: AccountId32 = AccountId32::new([11u8; 32]);

pub type RelayBalances = pallet_balances::Pallet<relay::Runtime>;
pub type SoraBalances = pallet_balances::Pallet<crate::Runtime>;
pub type ParaTokens = orml_tokens::Pallet<para_x::Runtime>;
pub type ParaXTokens = orml_xtokens::Pallet<para_x::Runtime>;

#[derive(
	Encode,
	Decode,
	Eq,
	PartialEq,
	Copy,
	Clone,
	RuntimeDebug,
	PartialOrd,
	Ord,
	codec::MaxEncodedLen,
	TypeInfo,
)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum CurrencyId {
	/// Relay chain token.
	R,
	/// Parachain X token.
	X,
}

pub struct ChannelInfo;
impl GetChannelInfo for ChannelInfo {
	fn get_channel_status(_id: ParaId) -> ChannelStatus {
		ChannelStatus::Ready(10, 10)
	}
	fn get_channel_max(_id: ParaId) -> Option<usize> {
		Some(usize::max_value())
	}
}

// Declare network and chains:

decl_test_network! {
	pub struct TestNet {
		relay_chain = Relay,
		parachains = vec![
			(1, ParaX),
			(2, SoraParachain),
		],
	}
}

decl_test_relay_chain! {
	pub struct Relay {
		Runtime = relay::Runtime,
		XcmConfig = relay::XcmConfig,
		new_ext = relay_ext(),
	}
}

decl_test_parachain! {
	pub struct ParaX {
		Runtime = para_x::Runtime,
		XcmpMessageHandler = para_x::XcmpQueue,
		DmpMessageHandler = para_x::DmpQueue,
		new_ext = para_ext(1),
	}
}

decl_test_parachain! {
	pub struct SoraParachain {
		Runtime = crate::Runtime,
		XcmpMessageHandler = crate::XcmpQueue,
		DmpMessageHandler = crate::DmpQueue,
		new_ext = para_ext(2),
	}
}

// Configure Mock Relay Chain

pub fn relay_ext() -> sp_io::TestExternalities {
	use relay::{Runtime, System};

	let mut t = frame_system::GenesisConfig::default().build_storage::<Runtime>().unwrap();

	pallet_balances::GenesisConfig::<Runtime> { balances: vec![(ALICE, 1_000)] }
		.assimilate_storage(&mut t)
		.unwrap();

	let mut ext = sp_io::TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1));
	ext
}

// Configure Mock dummy Parachain:

pub type Balance = u128;
pub type Amount = i128;

pub struct AllTokensAreCreatedEqualToWeight(MultiLocation);
impl WeightTrader for AllTokensAreCreatedEqualToWeight {
	fn new() -> Self {
		Self(MultiLocation::parent())
	}

	fn buy_weight(&mut self, weight: Weight, payment: Assets) -> Result<Assets, XcmError> {
		let asset_id = payment.fungible.iter().next().expect("Payment must be something; qed").0;
		let required = MultiAsset { id: asset_id.clone(), fun: Fungible(weight as u128) };

		if let MultiAsset { fun: _, id: Concrete(ref id) } = &required {
			self.0 = id.clone();
		}

		let unused = payment.checked_sub(required).map_err(|_| XcmError::TooExpensive)?;
		Ok(unused)
	}

	fn refund_weight(&mut self, weight: Weight) -> Option<MultiAsset> {
		if weight.is_zero() {
			None
		} else {
			Some((self.0.clone(), weight as u128).into())
		}
	}
}

pub fn para_ext(para_id: u32) -> TestExternalities {
	use para_x::{Runtime, System};

	let mut t = frame_system::GenesisConfig::default().build_storage::<Runtime>().unwrap();

	let parachain_info_config = parachain_info::GenesisConfig { parachain_id: para_id.into() };
	<parachain_info::GenesisConfig as frame_support::traits::GenesisBuild<Runtime, _>>::assimilate_storage(&parachain_info_config, &mut t)
		.unwrap();

	orml_tokens::GenesisConfig::<Runtime> {
		balances: vec![(ALICE, CurrencyId::R, 1_000_000_000_000_000_000), (ALICE, CurrencyId::X, 1_000_000_000_000_000_000)],
	}
	.assimilate_storage(&mut t)
	.unwrap();

	let mut ext = TestExternalities::new(t);
	ext.execute_with(|| System::set_block_number(1));
	ext
}

pub struct CurrencyIdConvert;
impl Convert<CurrencyId, Option<MultiLocation>> for CurrencyIdConvert {
	fn convert(id: CurrencyId) -> Option<MultiLocation> {
		match id {
			CurrencyId::R => Some(Parent.into()),
			CurrencyId::X => {
				Some((Parent, Parachain(1), GeneralKey(b"X".to_vec().try_into().unwrap())).into())
			},
		}
	}
}
impl Convert<MultiLocation, Option<CurrencyId>> for CurrencyIdConvert {
	fn convert(l: MultiLocation) -> Option<CurrencyId> {
		let x: Vec<u8> = "X".into();
		if l == MultiLocation::parent() {
			return Some(CurrencyId::R);
		}
		match l {
			MultiLocation { parents, interior } if parents == 1 => match interior {
				X2(Parachain(1), GeneralKey(k)) if k == x => Some(CurrencyId::X),
				_ => None,
			},
			MultiLocation { parents, interior } if parents == 0 => match interior {
				X1(GeneralKey(k)) if k == x => Some(CurrencyId::X),
				_ => None,
			},
			_ => None,
		}
	}
}
impl Convert<MultiAsset, Option<CurrencyId>> for CurrencyIdConvert {
	fn convert(a: MultiAsset) -> Option<CurrencyId> {
		if let MultiAsset { fun: Fungible(_), id: Concrete(id) } = a {
			Self::convert(id)
		} else {
			Option::None
		}
	}
}
