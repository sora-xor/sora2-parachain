// This file is part of the SORA network and Polkaswap app.

// Copyright (c) 2020, 2021, Polka Biome Ltd. All rights reserved.
// SPDX-License-Identifier: BSD-4-Clause

// Redistribution and use in source and binary forms, with or without modification,
// are permitted provided that the following conditions are met:

// Redistributions of source code must retain the above copyright notice, this list
// of conditions and the following disclaimer.
// Redistributions in binary form must reproduce the above copyright notice, this
// list of conditions and the following disclaimer in the documentation and/or other
// materials provided with the distribution.
//
// All advertising materials mentioning features or use of this software must display
// the following acknowledgement: This product includes software developed by Polka Biome
// Ltd., SORA, and Polkaswap.
//
// Neither the name of the Polka Biome Ltd. nor the names of its contributors may be used
// to endorse or promote products derived from this software without specific prior written permission.

// THIS SOFTWARE IS PROVIDED BY Polka Biome Ltd. AS IS AND ANY EXPRESS OR IMPLIED WARRANTIES,
// INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL Polka Biome Ltd. BE LIABLE FOR ANY
// DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING,
// BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS;
// OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT,
// STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE
// USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

pub mod para_x;
pub mod relay;
pub mod tests;

use cumulus_primitives_core::{ChannelStatus, GetChannelInfo, ParaId};
use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};
use sp_io::TestExternalities;
use sp_runtime::traits::{Convert, Zero};
use sp_runtime::AccountId32;
use xcm::{latest::Weight, prelude::*};
use xcm_executor::traits::WeightTrader;
use xcm_executor::Assets;
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

fn para_x_general_key() -> [u8; 32] {
	hex_literal::hex!("14fd1e1728cd833d21da6f3e36c50884062e35edfc24aec7a70c18a60451255b")
}

pub struct AllTokensAreCreatedEqualToWeight(MultiLocation);
impl WeightTrader for AllTokensAreCreatedEqualToWeight {
	fn new() -> Self {
		Self(MultiLocation::parent())
	}

	fn buy_weight(&mut self, weight: Weight, payment: Assets) -> Result<Assets, XcmError> {
		let asset_id = payment.fungible.iter().next().expect("Payment must be something; qed").0;
		let required = MultiAsset { id: asset_id.clone(), fun: Fungible(weight.ref_time() as u128) };

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
			Some((self.0.clone(), weight.ref_time() as u128).into())
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
		balances: vec![
			(ALICE, CurrencyId::R, 1_000_000_000_000_000_000),
			(ALICE, CurrencyId::X, 1_000_000_000_000_000_000),
		],
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
				Some((Parent, Parachain(1), GeneralKey{length: 32, data: para_x_general_key()}).into())
			},
		}
	}
}
impl Convert<MultiLocation, Option<CurrencyId>> for CurrencyIdConvert {
	fn convert(l: MultiLocation) -> Option<CurrencyId> {
		let x = para_x_general_key();
		if l == MultiLocation::parent() {
			return Some(CurrencyId::R);
		}
		match l {
			MultiLocation { parents, interior } if parents == 1 => match interior {
				X2(Parachain(1), GeneralKey {length: 32, data: k}) if k == x => Some(CurrencyId::X),
				_ => None,
			},
			MultiLocation { parents, interior } if parents == 0 => match interior {
				X1(GeneralKey {length: 32, data: k}) if k == x => Some(CurrencyId::X),
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
