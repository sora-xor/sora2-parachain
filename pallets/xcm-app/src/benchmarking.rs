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
use frame_benchmarking::{impl_benchmark_test_suite, v2::*, BenchmarkError};
use frame_support::{pallet_prelude::Weight, traits::EnsureOrigin};
use frame_system::RawOrigin;
use scale_info::prelude::vec;
use staging_xcm::{
    latest::prelude::{AssetId as XCMAssetId, *},
    opaque::latest::Junction::GeneralKey,
    v3::MultiLocation,
};

fn alice<T: Config>() -> T::AccountId {
    let bytes = [66; 32];
    T::AccountId::decode(&mut &bytes[..]).expect("Failed to decode account ID")
}

#[benchmarks]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn register_asset() -> Result<(), BenchmarkError> {
        let asset_id = [1; 32].into();
        let multilocation = test_multilocation();

        #[extrinsic_call]
        _(
            T::CallOrigin::try_successful_origin().unwrap() as T::RuntimeOrigin,
            asset_id,
            multilocation.into(),
            bridge_types::types::AssetKind::Thischain,
            1000,
        );

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

        Ok(())
    }

    #[benchmark]
    fn transfer() -> Result<(), BenchmarkError> {
        let asset_id = [1; 32].into();
        let multilocation = test_multilocation();
        let amount = 500;
        XCMApp::<T>::register_asset(
            T::CallOrigin::try_successful_origin().unwrap(),
            asset_id,
            multilocation.into(),
            bridge_types::types::AssetKind::Thischain,
            1000,
        )
        .expect("transfer: Failed register asset");

        #[extrinsic_call]
        _(
            T::CallOrigin::try_successful_origin().unwrap() as T::RuntimeOrigin,
            asset_id,
            alice::<T>(),
            multilocation.into(),
            amount,
        );

        assert_event::<T>(
            Event::<T>::AssetTransferred(alice::<T>(), multilocation, asset_id, amount).into(),
        );

        Ok(())
    }

    #[benchmark]
    fn try_claim_bridge_asset() -> Result<(), BenchmarkError> {
        let message_id = [0; 32].into();
        let asset_id = [1; 32].into();
        let amount = 500;
        // trap_asset:
        XCMApp::<T>::trap_asset(Some(message_id), asset_id, alice::<T>(), amount, true);

        #[extrinsic_call]
        _(RawOrigin::Root, 1);

        assert!(XCMApp::<T>::bridge_asset_trap(1).is_none());

        Ok(())
    }

    #[benchmark]
    fn set_asset_minimum_amount() -> Result<(), BenchmarkError> {
        let asset_id = [1; 32].into();
        let amount = 500;
        let multilocation = test_multilocation();
        XCMApp::<T>::register_asset(
            T::CallOrigin::try_successful_origin().unwrap(),
            asset_id,
            multilocation.into(),
            bridge_types::types::AssetKind::Thischain,
            1000,
        )
        .expect("set_asset_minimum_amount");

        #[extrinsic_call]
        _(T::CallOrigin::try_successful_origin().unwrap() as T::RuntimeOrigin, asset_id, amount);

        assert_eq!(
            XCMApp::<T>::asset_minimum_amount(multilocation)
                .expect("set_asset_minimum_amount: no min amount"),
            amount
        );

        Ok(())
    }

    #[benchmark]
    fn sudo_send_xcm() -> Result<(), BenchmarkError> {
        let asset = MultiAsset {
            id: XCMAssetId::Concrete(staging_xcm::v3::MultiLocation { parents: 1, interior: Here }),
            fun: staging_xcm::prelude::Fungible(100000000000000),
        };
        let msg = Xcm(vec![
            WithdrawAsset(asset.clone().into()),
            BuyExecution { fees: asset, weight_limit: WeightLimit::Unlimited },
            Transact {
                origin_kind: OriginKind::Native,
                require_weight_at_most: Weight::from_parts(4000000000, 10000),
                call: vec![0; 5000].into(),
            },
            RefundSurplus,
            DepositAsset {
                assets: MultiAssetFilter::Wild(staging_xcm::v3::WildMultiAsset::All),
                beneficiary: staging_xcm::v3::MultiLocation { parents: 1, interior: Here },
            },
        ]);
        let versioned_dest: bridge_types::substrate::VersionedMultiLocation =
            MultiLocation::parent().into();
        let versioned_msg = staging_xcm::VersionedXcm::from(msg);

        #[extrinsic_call]
        _(RawOrigin::Root, Box::new(versioned_dest), Box::new(versioned_msg));

        Ok(())
    }
}

impl_benchmark_test_suite!(XCMApp, crate::mock::new_test_ext(), crate::mock::Test,);

fn test_multilocation() -> MultiLocation {
    let general_key = GeneralKey { length: 32, data: [15; 32] };
    // take the biggest multilocation
    MultiLocation {
        parents: 1,
        interior: staging_xcm::v3::Junctions::X8(
            general_key,
            general_key,
            general_key,
            general_key,
            general_key,
            general_key,
            general_key,
            general_key,
        ),
    }
}

fn assert_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
    let events = frame_system::Pallet::<T>::events();
    let system_event: <T as frame_system::Config>::RuntimeEvent = generic_event.into();
    assert!(events.into_iter().any(|x| {
        let frame_system::EventRecord { event, .. } = x;
        event == system_event
    }));
}
