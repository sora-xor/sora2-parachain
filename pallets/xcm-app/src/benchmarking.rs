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

use super::*;
#[allow(unused)]
use crate::Pallet as XCMApp;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite};
use frame_support::WeakBoundedVec;
use frame_system::RawOrigin;
use xcm::opaque::latest::Junction::{GeneralKey, Parachain};
use xcm::opaque::latest::Junctions::X2;
use xcm::v1::MultiLocation;

benchmarks! {
	register_mapping {
		let asset_id = [
			1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
			1, 1, 1,
		];
		let multilocation = MultiLocation {
			parents: 1,
			interior: X2(Parachain(666), GeneralKey(test_general_key())),
		};
	}: _(RawOrigin::Root, asset_id, multilocation.clone())
	verify {
		assert_eq!(
			XCMApp::<T>::get_multilocation_from_asset_id(asset_id)
				.expect("register_mapping: multilocation is None"),
			multilocation.clone()
		);
		assert_eq!(
			XCMApp::<T>::get_asset_id_from_multilocation(multilocation)
				.expect("register_mapping: asset id is None"),
			asset_id
		);
	}

	change_asset_mapping {
		let asset_id = [
			1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
			1, 1, 1,
		];
		let multilocation = MultiLocation::parent();
		let new_multilocation = MultiLocation {
			parents: 1,
			interior: X2(Parachain(666), GeneralKey(test_general_key())),
		};
		XCMApp::<T>::register_mapping(RawOrigin::Root.into(), asset_id, multilocation.clone())
			.expect("change_asset_mapping: failed to create a map");
	}: _(RawOrigin::Root, asset_id, new_multilocation.clone())
	verify {
		assert_eq!(
			XCMApp::<T>::get_multilocation_from_asset_id(asset_id)
				.expect("change_asset_mapping: new_multilocation is None"),
			new_multilocation.clone()
		);
		assert_eq!(
			XCMApp::<T>::get_asset_id_from_multilocation(new_multilocation).expect(
				"change_asset_mapping: asset_id is None"
			),
			asset_id
		);
		assert_eq!(XCMApp::<T>::get_asset_id_from_multilocation(multilocation), None);
	}

	change_multilocation_mapping {
		let asset_id = [
			1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
			1, 1, 1,
		];
		let new_asset_id = [
			2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2,
			2, 2, 2,
		];
		let multilocation = MultiLocation {
			parents: 1,
			interior: X2(Parachain(666), GeneralKey(test_general_key())),
		};
		XCMApp::<T>::register_mapping(RawOrigin::Root.into(), asset_id, multilocation.clone())
		.expect("change_multilocation_mapping: failed to create a map");
	}: _(RawOrigin::Root, multilocation.clone(), new_asset_id)
	verify {
		assert_eq!(
			XCMApp::<T>::get_multilocation_from_asset_id(new_asset_id)
				.expect("change_multilocation_mapping: new_multilocation is None"),
				multilocation.clone()
		);
		assert_eq!(
			XCMApp::<T>::get_asset_id_from_multilocation(multilocation).expect(
				"change_multilocation_mapping: asset_id is None"
			),
			new_asset_id
		);
		assert_eq!(XCMApp::<T>::get_multilocation_from_asset_id(asset_id), None);
	}

	delete_mapping {
		let asset_id = [
			1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
			1, 1, 1,
		];
		let multilocation = MultiLocation {
			parents: 1,
			interior: X2(Parachain(666), GeneralKey(test_general_key())),
		};
		XCMApp::<T>::register_mapping(RawOrigin::Root.into(), asset_id, multilocation.clone())
		.expect("change_multilocation_mapping: failed to create a map");
	}: _(RawOrigin::Root, asset_id)
	verify {
		assert_eq!(XCMApp::<T>::get_multilocation_from_asset_id(asset_id), None);
		assert_eq!(XCMApp::<T>::get_asset_id_from_multilocation(multilocation), None);
	}
}

impl_benchmark_test_suite!(XCMApp, crate::mock::new_test_ext(), crate::mock::Test,);

pub fn test_general_key() -> WeakBoundedVec<u8, frame_support::traits::ConstU32<32>> {
	WeakBoundedVec::try_from(b"TEST_ASSET".to_vec()).unwrap()
}
