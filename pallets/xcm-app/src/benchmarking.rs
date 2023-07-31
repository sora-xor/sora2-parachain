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
use crate::Pallet as XCMApp;
use frame_benchmarking::{benchmarks, impl_benchmark_test_suite};
use frame_system::RawOrigin;
use xcm::{
    opaque::latest::{
        Junction::{GeneralKey},
    },
    v3::MultiLocation,
};

fn alice<T: Config>() -> T::AccountId {
    let bytes = [66; 32];
    T::AccountId::decode(&mut &bytes[..]).expect("Failed to decode account ID")
}

benchmarks! {
    register_asset {
        let asset_id = [1; 32].into();
        let multilocation = test_multilocation();
    }: _(RawOrigin::Root, asset_id, multilocation.clone().into(), bridge_types::types::AssetKind::Thischain, 1000)
    verify {
        assert_eq!(
            XCMApp::<T>::get_multilocation_from_asset_id(asset_id)
                .expect("register_asset: multilocation is None"),
            multilocation.clone()
        );
        assert_eq!(
            XCMApp::<T>::get_asset_id_from_multilocation(multilocation)
                .expect("register_asset: asset id is None"),
            asset_id
        );
    }

    transfer {
        let asset_id = [1; 32].into();
        let multilocation = test_multilocation();
        let amount = 500;
    }: _(RawOrigin::Root, asset_id, alice::<T>(), multilocation.clone().into(), amount)
    verify {
        assert_event::<T>(Event::<T>::AssetTransferred(alice::<T>(), multilocation, asset_id, amount).into());
    }

    try_claim_bridge_asset {
        // trap_asset:
        let message_id = [0; 32].into();
        let asset_id = [1; 32].into();
        let amount = 500;
        XCMApp::<T>::trap_asset(Some(message_id), asset_id, alice::<T>(), amount, true);
    }: _(RawOrigin::Root, 1)
    verify {
        assert!(XCMApp::<T>::bridge_asset_trap(1).is_none());
    }
}

impl_benchmark_test_suite!(XCMApp, crate::mock::new_test_ext(), crate::mock::Test,);

fn test_multilocation() -> MultiLocation {
    let general_key = GeneralKey{length: 32, data: [15; 32]};
    // take the biggest multilocation
    MultiLocation {
        parents: 1,
        interior: xcm::v3::Junctions::X8(general_key,  general_key, general_key, general_key, general_key, general_key, general_key, general_key),
    }
}

fn assert_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
    let events = frame_system::Pallet::<T>::events();
    let system_event: <T as frame_system::Config>::RuntimeEvent = generic_event.into();
    // compare to the last event record
    assert!(events.into_iter().any(|x| {
        let frame_system::EventRecord { event, .. } = x;
        event == system_event
    }));
}
